
# Video FPS Converter Script (PowerShell)

This script batch-converts videos to a target frame rate (FPS) **without dropping or duplicating frames**, using `ffmpeg`.  
It works on Windows and supports multiple formats and flexible configuration via command-line parameters.

> ⚠️ This script changes playback speed to match target FPS — **no frames are dropped or interpolated**.
---

Notes:

- Bitrate mode attempts to preserve approximate resulting file size relative to play time after FPS adjustment. Actual quality/size will vary depending on source complexity and encoder settings.
> ⚠️ If bitrate mode cannot determine duration/fps/size, the script will warn and fall back to CRF mode.
- Audio handling and `atempo` decomposition remain unchanged.

---

## Features

- Works from any folder (no system-wide installation of `ffmpeg` and `ffprobe` required).
- Supports `.mp4`, `.mov`, `.avi`, `.mkv`, `.webm`.
- Removes audio by default (use `-KeepAudio $true` to keep).
- CRF defines video quality:
     - `0 = lossless`
     - `18–23 = high quality (default: 18)`
     - `28+ = lower quality`
     - `51 = worst possible`
- New: `-AutoVideoBr` switch computes a target video bitrate from the source file size and adjusted duration and uses `-b:v` instead of `-crf`.
## Bitrate mode (`-AutoVideoBr`)

When `-AutoVideoBr` is provided, the script computes a target bitrate using the same formula as the Unix script:

- original file size (bytes) = S
- original duration (seconds) = D
- source FPS = src_fps
- target FPS = tfps

new_duration = D * (src_fps / tfps)

target_kbps = round((S * 8) / new_duration / 1000)

The resulting target is used as `-b:v <target_kbps>k -c:v libx264 -preset slow`. 
If detection fails, the script warns and uses CRF mode instead.

---

## Supported Parameters

| Parameter       | Description                                                        | Example                                  | Default Value                 |
|-----------------|--------------------------------------------------------------------|------------------------------------------|-------------------------------|
| `-Dir`          | Folder containing the source videos                                | `-Dir "D:\videos"`                       | `.` (current directory)       |
| `-Fps`          | Target output FPS                                                  | `-Fps 25`                                | `25`                          |
| `-KeepAudio`    | Keep audio (`$true`) or remove (`$false`)                          | `-KeepAudio $false`                      | `$false` (remove audio)       |
| `-Crf`          | Video quality (0–51, lower = better)                               | `-Crf 18`                                | `18` (pretty good quality)    |
| `-FfmpegPath`   | Absolute path to `ffmpeg.exe`                                      | `-FfmpegPath "C:\ffmpeg\bin\ffmpeg.exe"` | `"C:\ffmpeg\bin\ffmpeg.exe"`  |
| `-FfprobePath`  | Absolute path to `ffprobe.exe`                                     | `-FfmpegPath "C:\ffmpeg\bin\probe.exe"`  | `"C:\ffmpeg\bin\ffprobe.exe"` |
| `-OutputFolder` | Name of the output folder                                          | `-OutputFolder "converted_25_fps"`       | `converted_25_fps`            |
| `-AudioBitrate` | Output bitrate of the audio if needed                              | `-AudioBitrate 128`                      | `192` (kbps)                  |
| `-AutoVideoBr`  | Recalculates bitrate automatically (uses `-b:v` instead of `-crf`) | `-AutoVideoBr $false`                    | `$true` (enabled)             |

---

## Example Usage

```powershell
# Run with default values
.\convert_fps.ps1

# Full custom usage
.\convert_fps.ps1 -Dir "D:\videos" -Fps 30 -KeepAudio $false -Crf 20 -FfmpegPath "C:\Tools\ffmpeg\ffmpeg.exe" -FfprobePath "C:\Tools\ffmpeg\ffprobe.exe" -OutputFolder "ready_30fps" -AudioBitrate 128 -u $false
```


## An example of a workflow.

- We downloaded **ffmpeg.exe** and **ffprobe.exe** and placed it at `c:\ffmpeg\` (for example, the path can be anything).

- For convenience, we want our script to be in the same folder, i.e., `c:\ffmpeg\convert_fps.ps1` (for example, the path can be anything).

- We must first specify this path (to `ffmpeg.exe`) in the main **`convert_fps.ps1`** script. If we don't, we'll have to pass it in the command to the script every time. So, in `convert_fps.ps1`, we set `[string]$FfmpegPath = "C:\ffmpeg\bin\ffmpeg.exe"`.

- We want to create several folders for conversions, so that each uses specific parameters.

### Conversion to 50 fps

We create the folder `d:\vid-src\to-fps-50`. We'll put files that need to be converted to 50 fps here.

- We place the [to50fps.bat](to50fps.bat) file with the following content in this folder:
   `powershell c:\ffmpeg\convert_fps.ps1 -Dir d:\vid-src\to-fps-50 -Fps 50`

- Now, when you run the `.bat` file (for example, by double-clicking), the conversion will start, and upon completion, a `converted_50_fps` folder will be created in `d:\vid-src\to-fps-50` with the transcoded videos.

- After this, you can take the videos and clear the folder, leaving only the `.bat` file for future use.

---

### Conversion to 25 fps with audio

We create the folder **`d:\vid-src\to-fps-25-audio`**. We'll put files that need to be converted to 25 fps and have their audio transcoded here.

- We place the [to25fpsAudio.bat](to25fpsAudio.bat) file with the following content in this folder:
   `powershell c:\ffmpeg\convert_fps.ps1 -Dir d:\vid-src\to-fps-25-audio -Fps 25 -KeepAudio $true`

- Now, when you run the `.bat` file (for example, by double-clicking), the conversion will start, and upon completion, a `converted_25_fps` folder will be created in `d:\vid-src\to-fps-25` with the transcoded videos.

- After this, you can take the videos and clear the folder, leaving only the `.bat` file for future use.

---

You can create as many of these folders with **`.bat`** files as you want (for conversions with different sets of parameters), leaving the original script and **ffmpeg** in one place.
