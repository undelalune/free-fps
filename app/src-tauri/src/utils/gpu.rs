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
use std::process::Command;
use tauri::AppHandle;

use crate::utils::bundled_ffmpeg::get_ffmpeg_path;

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// GPU vendor type for hardware-accelerated encoding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GpuType {
    Nvidia,
    Amd,
    Intel,
    None,
}

impl Default for GpuType {
    fn default() -> Self {
        GpuType::None
    }
}

/// Information about detected GPU and its encoding capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub gpu_type: GpuType,
    pub has_h264: bool,
    pub has_h265: bool,
    pub model_name: String,
}

impl Default for GpuInfo {
    fn default() -> Self {
        Self {
            gpu_type: GpuType::None,
            has_h264: false,
            has_h265: false,
            model_name: String::from("None"),
        }
    }
}

/// Apply platform-specific flags to hide console window
#[cfg(windows)]
fn apply_no_window(cmd: &mut Command) {
    cmd.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn apply_no_window(_cmd: &mut Command) {}

/// Test if a specific GPU encoder actually works (not just compiled into FFmpeg)
/// This is important because FFmpeg may have encoder support compiled in,
/// but the actual hardware/drivers may not be available.
fn test_gpu_encoding(ffmpeg_bin: &str, encoder: &str) -> bool {
    let mut cmd = Command::new(ffmpeg_bin);
    apply_no_window(&mut cmd);

    let output = cmd
        .args([
            "-f", "lavfi",
            "-i", "color=black:s=320x240:d=1",
            "-c:v", encoder,
            "-f", "null",
            "-",
        ])
        .output();

    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}

/// Detect available GPU encoders by checking FFmpeg AND testing actual encoding.
/// Priority order: NVIDIA > AMD > Intel
pub fn detect_gpu(ffmpeg_bin: &str) -> GpuInfo {
    let mut info = GpuInfo::default();

    // Get list of available encoders from FFmpeg
    let mut cmd = Command::new(ffmpeg_bin);
    apply_no_window(&mut cmd);

    let output = cmd.args(["-hide_banner", "-encoders"]).output();

    let stdout = match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => return info,
    };

    // Check NVIDIA (highest priority) - NVENC
    if stdout.contains("h264_nvenc") && test_gpu_encoding(ffmpeg_bin, "h264_nvenc") {
        info.gpu_type = GpuType::Nvidia;
        info.has_h264 = true;
        info.has_h265 =
            stdout.contains("hevc_nvenc") && test_gpu_encoding(ffmpeg_bin, "hevc_nvenc");
        info.model_name = get_gpu_model(&["NVIDIA", "GeForce", "RTX", "Quadro"]);
        return info;
    }

    // Check AMD - AMF (Advanced Media Framework)
    if stdout.contains("h264_amf") && test_gpu_encoding(ffmpeg_bin, "h264_amf") {
        info.gpu_type = GpuType::Amd;
        info.has_h264 = true;
        info.has_h265 = stdout.contains("hevc_amf") && test_gpu_encoding(ffmpeg_bin, "hevc_amf");
        info.model_name = get_gpu_model(&["AMD", "Radeon"]);
        return info;
    }

    // Check Intel - QuickSync Video
    if stdout.contains("h264_qsv") && test_gpu_encoding(ffmpeg_bin, "h264_qsv") {
        info.gpu_type = GpuType::Intel;
        info.has_h264 = true;
        info.has_h265 = stdout.contains("hevc_qsv") && test_gpu_encoding(ffmpeg_bin, "hevc_qsv");
        info.model_name = get_gpu_model(&["Intel"]);
        return info;
    }

    info
}

/// Get GPU model name on Windows using PowerShell Get-CimInstance
/// (modern replacement for deprecated wmic)
#[cfg(target_os = "windows")]
fn get_gpu_model(vendor_keywords: &[&str]) -> String {
    let mut cmd = Command::new("powershell");
    apply_no_window(&mut cmd);

    let output = cmd
        .args([
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name",
        ])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let line_upper = line.to_uppercase();
            if vendor_keywords
                .iter()
                .any(|kw| line_upper.contains(&kw.to_uppercase()))
            {
                return line.trim().to_string();
            }
        }
    }

    // Fallback to generic name based on first keyword
    vendor_keywords
        .first()
        .map(|v| format!("{} GPU", v))
        .unwrap_or_else(|| "Unknown GPU".to_string())
}

/// Get GPU model name on macOS using system_profiler
#[cfg(target_os = "macos")]
fn get_gpu_model(vendor_keywords: &[&str]) -> String {
    let output = Command::new("system_profiler")
        .args(["SPDisplaysDataType"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let line_upper = line.to_uppercase();
            if vendor_keywords
                .iter()
                .any(|kw| line_upper.contains(&kw.to_uppercase()))
            {
                // Extract chipset name from lines like "Chipset Model: AMD Radeon Pro 5500M"
                if let Some(pos) = line.find(':') {
                    return line[pos + 1..].trim().to_string();
                }
                return line.trim().to_string();
            }
        }
    }

    vendor_keywords
        .first()
        .map(|v| format!("{} GPU", v))
        .unwrap_or_else(|| "Unknown GPU".to_string())
}

/// Tauri command to detect GPU and return information to the frontend
#[tauri::command]
pub async fn get_gpu_info(app: AppHandle) -> Result<GpuInfo, String> {
    let ffmpeg_bin = get_ffmpeg_path(&app).map_err(|e| {
        format!(
            "Failed to get FFmpeg path: {:?}",
            e.details.unwrap_or_else(|| "Unknown error".to_string())
        )
    })?;

    Ok(detect_gpu(&ffmpeg_bin.to_string_lossy()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_info_default() {
        let info = GpuInfo::default();
        assert_eq!(info.gpu_type, GpuType::None);
        assert!(!info.has_h264);
        assert!(!info.has_h265);
        assert_eq!(info.model_name, "None");
    }

    #[test]
    fn test_gpu_type_default() {
        let gpu_type = GpuType::default();
        assert_eq!(gpu_type, GpuType::None);
    }
}

