// rust
use chrono::Utc;
use std::{
    path::PathBuf,
    sync::OnceLock,
    time::{Duration, SystemTime},
};
use tauri::Manager;
use tokio::fs::{metadata, OpenOptions};
use tokio::io::AsyncWriteExt;

static LOG_PATH: OnceLock<PathBuf> = OnceLock::new();
const ROTATE_AFTER: Duration = Duration::from_secs(60 * 60 * 24 * 7); // 7 days

// Call this once at app startup to place the log next to `settings.json`
pub fn init_log_path(app: &tauri::AppHandle) {
    if let Ok(dir) = app.path().app_data_dir() {
        let _ = std::fs::create_dir_all(&dir);
        let _ = LOG_PATH.set(dir.join("log"));
    }
}

fn log_path() -> PathBuf {
    LOG_PATH
        .get()
        .cloned()
        // Fallback to CWD if not initialized (should be initialized in setup)
        .unwrap_or_else(|| PathBuf::from("log"))
}

async fn ensure_log_exists() -> std::io::Result<()> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path())
        .await
        .map(|_| ())
}

pub async fn rotate_log_if_needed() {
    if let Err(_) = ensure_log_exists().await {
        return;
    }

    if let Ok(meta) = metadata(log_path()).await {
        if let Ok(modified) = meta.modified() {
            if SystemTime::now()
                .duration_since(modified)
                .map(|d| d > ROTATE_AFTER)
                .unwrap_or(false)
            {
                let _ = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(log_path())
                    .await;
            }
        }
    }
}

async fn append_line(line: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path())
        .await
    {
        let _ = f.write_all(line.as_bytes()).await;
        let _ = f.write_all(b"\n").await;
    }
}

pub async fn log_ffmpeg_command(cmd: &str) {
    rotate_log_if_needed().await;
    let ts = Utc::now().to_rfc3339();
    let s = format!(r#"[{}] [FFMPEG CMD] {}"#, ts, cmd);
    append_line(&s).await;
}

pub async fn log_error(context: &str, details: &str) {
    rotate_log_if_needed().await;
    let ts = Utc::now().to_rfc3339();
    let s = format!(r#"[{}] [ERROR] {} : {}"#, ts, context, details);
    append_line(&s).await;
}
