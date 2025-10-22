# Video FPS Converter Script (Unix)

This script batch-converts videos to a target frame rate (FPS) **without dropping or duplicating frames**, using `ffmpeg`.  
It works on Unix-like systems (Linux, macOS) and supports multiple formats and flexible configuration via command-line options.

> ⚠️ This script changes playback speed to match target FPS — **no frames are dropped or interpolated**.
---

## Features

- Works from any folder (no system-wide installation of `ffmpeg` required if you pass a custom binary path).
- Supports `.mp4`, `.mov`, `.avi`, `.mkv`, `.webm`.
- Removes audio by default (use `-k` to keep audio).
- CRF defines video quality:
  - `0 = lossless`
  - `18–23 = high quality (default: 18)`
  - `28+ = lower quality`
  - `51 = worst possible`
- New: optional bitrate mode computes a target video bitrate from the source file size and adjusted duration and uses `-b:v` instead of `-crf`.

---

## Bitrate mode (`-u`)

When enabled with `-u` (or `--use-bitrate`), the script computes a target video bitrate to keep approximate file size proportional to playback duration after the FPS change. The computation:

- original file size (bytes) = S
- original duration (seconds) = D
- source FPS = src_fps
- target FPS = tfps

new_duration = D * (src_fps / tfps)

target_kbps = (S * 8) / new_duration / 1000

The script rounds to whole kbps and clamps to at least `1k`. If any detection step fails (missing `Duration`, `fps`, or file size), the script falls back to CRF mode.

---

## Supported Options

| Option             | Description                                                                | Example                     | Default                            |
|--------------------|----------------------------------------------------------------------------|-----------------------------|------------------------------------|
| `-d DIR`           | Folder containing the source videos                                        | `-d /path/to/videos`        | `.`                                |
| `-f FPS`           | Target output FPS                                                          | `-f 25`                     | `25`                               |
| `-k`               | Keep audio (omit to remove audio)                                          | `-k`                        | removed by default                 |
| `-c CRF`           | Video quality (0–51, lower = better)                                       | `-c 18`                     | `18`  (ignored by default, see -u) |
| `-p FFMPEG_PATH`   | Path to `ffmpeg` binary                                                    | `-p /usr/local/bin/ffmpeg`  | `ffmpeg`                           |
| `-p FFPROBE_PATH`  | Path to `ffprobe` binary                                                   | `-p /usr/local/bin/ffprobe` | `ffprobe`                          |
| `-o OUTPUT_FOLDER` | Name of the output folder inside the input dir                             | `-o converted_out`          | `converted_fps_<FPS>`              |
| `-b AUDIO_BITRATE` | Audio bitrate in kbps (when keeping audio)                                 | `-b 192`                    | `192`                              |
| `-u`               | Use bitrate mode (compute target bitrate and use `-b:v` instead of `-crf`) | `-u`                        | enabled                            |

---

## Usage

Make the script executable:
   ```shell 
      chmod +x ./convert_fps.sh
   ```

or run with bash/sh:

   ```shell
      bash ./convert_fps.sh
   ```
   ```shell
      sh ./convert_fps.sh
   ```
---
### ⚠️ Bellows you'll see examples of usage with `sh`.

**Basic example with the default parameters :**
  ```shell 
     sh ./convert_fps.sh
  ```

**___Results:___** video files are in the same folder as script, output folder with name `converted_fps_25` will be created, audio removed, target FPS is 25, video quality is high (CRF 18)

--- 

**Keep audio:**
  ```shell 
     sh ./convert_fps.sh -k
  ```

**___Results:___** same as default parameters, but the audio is kept, audio bitrate is 192 kbps.


**Custom ffmpeg binary:**
```shell 
   sh ./convert_fps.sh -p /usr/local/bin/ffmpeg
```

**___Results:___** same as default parameters, but using custom ffmpeg binary.


**Full example:**
  ```shell 
     sh ./convert_fps.sh -d /media/videos -f 30 -k -c 18 -p /usr/bin/ffmpeg -P /usr/bin/ffprobe -o converted_out -b 256
  ```


**___Results:___**

* `-d /media/videos` -\- input folder
* `-f 30` -\- target FPS
* `-k` -\- keep audio
* `-c 18` -\- high video quality, ignored by default since `-u` is enabled
* `-p /usr/bin/ffmpeg` -\- custom ffmpeg binary path
* `-P /usr/bin/ffprobe` -\- custom ffprobe binary path
* `-o converted_out` -\- output folder name
* `-b 256` -\- audio bitrate in kbps
* `-u` -\- use bitrate mode (compute target video bitrate and use `-b:v` instead of `-crf`)

---

## Notes

- The script detects source FPS from `ffmpeg -i` output and adjusts video playback speed (`setpts`) and audio tempo (`atempo`) to preserve frame order.
- When keeping audio, `atempo` filters are automatically decomposed into allowed ranges (0.5–2.0) as needed.
- Output files are written to `./<INPUT_DIR>/converted_fps_<FPS>` by default (or the folder provided with `-o`).
