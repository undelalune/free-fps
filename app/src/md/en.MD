<img src="logo.png" style="border-radius: 8px; margin-right:16px; margin-top: 20px; height: 89px;" align="right" alt="Free FPS Logo" />

- [About this tool](#about-this-tool)
- [Main page](#main-page)
- [Conversion page](#processing-page)
- [License](#license)
- [Third-Party Software](#3d-party-license)
- [Source Code](#source-code)

---

<a id="about-this-tool"></a>
### About this tool

A utility program for changing the video file frame rate (FPS) using FFmpeg. Unlike video editors, it does not add effects or alter content - it only adjusts playback speed and, if needed, re-encodes audio as well.

Useful if you work with multiple videos shot at different frame rates that cannot be combined or edited without interpolation or frame loss.
Also doubles as a fast video compressor: keep the original FPS and raise compression (e.g., higher CRF or lower bitrate) to reduce file size.

Important:
The program is intended for users who understand what FPS conversion does. Changing the frame rate affects the speed and duration of the video. The tool does not modify the original files!

This program is distributed for free ("As Is") under the GNU General Public License v3.0 (GPL\-3.0).
If you want to support the author, you can do it here:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px;" alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px;" alt="suppi Logo" />
</a>

<br>

---

<a id="main-page"></a>
### Main page

- **Input Folder** - choose a folder with files to convert. (At this step, you cannot select a single file, but later you will be able to pick the videos you need.)
- **Output Folder** - specify the folder to save results. If left empty, files will be saved in an automatically created subfolder next to the originals.
- **Target FPS** - target frame rate.
- **Custom FPS** - set your own FPS value if the desired one is not in the list.
- **Keep Audio** - keeps the audio from the source file. The sound may be **slowed down or sped up**, so the option is disabled by default.
- **Audio Bitrate** - set the bitrate if you keep audio.
- **Custom Video Quality** - output video quality (CRF). By default, the app tries to preserve the source quality - this is recommended.
  Caution: too low a value results in poor quality; too high - huge files. More: [FFmpeg Encode H.264](https://trac.ffmpeg.org/wiki/Encode/H.264)
- **CPU Limit** - limit CPU usage. May slightly reduce load, but not guaranteed.

---

<a id="processing-page"></a>
### Conversion page

- Use checkboxes to select which videos to convert. All are selected by default.
- **Rescan Folder** - rescan the folder and refresh the file list.
- After starting the converter, you can cancel the process at any time.
- Video conversion is resource-intensive and may take time. Progress and status are shown on the screen.

> If conversion of any video finishes with an error, it is unlikely that a retry will succeed.
> It is possible that FFmpeg could not read the required data from the file.

---

<a id="license"></a>
### License
Free FPS is free software licensed under the **GNU General Public License v3.0**. You are free to:
- Use this software for any purpose
- Study and modify the source code
- Redistribute copies
- Distribute modified versions (under GPL v3)

Read project [license](FreeFPSLicense).

---

<a id="3d-party-license"></a>
### Third-Party Software
This program uses **FFmpeg**, which is also licensed under GPL v3.0.
- Read [FFmpeg Notice](FFmpegNotice)
- Read [FFmpeg License](FFmpegLicense)

---

<a id="source-code"></a>
### Source Code

The complete source code is available at:
https://github.com/undelalune/free-fps
 
<br/>

---

<p style="text-align:center;">Copyright © 2025 undelalune</p>
