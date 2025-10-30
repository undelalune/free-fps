use crate::errors::{AppError, AppErrorCode, AppResult};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use tokio_util::sync::CancellationToken;

use crate::utils::proc::apply_no_window_tokio;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThumbnailParams {
    pub path: String,
    pub ffmpeg_path: String,
    pub ffprobe_path: String,
    pub ffmpeg_use_installed: bool,
    pub ffprobe_use_installed: bool,
}

fn resolve_ffmpeg_from_thumb(params: &ThumbnailParams) -> AppResult<String> {
    crate::commands::video::resolve_ffmpeg_common(params.ffmpeg_use_installed, &params.ffmpeg_path)
}

async fn extract_thumbnail_data_url(
    ffmpeg_bin: &str,
    input: &Path,
    cancel: &CancellationToken,
) -> AppResult<Option<String>> {
    if cancel.is_cancelled() {
        return Ok(None);
    }

    let mut cmd = Command::new(ffmpeg_bin);
    cmd.kill_on_drop(true);
    apply_no_window_tokio(&mut cmd);

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
        .arg("-")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null());

    let mut child = cmd
        .spawn()
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

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
            let _ = child.wait().await;
            let _ = read_task.await;
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
            Ok(Some(format!("data:image/jpeg;base64,{}", b64)))
        }
    }
}

// System thumbnail extraction (Windows)
#[cfg(target_os = "windows")]
async fn extract_thumbnail_system(
    input: &std::path::Path,
    size: u32,
    cancel: &tokio_util::sync::CancellationToken,
) -> crate::errors::AppResult<Option<String>> {
    use crate::errors::{AppError, AppErrorCode};
    use base64::{engine::general_purpose, Engine as _};
    use windows::core::HSTRING;
    use windows::Storage::FileProperties::{ThumbnailMode, ThumbnailOptions};
    use windows::Storage::StorageFile;
    use windows::Storage::Streams::{DataReader, InputStreamOptions};
    use windows::Win32::Foundation::RPC_E_CHANGED_MODE;
    use windows::Win32::System::WinRT::{RoInitialize, RO_INIT_MULTITHREADED};

    if cancel.is_cancelled() {
        return Ok(None);
    }

    unsafe {
        match RoInitialize(RO_INIT_MULTITHREADED) {
            Ok(_) => {}
            Err(e) if e.code() == RPC_E_CHANGED_MODE => {}
            Err(e) => {
                return Err(AppError::new(
                    AppErrorCode::Io,
                    format!("RoInitialize failed: {:?}", e),
                ));
            }
        }
    }

    let hpath = HSTRING::from(input.to_string_lossy().to_string());

    let file = StorageFile::GetFileFromPathAsync(&hpath)
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?
        .await
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    if cancel.is_cancelled() {
        return Ok(None);
    }

    let thumb = file
        .GetThumbnailAsync(
            ThumbnailMode::VideosView,
            size,
            ThumbnailOptions::UseCurrentScale,
        )
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?
        .await
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    let total = thumb
        .Size()
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;
    if total == 0 {
        return Ok(None);
    }

    let stream = thumb
        .GetInputStreamAt(0)
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    let reader = DataReader::CreateDataReader(&stream)
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    reader
        .SetInputStreamOptions(InputStreamOptions::ReadAhead)
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    let to_read = u32::try_from(total.min(10 * 1024 * 1024)).unwrap_or(0);
    if to_read == 0 {
        return Ok(None);
    }

    reader
        .LoadAsync(to_read)
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?
        .await
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    if cancel.is_cancelled() {
        return Ok(None);
    }

    let mut buf = vec![0u8; to_read as usize];
    reader
        .ReadBytes(&mut buf)
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    let content_type = thumb
        .ContentType()
        .map(|s| s.to_string())
        .unwrap_or_else(|_| "image/png".to_string());

    let b64 = general_purpose::STANDARD.encode(buf);
    Ok(Some(format!("data:{};base64,{}", content_type, b64)))
}

#[cfg(target_os = "macos")]
async fn extract_thumbnail_system(
    input: &std::path::Path,
    size: u32,
    cancel: &tokio_util::sync::CancellationToken,
) -> crate::errors::AppResult<Option<String>> {
    use crate::errors::{AppError, AppErrorCode};
    use base64::{engine::general_purpose, Engine as _};
    use std::fs;
    use std::path::PathBuf;
    use std::process::{Command as StdCommand, Stdio};
    use std::time::{Duration, Instant};
    use tokio::io::AsyncReadExt;

    if cancel.is_cancelled() {
        return Ok(None);
    }

    let tmp_dir = std::env::temp_dir().join("tauri_video_thumbs");
    let _ = fs::create_dir_all(&tmp_dir);

    let mut out_png: PathBuf = tmp_dir.join(
        input
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    );
    out_png.set_extension("png");

    let mut cmd = StdCommand::new("/usr/bin/qlmanage");
    cmd.arg("-t")
        .arg("-s")
        .arg(size.to_string())
        .arg("-o")
        .arg(&tmp_dir)
        .arg(input)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // Make a new process group so we can kill children too.
    #[cfg(unix)]
    unsafe {
        use std::os::unix::process::CommandExt;
        cmd.pre_exec(|| {
            libc::setpgid(0, 0);
            Ok(())
        });
    }

    let mut child = cmd
        .spawn()
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    let pid = child.id();
    let cancel2 = cancel.clone();

    // Hard wall‑clock timeout: 1s
    let finished_ok = tokio::task::spawn_blocking(move || {
        let start = Instant::now();
        loop {
            if cancel2.is_cancelled() || start.elapsed() > Duration::from_secs(1) {
                #[cfg(unix)]
                unsafe {
                    // Kill the whole process group: -pid
                    libc::kill(-(pid as i32), libc::SIGKILL);
                }
                let _ = child.wait();
                return false;
            }
            match child.try_wait() {
                Ok(Some(status)) => return status.success(),
                Ok(None) => {
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(_) => return false,
            }
        }
    })
        .await
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    if !finished_ok {
        return Ok(None);
    }

    if !out_png.exists() {
        if let Some(stem) = input.file_stem().and_then(|s| s.to_str()) {
            if let Ok(rd) = fs::read_dir(&tmp_dir) {
                for e in rd.flatten() {
                    let p = e.path();
                    if p.extension().and_then(|e| e.to_str()) == Some("png")
                        && p.file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.starts_with(stem))
                        .unwrap_or(false)
                    {
                        out_png = p;
                        break;
                    }
                }
            }
        }
    }

    if !out_png.exists() {
        return Ok(None);
    }

    let mut f = tokio::fs::File::open(&out_png)
        .await
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)
        .await
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;
    let _ = tokio::fs::remove_file(&out_png).await;

    let b64 = general_purpose::STANDARD.encode(buf);
    Ok(Some(format!("data:image/png;base64,{}", b64)))
}

// System thumbnail extraction (other OS)
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
async fn extract_thumbnail_system(
    _input: &std::path::Path,
    _size: u32,
    _cancel: &tokio_util::sync::CancellationToken,
) -> crate::errors::AppResult<Option<String>> {
    Ok(None)
}


pub async fn get_video_thumbnail_data_url(
    params: ThumbnailParams,
    cancel: &CancellationToken,
) -> AppResult<Option<String>> {
    let p = PathBuf::from(&params.path);
    if !p.exists() || !p.is_file() {
        return Ok(None);
    }

    // Try system thumbnail first.
    #[cfg(target_os = "windows")]
    {
        let path_cloned = p.clone();
        let cancel2 = cancel.clone();
        let sys_res = tauri::async_runtime::spawn_blocking(move || {
            tauri::async_runtime::block_on(extract_thumbnail_system(&path_cloned, 320, &cancel2))
        })
        .await
        .map_err(|e| AppError::new(AppErrorCode::Io, e.to_string()))?;

        if let Ok(Some(sys_thumb)) = sys_res {
            return Ok(Some(sys_thumb));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(Some(sys_thumb)) = extract_thumbnail_system(&p, 320, cancel).await {
            return Ok(Some(sys_thumb));
        }
    }

    // Fallback to ffmpeg.
    let ffmpeg_bin = resolve_ffmpeg_from_thumb(&params)?;
    extract_thumbnail_data_url(&ffmpeg_bin, &p, cancel).await
}
