// app/src-tauri/src/utils/bundled_ffmpeg.rs
// Free FPS - Video Frame Rate Converter
// Copyright (C) 2025 undelalune
//
// GPLv3-or-later

use crate::errors::{AppError, AppErrorCode, AppResult};
use std::path::PathBuf;
use tauri::Manager;

#[cfg(unix)]
fn ensure_exec_bit(path: &PathBuf) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(metadata) = std::fs::metadata(path) {
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        let _ = std::fs::set_permissions(path, perms);
    }
}

// Try to locate a tool by name, considering macOS externalBin sidecars and resource `binaries/*`.
#[cfg(not(windows))]
fn find_tool_path(app: &tauri::AppHandle, base_name: &str) -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    // macOS: sidecars are placed next to the app executable
    #[cfg(target_os = "macos")]
    {
        if let Ok(mut exe_dir) = std::env::current_exe() {
            if exe_dir.pop() {
                // Unsuffixed sidecar (common)
                candidates.push(exe_dir.join(base_name));

                // Target-suffixed sidecar when building with --target
                let triple = if cfg!(target_arch = "x86_64") {
                    "x86_64-apple-darwin"
                } else {
                    "aarch64-apple-darwin"
                };
                candidates.push(exe_dir.join(format!("{base_name}-{triple}")));
            }
        }
    }

    // Fallback to packaged resources `binaries/*`
    if let Ok(resource_dir) = app.path().resource_dir() {
        #[cfg(windows)]
        let filename = format!("{base}.exe", base = base_name);
        #[cfg(not(windows))]
        let filename = base_name.to_string();

        candidates.push(resource_dir.join("binaries").join(filename));
    }

    for p in candidates {
        if p.exists() {
            return Some(p);
        }
    }
    None
}

#[cfg(windows)]
pub fn get_ffmpeg_path(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    let resource_path = app
        .path()
        .resource_dir()
        .map_err(|e| AppError::new(AppErrorCode::FfmpegNotFound, e.to_string()))?;
    let ffmpeg_path = resource_path.join("binaries").join("ffmpeg.exe");
    if !ffmpeg_path.exists() {
        return Err(AppError::new(
            AppErrorCode::FfmpegNotFound,
            format!("Bundled ffmpeg not found at: {:?}", ffmpeg_path),
        ));
    }
    Ok(ffmpeg_path)
}

#[cfg(not(windows))]
pub fn get_ffmpeg_path(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    let base = "ffmpeg";
    if let Some(path) = find_tool_path(app, base) {
        #[cfg(unix)]
        ensure_exec_bit(&path);
        return Ok(path);
    }
    Err(AppError::new(
        AppErrorCode::FfmpegNotFound,
        "Bundled ffmpeg not found in sidecar or resources".to_string(),
    ))
}

#[cfg(windows)]
pub fn get_ffprobe_path(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    let resource_path = app
        .path()
        .resource_dir()
        .map_err(|e| AppError::new(AppErrorCode::FfprobeNotFound, e.to_string()))?;
    let ffprobe_path = resource_path.join("binaries").join("ffprobe.exe");
    if !ffprobe_path.exists() {
        return Err(AppError::new(
            AppErrorCode::FfprobeNotFound,
            format!("Bundled ffprobe not found at: {:?}", ffprobe_path),
        ));
    }
    Ok(ffprobe_path)
}

#[cfg(not(windows))]
pub fn get_ffprobe_path(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    let base = "ffprobe";
    if let Some(path) = find_tool_path(app, base) {
        #[cfg(unix)]
        ensure_exec_bit(&path);
        return Ok(path);
    }
    Err(AppError::new(
        AppErrorCode::FfprobeNotFound,
        "Bundled ffprobe not found in sidecar or resources".to_string(),
    ))
}
