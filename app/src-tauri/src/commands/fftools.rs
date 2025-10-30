use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::utils::bins::resolve_bin;
use crate::utils::proc::apply_no_window_std;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCheckParams {
    pub tool: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct FfToolsStatus {
    pub ffmpeg: Option<String>,
    pub ffprobe: Option<String>,
}

// Strong identity check: ensure the binary claims to be the expected tool.
fn is_expected_ff_tool(bin: &str, expected: &str) -> bool {
    let mut cmd = std::process::Command::new(bin);
    apply_no_window_std(&mut cmd);

    let output = match cmd.arg("-version").output() {
        Ok(o) => o,
        Err(_) => return false,
    };

    if !output.status.success() {
        return false;
    }

    let text = if !output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stdout)
    } else {
        String::from_utf8_lossy(&output.stderr)
    };

    let first = text
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("")
        .to_ascii_lowercase();

    let needle = format!("{} version", expected.to_ascii_lowercase());
    first.starts_with(&needle) || first.contains(&needle)
}

fn version_of(bin: &str) -> Option<String> {
    let mut cmd = Command::new(bin);
    apply_no_window_std(&mut cmd);

    let output = cmd.arg("-version").output().ok()?;
    let text = if !output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    };
    let first_line = text.lines().next()?.trim().to_string();

    for tool in ["ffmpeg", "ffprobe"] {
        let prefix = format!("{} version ", tool);
        if let Some(rest) = first_line.to_ascii_lowercase().strip_prefix(&prefix) {
            if let Some(ver) = rest.split_whitespace().next() {
                return Some(ver.to_string());
            }
        }
    }
    Some(first_line)
}

#[tauri::command]
pub fn check_ff_tools(ffmpeg_path: Option<String>, ffprobe_path: Option<String>) -> FfToolsStatus {
    let ffmpeg_bin = resolve_bin(ffmpeg_path.as_deref(), "ffmpeg");
    let ffprobe_bin = resolve_bin(ffprobe_path.as_deref(), "ffprobe");

    FfToolsStatus {
        ffmpeg: version_of(&ffmpeg_bin),
        ffprobe: version_of(&ffprobe_bin),
    }
}

#[tauri::command]
pub fn check_ff_tool_selected(params: ToolCheckParams) -> bool {
    let bin = resolve_bin(Some(&params.path), &params.tool);
    match params.tool.as_str() {
        "ffmpeg" | "ffprobe" => is_expected_ff_tool(&bin, &params.tool),
        _ => false,
    }
}
