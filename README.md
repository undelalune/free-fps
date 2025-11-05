# Free‑FPS [![GitHub release][release-img]][release-url]  [![license][license-url]](app/src-tauri/licenses/LICENSE.txt)

<img src="app/src/md/logo.png" alt="Free‑FPS logo" align="left" width="128" style="border-radius: 24px;">

Open‑source desktop app and scripts to change a video file frame rate \(FPS\) <ins>**using FFmpeg**</ins>. <br/>
It does not add effects or alter content; it adjusts playback speed and, if needed, re‑encodes audio. <br/>
Originals are never modified.


Download [latest release](https://github.com/undelalune/free-fps/releases/latest/) for Windows / macOS
<br/><br/>

## What it does

- Changes video FPS via FFmpeg.
- Keeps or re‑encodes audio if requested.
- No effects, no content edits.
- Writes results to a separate output folder by default.
- Also doubles as a fast video compressor: keep the original FPS and raise compression (e.g., higher CRF or lower bitrate) to reduce file size.

<p align="center">
  <a href="app/docs/previews/home.png?raw=1">
    <img src="app/docs/previews/home.png" alt="home" width="260" style="max-width: 260px; width: auto;" loading="lazy">
  </a>
  <a href="app/docs/previews/home2.png?raw=1">
    <img src="app/docs/previews/home2.png" alt="home2" width="260" style="max-width: 260px; width: auto;" loading="lazy">
  </a>
  <a href="app/docs/previews/processing3.png?raw=1">
    <img src="app/docs/previews/processing3.png" alt="processing3" width="260" style="max-width: 260px; width: auto;" loading="lazy">
  </a>
  <a href="app/docs/previews/processing1.png?raw=1">
    <img src="app/docs/previews/processing1.png" alt="processing1" width="260" style="max-width: 260px; width: auto;" loading="lazy">
  </a>
  <a href="app/docs/previews/processing2.png?raw=1">
    <img src="app/docs/previews/processing2.png" alt="processing2" width="260" style="max-width: 260px; width: auto;" loading="lazy">
  </a>
  <a href="app/docs/previews/help.png?raw=1">
    <img src="app/docs/previews/help.png" alt="help" width="260" style="max-width: 260px; width: auto;" loading="lazy">
  </a>
</p>


## First Run
> Your OS may warn or block the first launch because the app is not signed/notarized. <br/>
> Look here for instructions to allow it: [Windows 10/11](app/docs/first_run_win.MD) or [macOS 10.15\+](app/docs/first_run_mac.MD)


## Quick links

- App user guide:
  [Беларуская](app/src/md/by.MD), [Deutsch](app/src/md/de.MD), [English](app/src/md/en.MD), [Español](app/src/md/es.MD),
  [Français](app/src/md/fr.MD), [Italiano](app/src/md/it.MD), [Polski](app/src/md/pl.MD), [Português](app/src/md/pt.MD),
  [Русский](app/src/md/ru.MD), [Українська](app/src/md/ua.MD)

- You can also run the scripts directly with PowerShell or bash/sh if you don't want to use the desktop app. <br/>
  Scripts user guide: [Windows](scripts/win/README.md), [MacOS](scripts/unix/README.md).

## Desktop app

- Platforms
    - Windows 10 1809\+ and Windows 11. Requires Microsoft Edge WebView2 runtime. Older Windows versions are not supported and the app may not start.
    - macOS 10.15\+ on Intel and Apple Silicon. Older macOS versions are not supported and the app may not start.
- UI tech is web‑based. Internet Explorer is not supported. Docs/UI render correctly in modern Edge, Firefox, Chrome, and Safari \(latest two versions\).

## Scripts

- Windows PowerShell and macOS shell scripts perform the same conversion with fewer controls.
- FFmpeg and FFprobe must be available (in `PATH` or standalone). See per‑platform guides above.

## Support the project

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener">
  <img src="app/src/md/bmc-logo.svg" alt="Buy Me a Coffee" height="36">
</a>
&nbsp;&nbsp;
<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener">
  <img src="app/src/md/suppi-logo.svg" alt="Suppi" height="36">
</a>

## Tech stack
[![tauri-img]][tauri-url] [![vue-img]][vue-url] [![vite-img]][vite-url]
- Tauri 2 \(Rust\)
- Vue 3, TypeScript, Naive UI, Vite
- GitHub Actions for builds and releases

## For developers
### Build from source

- Prerequisites: Node.js LTS, Rust/Cargo, and Tauri prerequisites for your platform.
- Commands:
    - Install: `npm i`
    - Dev: `npm run tauri:dev`
    - Build: `npm run tauri:build`

### Licence

> This software uses FFmpeg (GPL v3.0). License and source code information available in the app's Help section.

- [GNU GPLv3](app/src-tauri/licenses/LICENSE.txt)
- [FFMPEG_NOTICE](app/src-tauri/licenses/FFMPEG_NOTICE.txt)


[release-img]:     https://img.shields.io/github/v/release/undelalune/free-fps
[release-url]:     https://github.com/undelalune/free-fps/releases/latest/
[license-url]:     https://img.shields.io/github/license/Nats-ji/paper-sand-dupe-unpatched?style=flat-rounded
[tauri-img]:       https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=Tauri&logoColor=white
[tauri-url]:       https://tauri.app/
[vue-img]:         https://img.shields.io/badge/Vue%20js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[vue-url]:         https://vuejs.org/
[vite-img]:        https://img.shields.io/badge/Vite-B73BFE?style=for-the-badge&logo=vite&logoColor=FFD62E
[vite-url]:        https://vite.dev/
