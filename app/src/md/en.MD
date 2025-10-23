- [About this tool](#about-this-tool) <img src="logo.png" style="float: right; margin-right:24px; margin-top:12px; height: 89px; " alt="Free FPS Logo" />
- [How to start?](#how-to-start)
- [Settings page](#settings-page)
- [Main page](#main-page)
- [Conversion page](#processing-page)

---

<a id="about-this-tool"></a>
### About this tool

A utility program for changing the video file frame rate (FPS) using FFmpeg. Unlike video editors, it does not add effects or alter content - it only adjusts playback speed and, if needed, re-encodes audio as well.

Useful if you work with multiple videos shot at different frame rates that cannot be combined or edited without interpolation or frame loss.

Important:
The program is intended for users who understand what FPS conversion does. Changing the frame rate affects the speed and duration of the video. The tool does not modify the original files!

This program is distributed for free ("As Is") under the MIT License.
If you want to support the author, you can do it here:

<a href="https://buymeacoffee.com/undelalune" target="_blank" rel="noopener" title="Go to buymeacoffee.com">
<img src="bmc-logo.svg" style="float: right; margin-right:24px; height: 36px; alt="bmc Logo" />
</a>

<a href="https://suppi.pl/undelalune" target="_blank" rel="noopener" title="Go to suppi.pl">
<img src="suppi-logo.svg" style="margin-left:44px; height: 36px;" alt="suppi Logo" />
</a>

<br>

---

<a id="how-to-start"></a>
### How to start?

On launch, the app automatically checks if the packages ffmpeg and ffprobe are available in the system `PATH` and will offer to use them.
If they are not present, download FFmpeg from the official site and extract the archive.<br>
Windows: <a href="https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" target="_blank" rel="noopener" title="Download ffmpeg/ffprobe archive">ffmpeg/ffprobe</a><br>
macOS: <a href="https://evermeet.cx/ffmpeg/ffmpeg-8.0.zip" target="_blank" rel="noopener" title="Download ffmpeg">ffmpeg</a> |
<a href="https://evermeet.cx/ffmpeg/ffprobe-8.0.zip" target="_blank" rel="noopener" title="Download ffprobe archive">ffprobe</a>

Installation into the system is not required - just provide the paths to the executables in the app settings.

- **Windows**: `bin/ffmpeg.exe` and `bin/ffprobe.exe`
- **macOS**: `ffmpeg` and `ffprobe` (at the root of the folder)

After that, you can start working.

---

<a id="settings-page"></a>
### Settings page

- **Path to ffmpeg** - path to the `ffmpeg` executable. This field is required.
- **Path to ffprobe** - path to `ffprobe`. Optional, but recommended for more stable operation.
- **Use installed version (ffmpeg / ffprobe)** - becomes available if installed versions are found.
- **Language** - select the interface language.
- **Theme** - choose the app color theme.
- **Reset settings** - reset saved data (only the form input values).

The app automatically saves your settings - you do not need to re-enter them on every launch.

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

<p style="text-align:center;">© 2025 undelalune</p>
