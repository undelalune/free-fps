use serde::{Deserialize, Serialize};
use std::{path::Path, process::Command};

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

fn resolve_bin(custom: Option<&String>, tool: &str) -> String {
    if let Some(p) = custom {
        if Path::new(p).exists() {
            return p.clone();
        }
    }
    #[cfg(target_os = "macos")]
    {
        for dir in ["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin"] {
            let cand = Path::new(dir).join(tool);
            if cand.exists() {
                return cand.to_string_lossy().to_string();
            }
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        for dir in ["/usr/local/bin", "/usr/bin"] {
            let cand = Path::new(dir).join(tool);
            if cand.exists() {
                return cand.to_string_lossy().to_string();
            }
        }
    }
    tool.to_string()
}

// Strong identity check: ensure the binary claims to be the expected tool.
fn is_expected_ff_tool(bin: &str, expected: &str) -> bool {
    let mut cmd = std::process::Command::new(bin);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

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
    // Be permissive with vendor prefixes but require "<tool> version".
    first.starts_with(&needle) || first.contains(&needle)
}

fn version_of(bin: &str) -> Option<String> {
    let mut cmd = Command::new(bin);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let output = cmd.arg("-version").output().ok()?;
    let text = if !output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    };
    let first_line = text.lines().next()?.trim().to_string();

    // Try to parse "ffmpeg version X" or "ffprobe version X"
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
    let ffmpeg_bin = resolve_bin(ffmpeg_path.as_ref(), "ffmpeg");
    let ffprobe_bin = resolve_bin(ffprobe_path.as_ref(), "ffprobe");

    FfToolsStatus {
        ffmpeg: version_of(&ffmpeg_bin),
        ffprobe: version_of(&ffprobe_bin),
    }
}

#[tauri::command]
pub fn check_ff_tool_selected(params: ToolCheckParams) -> bool {
    let bin = super::fftools::resolve_bin(Some(&params.path), &params.tool);
    match params.tool.as_str() {
        "ffmpeg" | "ffprobe" => is_expected_ff_tool(&bin, &params.tool),
        _ => false,
    }
}
