use crate::errors::{AppError, AppErrorCode, AppResult};
use crate::utils::ffmpeg::{convert_video_with_progress, ConvertOptions};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use filetime::{set_file_times, FileTime};
use open;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, State};
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoFile {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoConversionParams {
    pub ffmpeg_path: String,
    pub ffprobe_path: String,
    pub ffmpeg_use_installed: bool,
    pub ffprobe_use_installed: bool,
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

#[derive(Default)]
pub struct ConversionController {
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
}

fn resolve_ffmpeg(params: &VideoConversionParams) -> AppResult<String> {
    if params.ffmpeg_use_installed {
        Ok("ffmpeg".to_string())
    } else if !params.ffmpeg_path.trim().is_empty() {
        let p = Path::new(&params.ffmpeg_path);
        if p.exists() {
            Ok(p.to_string_lossy().to_string())
        } else {
            Err(AppError::new(
                AppErrorCode::FfmpegNotFound,
                format!("ffmpeg not found at {}", params.ffmpeg_path),
            ))
        }
    } else {
        Err(AppError::new(
            AppErrorCode::FfmpegNotFound,
            "ffmpeg path not provided and ffmpeg_use_installed = false",
        ))
    }
}

fn resolve_ffprobe(params: &VideoConversionParams) -> Option<String> {
    if params.ffprobe_use_installed {
        Some("ffprobe".into())
    } else if !params.ffprobe_path.trim().is_empty() {
        let p = Path::new(&params.ffprobe_path);
        if p.exists() {
            Some(p.to_string_lossy().to_string())
        } else {
            None
        }
    } else {
        None
    }
}

// rust
async fn extract_thumbnail_data_url(
    ffmpeg_bin: &str,
    input: &Path,
    cancel: &CancellationToken,
) -> AppResult<Option<String>> {
    if cancel.is_cancelled() {
        return Ok(None);
    }

    let mut cmd = Command::new(ffmpeg_bin);

    // Kill the child if the handle is dropped.
    cmd.kill_on_drop(true);

    // Windows: hide console window.
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.arg("-hide_banner")
        .arg("-loglevel")
        .arg("error")
        .arg("-nostdin")
        .arg("-y")
        .arg("-ss")
        .arg("1")
        .arg("-i")
        .arg(input)
        .arg("-frames:v")
        .arg("1")
        .arg("-vf")
        .arg("scale=320:-2")
        .arg("-q:v")
        .arg("5")
        .arg("-f")
        .arg("mjpeg")
        .arg("-");

    // Ensure no stdin; pipe stdout; silence stderr.
    cmd.stdin(std::process::Stdio::null());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::null());

    let mut child = cmd
        .spawn()
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    // Read stdout concurrently to avoid blocking the child.
    let Some(mut stdout) = child.stdout.take() else {
        return Ok(None);
    };
    let read_task = tokio::spawn(async move {
        let mut buf = Vec::new();
        let _ = stdout.read_to_end(&mut buf).await;
        buf
    });

    let cancel_for_wait = cancel.clone();
    tokio::select! {
        _ = cancel_for_wait.cancelled() => {
            let _ = child.kill().await;
            let _ = child.wait().await;   // reap process
            let _ = read_task.await;      // join reader task
            Ok(None)
        }
        status = child.wait() => {
            let status = status.map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;
            let out = read_task.await
                .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;
            if !status.success() || out.is_empty() {
                return Ok(None);
            }
            let b64 = general_purpose::STANDARD.encode(out);
            let data_url = format!("data:image/jpeg;base64,{}", b64);
            Ok(Some(data_url))
        }
    }
}

#[tauri::command]
pub async fn get_video_thumbnail(
    path: String,
    state: State<'_, ConversionController>,
) -> AppResult<Option<String>> {
    let cancel = state.new_token().await;

    // Use installed ffmpeg on PATH for simplicity.
    // If you need custom paths, pass them in as params like in `convert_videos`.
    let ffmpeg_bin = "ffmpeg";

    let p = PathBuf::from(&path);
    if !p.exists() || !p.is_file() {
        return Ok(None);
    }
    extract_thumbnail_data_url(ffmpeg_bin, &p, &cancel).await
}

// Note: this will be slower on big folders. Consider limiting concurrency if needed.
// #[tauri::command]
// pub async fn get_video_files_with_thumbnails(
//     folder_path: String,
//     state: State<'_, ConversionController>,
// ) -> AppResult<Vec<VideoFile>> {
//     let cancel = state.new_token().await;
//     let mut files = list_video_files(folder_path, cancel.clone()).await?;
//
//     for f in files.iter_mut() {
//         if cancel.is_cancelled() {
//             break;
//         }
//         let thumb = extract_thumbnail_data_url("ffmpeg", Path::new(&f.path), &cancel).await?;
//         f.thumbnail = thumb; // small data URL or `None` if failed/cancelled
//     }
//     Ok(files)
// }

async fn list_video_files(
    folder_path: String,
    cancel: CancellationToken,
) -> AppResult<Vec<VideoFile>> {
    let path = Path::new(&folder_path);
    if !path.exists() {
        return Err(AppError::code_only(AppErrorCode::FolderNotFound));
    }

    let supported_extensions = ["mp4", "mkv", "avi", "mov", "webm"];
    let mut video_files = Vec::new();

    let dir = fs::read_dir(path).map_err(AppError::from)?;
    for entry in dir {
        if cancel.is_cancelled() {
            return Err(AppError::code_only(AppErrorCode::Cancelled));
        }
        let entry = entry.map_err(AppError::from)?;
        let file_type = entry.file_type().map_err(AppError::from)?;
        if !file_type.is_file() {
            continue;
        }
        let file_path = entry.path();
        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        if let Some(ext) = extension {
            if supported_extensions.contains(&ext.as_str()) {
                let metadata = entry.metadata().map_err(AppError::from)?;
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

// rust
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
            std::ptr::null_mut(), // LPSECURITY_ATTRIBUTES
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            std::ptr::null_mut() as HANDLE, // hTemplateFile
        )
    };
    if h == INVALID_HANDLE_VALUE {
        return Err("CreateFileW failed".to_string());
    }

    let ft = system_time_to_filetime(t);
    let ok = unsafe { SetFileTime(h, &ft, std::ptr::null(), std::ptr::null()) };

    // Always close the handle
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
    let cancel = state.new_token().await;
    list_video_files(folder_path, cancel).await
}

#[tauri::command]
pub async fn convert_videos(
    app: AppHandle,
    params: VideoConversionParams,
    state: State<'_, ConversionController>,
) -> AppResult<String> {
    let cancel = state.new_token().await;

    let ffmpeg_bin = resolve_ffmpeg(&params)?;
    let ffprobe_bin = resolve_ffprobe(&params);

    let inputs: Vec<VideoFile> = if !params.files.is_empty() {
        params
            .files
            .iter()
            .map(|p| {
                let pb = PathBuf::from(p);
                let name = pb
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let size = fs::metadata(&pb).map(|m| m.len()).unwrap_or(0);
                VideoFile {
                    path: pb.to_string_lossy().to_string(),
                    name,
                    size,
                    thumbnail: None,
                }
            })
            .collect()
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

        let convert_res = convert_video_with_progress(
            ConvertOptions {
                ffmpeg_bin: &ffmpeg_bin,
                ffprobe_bin: ffprobe_bin.as_deref(),
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
                        .ok()
                        .and_then(|m| m.modified().ok())
                };
                if let Some(ts) = ts_sys {
                    let ft = FileTime::from_system_time(ts);
                    let _ = set_file_times(&output_path, ft, ft);
                    #[cfg(target_os = "windows")]
                    {
                        use super::video::set_creation_time_windows;
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
                // Map cancellation to a thrown error; all other per-file errors are only signaled via events.
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
    //now just open output folder for the user in a separate window
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
