// Free FPS - Video Frame Rate Converter
// Copyright (C) 2025 undelalune
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::errors::{AppError, AppErrorCode, AppResult};
use crate::utils::bundled_ffmpeg::{get_ffmpeg_path, get_ffprobe_path};
use crate::utils::ffmpeg::{convert_video_with_progress, ConvertOptions};
use crate::utils::rate_limiter::RateLimiter;
use chrono::{DateTime, Utc};
use filetime::{set_file_times, FileTime};
use open;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, State};
use tokio::fs;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::commands::thumbnail::{get_video_thumbnail_data_url};

// Security: Validate that a path is within a base folder to prevent path traversal
fn validate_safe_path(path: &str, base_folder: &str) -> AppResult<PathBuf> {
    let base = PathBuf::from(base_folder).canonicalize().map_err(|e| {
        AppError::new(
            AppErrorCode::InvalidInputPath,
            format!("Invalid base folder: {}", e),
        )
    })?;

    let target = PathBuf::from(path).canonicalize().map_err(|e| {
        AppError::new(
            AppErrorCode::InvalidInputPath,
            format!("Invalid file path: {}", e),
        )
    })?;

    if !target.starts_with(&base) {
        return Err(AppError::new(
            AppErrorCode::PathTraversalDetected,
            format!("Path '{}' is outside allowed directory", path),
        ));
    }

    Ok(target)
}

// Validate conversion parameters
fn validate_conversion_params(params: &VideoConversionParams) -> AppResult<()> {
    if params.target_fps <= 0.0 || params.target_fps > 1000.0 {
        return Err(AppError::new(
            AppErrorCode::InvalidFps,
            format!(
                "FPS must be between 0.1 and 1000, got {}",
                params.target_fps
            ),
        ));
    }

    if params.keep_audio {
        if params.audio_bitrate == 0 || params.audio_bitrate > 512 {
            return Err(AppError::new(
                AppErrorCode::AudioBitrateInvalid,
                format!(
                    "Audio bitrate must be between 1 and 512 kbps, got {}",
                    params.audio_bitrate
                ),
            ));
        }
    }

    if params.use_custom_video_quality && params.video_quality > 51 {
        return Err(AppError::new(
            AppErrorCode::VideoQualityOutOfRange,
            format!("CRF must be between 0 and 51, got {}", params.video_quality),
        ));
    }

    if params.cpu_limit == 0 || params.cpu_limit > 100 {
        return Err(AppError::new(
            AppErrorCode::Io,
            format!(
                "CPU limit must be between 1 and 100, got {}",
                params.cpu_limit
            ),
        ));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoFile {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoConversionParams {
    pub input_folder: String,
    pub output_folder: String,
    pub target_fps: f32,
    pub cpu_limit: u8,
    pub keep_audio: bool,
    pub audio_bitrate: u32,
    pub use_custom_video_quality: bool,
    pub video_quality: u8,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionStatus {
    Processing,
    Success,
    Error,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionProgress {
    pub current_file: String,
    pub current_file_index: usize,
    pub total_files: usize,
    pub percentage: f32,
    pub status: ConversionStatus,
}

pub struct ConversionController {
    scan_limiter: RateLimiter,
    conversion_limiter: RateLimiter,
    token: Mutex<Option<CancellationToken>>,
}

impl ConversionController {
    pub async fn new_token(&self) -> CancellationToken {
        let mut guard = self.token.lock().await;
        if let Some(old) = guard.take() {
            old.cancel();
        }
        let new = CancellationToken::new();
        *guard = Some(new.clone());
        new
    }

    pub async fn cancel(&self) {
        if let Some(tok) = self.token.lock().await.take() {
            tok.cancel();
        }
    }

    pub fn scan_limiter(&self) -> &RateLimiter {
        &self.scan_limiter
    }

    pub fn conversion_limiter(&self) -> &RateLimiter {
        &self.conversion_limiter
    }
}

impl Default for ConversionController {
    fn default() -> Self {
        Self {
            token: Mutex::new(None),
            scan_limiter: RateLimiter::new(1), // Only 1 scan at a time
            conversion_limiter: RateLimiter::new(1), // Only 1 conversion at a time
        }
    }
}

#[tauri::command]
pub async fn get_video_thumbnail(
    path: String,
    state: tauri::State<'_, ConversionController>,
) -> AppResult<Option<String>> {
    let cancel = state.new_token().await;
    get_video_thumbnail_data_url(&path, &cancel).await
}

async fn list_video_files(
    folder_path: String,
    cancel: CancellationToken,
) -> AppResult<Vec<VideoFile>> {
    use tokio::fs as async_fs;

    let path = Path::new(&folder_path);
    if !async_fs::try_exists(path).await.unwrap_or(false) {
        return Err(AppError::code_only(AppErrorCode::FolderNotFound));
    }

    // Canonicalize the base folder for security validation
    let base_canonical = async_fs::canonicalize(path)
        .await
        .map_err(|e| AppError::new(AppErrorCode::InvalidInputPath, e.to_string()))?;

    let supported_extensions = ["mp4", "mkv", "avi", "mov", "webm"];
    let mut video_files = Vec::new();

    let mut dir = async_fs::read_dir(path).await.map_err(AppError::from)?;
    while let Some(entry) = dir.next_entry().await.map_err(AppError::from)? {
        if cancel.is_cancelled() {
            return Err(AppError::code_only(AppErrorCode::Cancelled));
        }

        let file_type = entry.file_type().await.map_err(AppError::from)?;
        if !file_type.is_file() {
            continue;
        }

        let file_path = entry.path();

        // Security: Ensure file is within the base folder
        if let Ok(canonical) = async_fs::canonicalize(&file_path).await {
            if !canonical.starts_with(&base_canonical) {
                continue; // Skip files outside base folder
            }
        } else {
            continue; // Skip files that can't be canonicalized
        }

        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        if let Some(ext) = extension {
            if supported_extensions.contains(&ext.as_str()) {
                let metadata = entry.metadata().await.map_err(AppError::from)?;
                video_files.push(VideoFile {
                    path: file_path.to_string_lossy().to_string(),
                    name: file_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    size: metadata.len(),
                    thumbnail: None,
                });
            }
        }
    }

    Ok(video_files)
}

fn derive_output_folder(params: &VideoConversionParams) -> PathBuf {
    if !params.output_folder.trim().is_empty() {
        return PathBuf::from(&params.output_folder);
    }
    let mut p = PathBuf::from(&params.input_folder);
    p.push(format!("converted_videos_{}fps", params.target_fps));
    p
}

fn parse_creation_time(ct: &str) -> Option<std::time::SystemTime> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(ct) {
        return Some(std::time::SystemTime::from(dt.with_timezone(&Utc)));
    }
    None
}

#[cfg(target_os = "windows")]
fn set_creation_time_windows(path: &Path, t: std::time::SystemTime) -> Result<(), String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, SetFileTime, FILE_ATTRIBUTE_NORMAL, FILE_WRITE_ATTRIBUTES, OPEN_EXISTING,
    };

    fn to_utf16(s: &OsStr) -> Vec<u16> {
        s.encode_wide().chain(std::iter::once(0)).collect()
    }

    fn system_time_to_filetime(
        st: std::time::SystemTime,
    ) -> windows_sys::Win32::Foundation::FILETIME {
        use windows_sys::Win32::Foundation::FILETIME;
        const EPOCH_DIFF_SECS: i64 = 11644473600;
        let (secs, nanos, neg) = match st.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => (d.as_secs() as i128, d.subsec_nanos() as i128, false),
            Err(e) => {
                let d = e.duration();
                (d.as_secs() as i128, d.subsec_nanos() as i128, true)
            }
        };
        let unix_100ns = (secs * 10_000_000) + (nanos / 100);
        let total_100ns = if neg {
            (EPOCH_DIFF_SECS as i128) * 10_000_000 - unix_100ns
        } else {
            (EPOCH_DIFF_SECS as i128) * 10_000_000 + unix_100ns
        };
        FILETIME {
            dwLowDateTime: (total_100ns as u64 & 0xFFFF_FFFF) as u32,
            dwHighDateTime: ((total_100ns as u64 >> 32) & 0xFFFF_FFFF) as u32,
        }
    }

    let h: HANDLE = unsafe {
        CreateFileW(
            to_utf16(path.as_os_str()).as_ptr(),
            FILE_WRITE_ATTRIBUTES,
            0,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            std::ptr::null_mut() as HANDLE,
        )
    };
    if h == INVALID_HANDLE_VALUE {
        return Err("CreateFileW failed".to_string());
    }

    let ft = system_time_to_filetime(t);
    let ok = unsafe { SetFileTime(h, &ft, std::ptr::null(), std::ptr::null()) };

    unsafe { CloseHandle(h) };

    if ok == 0 {
        return Err("SetFileTime failed".to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn get_video_files(
    folder_path: String,
    state: State<'_, ConversionController>,
) -> AppResult<Vec<VideoFile>> {
    // Rate limiting: Only one scan at a time
    let _permit = state.scan_limiter().acquire().await;

    let cancel = state.new_token().await;
    list_video_files(folder_path, cancel).await
}

#[tauri::command]
pub async fn convert_videos(
    app: AppHandle,
    params: VideoConversionParams,
    state: State<'_, ConversionController>,
) -> AppResult<String> {
    // Rate limiting: Only one conversion at a time
    let _permit = state.conversion_limiter().acquire().await;
    let cancel = state.new_token().await;

    // Validate parameters first
    validate_conversion_params(&params)?;

    // Get bundled FFmpeg paths
    let ffmpeg_bin = get_ffmpeg_path(&app)?;
    let ffprobe_bin = get_ffprobe_path(&app).ok();

    let inputs: Vec<VideoFile> = if !params.files.is_empty() {
        let mut video_files = Vec::new();
        for p in &params.files {
            let pb = PathBuf::from(p);
            let name = pb
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let size = fs::metadata(&pb).await.map(|m| m.len()).unwrap_or(0);
            video_files.push(VideoFile {
                path: pb.to_string_lossy().to_string(),
                name,
                size,
                thumbnail: None,
            });
        }
        video_files
    } else {
        list_video_files(params.input_folder.clone(), cancel.clone()).await?
    };

    if inputs.is_empty() {
        return Err(AppError::code_only(AppErrorCode::NoVideoFiles));
    }

    let output_dir = derive_output_folder(&params);
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    let total_files = inputs.len();

    for (index, video_file) in inputs.iter().enumerate() {
        if cancel.is_cancelled() {
            let cancelled = ConversionProgress {
                current_file: String::new(),
                current_file_index: index,
                total_files,
                percentage: 0.0,
                status: ConversionStatus::Cancelled,
            };
            let _ = app.emit("conversion-progress", &cancelled);
            return Err(AppError::code_only(AppErrorCode::Cancelled));
        }

        // Security: Validate the file path is within input folder
        match validate_safe_path(&video_file.path, &params.input_folder) {
            Ok(p) => p,
            Err(e) => {
                let err_evt = ConversionProgress {
                    current_file: video_file.name.clone(),
                    current_file_index: index + 1,
                    total_files,
                    percentage: 0.0,
                    status: ConversionStatus::Error,
                };
                let _ = app.emit("conversion-progress", &err_evt);
                eprintln!("Path validation failed for {}: {:?}", video_file.path, e);
                continue;
            }
        };
        let input_path = Path::new(&video_file.path);

        if !input_path.is_file() {
            let err_evt = ConversionProgress {
                current_file: video_file.name.clone(),
                current_file_index: index + 1,
                total_files,
                percentage: 0.0,
                status: ConversionStatus::Error,
            };
            let _ = app.emit("conversion-progress", &err_evt);
            continue;
        }

        let progress = ConversionProgress {
            current_file: video_file.name.clone(),
            current_file_index: index + 1,
            total_files,
            percentage: 0.0,
            status: ConversionStatus::Processing,
        };
        app.emit("conversion-progress", &progress)
            .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

        let output_filename = format!(
            "{}_{}fps.{}",
            input_path.file_stem().unwrap().to_string_lossy(),
            params.target_fps,
            input_path.extension().unwrap_or_default().to_string_lossy()
        );
        let output_path = output_dir.join(output_filename);

        let app_clone = app.clone();
        let video_name = video_file.name.clone();
        let current_index = index;
        let total = total_files;
        let cancel_clone = cancel.clone();

        let ffmpeg_str = ffmpeg_bin.to_string_lossy().to_string();
        let ffprobe_str = ffprobe_bin.as_ref().map(|p| p.to_string_lossy().to_string());

        let convert_res = convert_video_with_progress(
            ConvertOptions {
                ffmpeg_bin: &ffmpeg_str,
                ffprobe_bin: ffprobe_str.as_deref(),
                input: &video_file.path,
                output: &output_path.to_string_lossy(),
                target_fps: params.target_fps,
                keep_audio: params.keep_audio,
                audio_bitrate: params.audio_bitrate,
                use_custom_video_quality: params.use_custom_video_quality,
                video_quality: params.video_quality,
                cpu_limit: Some(params.cpu_limit),
            },
            move |p| {
                if cancel_clone.is_cancelled() {
                    return;
                }
                let p01 = (p / 100.0).clamp(0.0, 1.0);
                let file_pct = p01 * 100.0;

                let detailed = ConversionProgress {
                    current_file: video_name.clone(),
                    current_file_index: current_index + 1,
                    total_files: total,
                    percentage: file_pct,
                    status: ConversionStatus::Processing,
                };
                let _ = app_clone.emit("conversion-progress", &detailed);
            },
            cancel.clone(),
        )
            .await;

        match convert_res {
            Ok(creation_time_str) => {
                let ts_sys = if let Some(ct) = creation_time_str.as_deref() {
                    parse_creation_time(ct)
                } else {
                    fs::metadata(&video_file.path)
                        .await
                        .ok()
                        .and_then(|m| m.modified().ok())
                };
                if let Some(ts) = ts_sys {
                    let ft = FileTime::from_system_time(ts);
                    let _ = set_file_times(&output_path, ft, ft);
                    #[cfg(target_os = "windows")]
                    {
                        let _ = set_creation_time_windows(&output_path, ts);
                    }
                }

                let done = ConversionProgress {
                    current_file: video_file.name.clone(),
                    current_file_index: index + 1,
                    total_files,
                    percentage: 100.0,
                    status: ConversionStatus::Success,
                };
                let _ = app.emit("conversion-progress", &done);
            }
            Err(e) => {
                if e == "Cancelled" {
                    let _ = app.emit(
                        "conversion-progress",
                        &ConversionProgress {
                            current_file: String::new(),
                            current_file_index: index,
                            total_files,
                            percentage: 0.0,
                            status: ConversionStatus::Cancelled,
                        },
                    );
                    return Err(AppError::code_only(AppErrorCode::Cancelled));
                }

                let err_evt = ConversionProgress {
                    current_file: video_file.name.clone(),
                    current_file_index: index + 1,
                    total_files,
                    percentage: 0.0,
                    status: ConversionStatus::Error,
                };
                let _ = app.emit("conversion-progress", &err_evt);
                continue;
            }
        }
    }

    if let Err(e) = open::that(&output_dir) {
        eprintln!("Failed to open file manager: {}", e);
    }

    Ok(format!("Successfully converted {} videos", total_files))
}

#[tauri::command]
pub async fn cancel_conversion(
    app: AppHandle,
    state: State<'_, ConversionController>,
) -> AppResult<()> {
    state.cancel().await;
    app.emit(
        "conversion-progress",
        &ConversionProgress {
            current_file: String::new(),
            current_file_index: 0,
            total_files: 0,
            percentage: 0.0,
            status: ConversionStatus::Cancelled,
        },
    )
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;
    Ok(())
}