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
use std::path::PathBuf;
use tauri::Manager;

/// Get the path to bundled ffmpeg binary
pub fn get_ffmpeg_path(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    let resource_path = app
        .path()
        .resource_dir()
        .map_err(|e| AppError::new(AppErrorCode::FfmpegNotFound, e.to_string()))?;

    let binary_name = if cfg!(windows) {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    };

    let ffmpeg_path = resource_path.join("binaries").join(binary_name);

    if !ffmpeg_path.exists() {
        return Err(AppError::new(
            AppErrorCode::FfmpegNotFound,
            format!("Bundled ffmpeg not found at: {:?}", ffmpeg_path),
        ));
    }

    // Ensure executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(&ffmpeg_path) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            let _ = std::fs::set_permissions(&ffmpeg_path, perms);
        }
    }

    Ok(ffmpeg_path)
}

/// Get the path to bundled ffprobe binary
pub fn get_ffprobe_path(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    let resource_path = app
        .path()
        .resource_dir()
        .map_err(|e| AppError::new(AppErrorCode::FfprobeNotFound, e.to_string()))?;

    let binary_name = if cfg!(windows) {
        "ffprobe.exe"
    } else {
        "ffprobe"
    };

    let ffprobe_path = resource_path.join("binaries").join(binary_name);

    if !ffprobe_path.exists() {
        return Err(AppError::new(
            AppErrorCode::FfprobeNotFound,
            format!("Bundled ffprobe not found at: {:?}", ffprobe_path),
        ));
    }

    // Ensure executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(&ffprobe_path) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            let _ = std::fs::set_permissions(&ffprobe_path, perms);
        }
    }

    Ok(ffprobe_path)
}