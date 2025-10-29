use std::path::Path;

/// Resolve a binary path:
/// - If `custom` is provided and exists, use it.
/// - Otherwise, try common install locations (macOS and Linux).
/// - Fallback to the bare tool name so PATH can resolve it.
pub fn resolve_bin(custom: Option<&str>, tool: &str) -> String {
    if let Some(p) = custom {
        if Path::new(p).exists() {
            return p.to_string();
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
