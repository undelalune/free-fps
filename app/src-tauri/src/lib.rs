mod commands;
mod errors;
mod utils;

use commands::fftools::check_ff_tools;
use commands::video::{cancel_conversion, convert_videos, get_video_files, get_video_files_with_thumbnails, get_video_thumbnail, ConversionController};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ConversionController::default())
        // Initialize the log file path next to `settings.json`
        .setup(|app| {
            // Pass an AppHandle to the logger initializer
            crate::utils::logger::init_log_path(&app.handle());
            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_video_files,
            get_video_files_with_thumbnails,
            get_video_thumbnail,
            convert_videos,
            cancel_conversion,
            check_ff_tools
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
