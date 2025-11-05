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

use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseType {
    FFmpegNotice,
    FFmpegLicense,
    FreeFPSLicense,
}

#[tauri::command]
pub fn open_bundled_license(license: LicenseType, app_handle: tauri::AppHandle) -> Result<(), String> {
    let resource_dir = app_handle
        .path()
        .resource_dir()
        .map_err(|e| format!("license resource dir error: {}", e))?;

    let src = match license {
        LicenseType::FFmpegNotice => resource_dir.join("licenses").join("FFMPEG_NOTICE.txt"),
        LicenseType::FFmpegLicense => resource_dir.join("licenses").join("FFMPEG_LICENSE.txt"),
        LicenseType::FreeFPSLicense => resource_dir.join("licenses").join("LICENSE.txt"),
    };

    if !src.exists() {
        return Err(format!("file not found: {}", src.display()));
    }

    let mut dst = std::env::temp_dir();
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    dst.push(match license {
        LicenseType::FFmpegNotice => format!("FFMPEG_NOTICE_{}.txt", ts),
        LicenseType::FFmpegLicense => format!("FFMPEG_LICENSE_{}.txt", ts),
        LicenseType::FreeFPSLicense => format!("LICENSE_{}.txt", ts),
    });

    fs::copy(&src, &dst).map_err(|e| e.to_string())?;
    open::that(&dst).map_err(|e| e.to_string())?;
    Ok(())
}
