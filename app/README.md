# Free‑FPS App \(Tauri 2 \+ Vue 3 \+ Naive UI\)

Developer docs for the desktop app in `app`. The app changes video file frame rate \(FPS\) using FFmpeg/FFprobe. 
Built with Vue 3 and Naive UI. Project is open‑source under GPL‑3.0 license.

## Prerequisites

- Node.js LTS and npm
- Rust stable and Cargo
- Tauri platform prerequisites
    - Windows: MSVC toolchain \(Visual Studio Build Tools or VS\) and Microsoft Edge WebView2 runtime
    - macOS: Xcode Command Line Tools

FFmpeg can be in system `PATH` or configured in the app settings at runtime.

## Install

From repo root:

- `cd app`
- `npm i`

## Run \(desktop dev\)

- Windows: `npm run tauri:dev`
- macOS \(explicit Cargo path\): `npm run tauri:dev:mac`

This starts the Tauri desktop app with Vite dev server.

## Build \(desktop bundles\)

- Windows: `npm run tauri:build`
- macOS \(explicit Cargo path\): `npm run tauri:build:mac`

Artifacts are produced under `app/src-tauri/target/release` \(per‑platform subfolders\).

## NPM scripts

- `dev`: Vite dev server for the web UI
- `build`: Type‑check and build UI assets
- `preview`: Preview built UI
- `tauri`, `tauri:dev`, `tauri:build`, `tauri:dev:mac`, `tauri:build:mac`: Tauri helpers

See `app/package.json` for the full list.

## Tech stack

- Desktop: Tauri 2 \(Rust\)
    - Crates: `tauri`, `tokio`, `serde`, `serde_json`, `regex`, `chrono`, `filetime`, `base64`, `libc`
    - Plugins: `tauri-plugin-fs`, `tauri-plugin-dialog`, `tauri-plugin-shell`, `tauri-plugin-store`, `tauri-plugin-opener`, `tauri-plugin-os`
    - Windows‑only: `windows`, `windows-sys`
- Frontend: Vue 3, TypeScript, Vite, Pinia, Vue Router
    - UI library: Naive UI
    - Utilities: `markdown-it`, `dompurify`

## Project structure

- `app/src`: Vue 3 UI
- `app/src-tauri`: Rust \(Tauri app, plugins, build config\)
- `app/src/md`: In‑app user guides and assets

Tauri config files live in `app/src-tauri`.

## Runtime notes

- `ffmpeg` and `ffprobe` are required to perform conversions.
    - Detected from system `PATH`, or set explicit paths in the app’s Settings page.
    - Official downloads: https://ffmpeg.org/download.html
- Supported platforms
    - Windows 10 1809\+ and Windows 11 \(requires WebView2 runtime\)
    - macOS 10.15\+ \(Intel and Apple Silicon\)
    - Older OS versions are not supported and the app may not start.

## Troubleshooting

- WebView2 missing \(Windows\): install the WebView2 runtime.
- `ffmpeg`/`ffprobe` not found: add to `PATH` or set paths in app settings.
- Rust toolchain errors: ensure Rust stable MSVC \(Windows\) or standard toolchain \(macOS\) is installed and on `PATH`.

## License

GNU General Public License v3.0. This software uses FFmpeg (GPL v3.0). License and source code information available in the app's Help section.
