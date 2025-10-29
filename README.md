# Free‑FPS

<p align="center">
  <img src="app/src/md/logo.png" alt="Free‑FPS logo" width="120" style="border-radius: 24px;">
</p>

Open‑source desktop app and scripts to change a video file frame rate \(FPS\) using FFmpeg. It does not add effects or alter content; it adjusts playback speed and, if needed, re‑encodes audio. Originals are never modified.

## Quick links

- App user guide: 
  [Беларуская](app/src/md/by.MD), [Deutsch](app/src/md/de.MD), [English](app/src/md/en.MD), [Español](app/src/md/es.MD),
  [Français](app/src/md/fr.MD), [Italiano](app/src/md/it.MD), [Polski](app/src/md/pl.MD), [Português](app/src/md/pt.MD),
  [Русский](app/src/md/ru.MD), [Українська](app/src/md/ua.MD)

- Scripts user guide: [Windows](scripts/win/README.md), [MacOS](scripts/unix/README.md) - you can also run the scripts directly with PowerShell or bash/sh if you don't want to use the desktop app.

- Downloads
    - Latest release: https://github.com/undelalune/free-fps/releases/latest/
    - All releases: https://github.com/undelalune/free-fps/releases
    - Windows: Installer and Portable \(Portable recommended - free-fps.exe file \)
    - macOS: Universal build

## Requirements

- FFmpeg and FFprobe are required.
    - Uses versions found in system `PATH`, or set explicit paths in app settings.
    - Get FFmpeg: [FFmpeg downloads](https://ffmpeg.org/download.html) (or see localized App's guides above).

## What it does

- Changes video FPS via FFmpeg.
- Keeps or re‑encodes audio if requested.
- No effects, no content edits.
- Writes results to a separate output folder by default.

## Desktop app

- Platforms
    - Windows 10 1809\+ and Windows 11. Requires Microsoft Edge WebView2 runtime. Older Windows versions are not supported and the app may not start.
    - macOS 10.15\+ on Intel and Apple Silicon. Older macOS versions are not supported and the app may not start.
- UI tech is web‑based. Internet Explorer is not supported. Docs/UI render correctly in modern Edge, Firefox, Chrome, and Safari \(latest two versions\).

## Scripts

- Windows PowerShell and macOS shell scripts perform the same conversion with fewer controls.
- FFmpeg and FFprobe must be available in `PATH`. See per‑platform guides above.

## Support the project

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener">
  <img src="app/src/md/bmc-logo.svg" alt="Buy Me a Coffee" height="36">
</a>
&nbsp;&nbsp;
<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener">
  <img src="app/src/md/suppi-logo.svg" alt="Suppi" height="36">
</a>

## Tech stack

- Tauri 2 \(Rust\)
- Vue 3, TypeScript, Naive UI, Vite
- GitHub Actions for builds and releases

## Open source

- License: MIT
- Repository: https://github.com/undelalune/free-fps

## Build from source

- Prerequisites: Node.js LTS, Rust/Cargo, and Tauri prerequisites for your platform.
- Commands:
    - Install: `npm i`
    - Dev: `npm run tauri:dev`
    - Build: `npm run tauri:build`
