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
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10MB

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
        let size_exceeded = meta.len() > MAX_LOG_SIZE;
        let time_exceeded = if let Ok(modified) = meta.modified() {
            SystemTime::now()
                .duration_since(modified)
                .map(|d| d > ROTATE_AFTER)
                .unwrap_or(false)
        } else {
            false
        };

        if size_exceeded || time_exceeded {
            // Rotate by truncating the log file
            let _ = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(log_path())
                .await;

            // Log rotation event
            let ts = Utc::now().to_rfc3339();
            let reason = if size_exceeded { "size exceeded" } else { "time exceeded" };
            let line = format!("[{}] [LOG] Log rotated ({})", ts, reason);
            append_line(&line).await;
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
