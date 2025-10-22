use serde::Serialize;
use std::{path::Path, process::Command};

#[derive(Serialize)]
pub struct FfToolsStatus {
    pub ffmpeg: Option<String>,
    pub ffprobe: Option<String>,
}

fn resolve_bin(custom: &Option<String>, default: &str) -> String {
    if let Some(p) = custom {
        if Path::new(p).exists() {
            return p.clone();
        }
    }
    default.to_string()
}

fn version_of(bin: &str) -> Option<String> {
    let mut cmd = Command::new(bin);
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        std::os::windows::process::CommandExt::creation_flags(&mut cmd, CREATE_NO_WINDOW);
    }
    let output = cmd.arg("-version").output().ok()?;
    let text = if !output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    };
    let first_line = text.lines().next()?.trim().to_string();

    let bin_name = Path::new(bin)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(bin);

    let prefix = format!("{} version ", bin_name);
    if let Some(rest) = first_line.strip_prefix(&prefix) {
        if let Some(ver) = rest.split_whitespace().next() {
            return Some(ver.to_string());
        }
    }
    Some(first_line)
}

#[tauri::command]
pub fn check_ff_tools(ffmpeg_path: Option<String>, ffprobe_path: Option<String>) -> FfToolsStatus {
    let ffmpeg_bin = resolve_bin(&ffmpeg_path, "ffmpeg");
    let ffprobe_bin = resolve_bin(&ffprobe_path, "ffprobe");

    FfToolsStatus {
        ffmpeg: version_of(&ffmpeg_bin),
        ffprobe: version_of(&ffprobe_bin),
    }
}
