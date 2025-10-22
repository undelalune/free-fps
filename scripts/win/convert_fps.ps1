param (
    [string]$Dir = ".",
    [int]$Fps = 25,
    [bool]$KeepAudio = $false,
    [int]$Crf = 18,
    [string]$FfmpegPath = "C:\ffmpeg\bin\ffmpeg.exe",
    [string]$FfprobePath = "C:\ffmpeg\bin\ffprobe.exe",
    [string]$OutputFolder = "",
    [int]$AudioBitrate = 192,
    [bool]$AutoVideoBr = $true
)

# ========== CHECK FFMPEG ==========
if (!(Test-Path $FfmpegPath)) {
    Write-Error "ffmpeg not found at: $FfmpegPath"
    exit 1
}

#========== CHECK FFPROBE =========
if (!(Test-Path $FfprobePath)) {
    Write-Error "ffprobe not found at: $FfprobePath"
    exit 1
}

# ========== PREPARE OUTPUT DIR ==========
$InputDir = Resolve-Path $Dir
$OutputDir = if ($OutputFolder -eq "") {
    Join-Path $InputDir "converted_fps_$Fps"
} else {
    Join-Path $InputDir $OutputFolder
}

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

# ========== PRINT CONFIG ==========
Write-Host "ffmpeg: $FfmpegPath"
Write-Host "ffprobe: $FfprobePath"
Write-Host "Input folder: $InputDir"
Write-Host "Output folder: $OutputDir"
Write-Host "Target FPS: $Fps"
Write-Host "Remove audio: $( !($KeepAudio) )"
Write-Host "CRF: $Crf"
Write-Host "Use bitrate mode: $AutoVideoBr"
Write-Host ""

# ========== SUPPORTED EXTENSIONS ==========
$extensions = @("*.mp4", "*.mov", "*.mkv", "*.avi", "*.webm")

# ========== PROCESSING ==========
foreach ($ext in $extensions) {
    Get-ChildItem -Path $InputDir -Filter $ext | ForEach-Object {
        $source_file_path = $_.FullName
        $original_file_name = $_.BaseName
        $extension = $_.Extension.TrimStart(".")
        $output_file_path = Join-Path $OutputDir "${original_file_name}_${Fps}fps.$extension"
        $modification_time = $_.LastWriteTime

        $ffprobeCmd = "$FfprobePath -v quiet -print_format json -show_entries format_tags=creation_time -i `"$source_file_path`""
        $ffprobeResult = Invoke-Expression $ffprobeCmd | ConvertFrom-Json
        $metadata_creation_time = $ffprobeResult.format.tags.creation_time

        if (-not $metadata_creation_time) {
            $metadata_creation_time = $modification_time.ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
        }

        Write-Host "----------------------------------------"
        Write-Host "Start processing: $( $_.Name ) File: $metadata_creation_time (modified: $modification_time)"

        # get source fps
        $src_fps_match = & $FfmpegPath -i $source_file_path -hide_banner 2>&1 |
                Select-String "Stream.*Video" | Select-Object -First 1

        if (-not $src_fps_match) {
            Write-Warning "Unable to detect FPS. Skipping."
            return
        }

        $src_fps_line = $src_fps_match.Line

        if ($src_fps_line -match '(\d+(?:\.\d+)?)\s*fps') {
            $src_fps = [double]$matches[1]
            $setpts = [math]::Round($src_fps / $Fps, 5)
            $atempo = [math]::Round($Fps / $src_fps, 5)
            Write-Host "  FPS: $src_fps → setpts: $setpts / atempo: $atempo"
        } else {
            Write-Warning "FPS value not found in line: $src_fps_line. Skipping."
            return
        }

        # get duration (seconds)
        $duration_match = & $FfmpegPath -i $source_file_path -hide_banner 2>&1 |
            Select-String -CaseSensitive -Pattern '^\s*Duration:\s*\d+:\d+:\d+(?:\.\d+)?' |
            Select-Object -First 1

        if (-not $duration_match) {
            Write-Warning "Unable to detect duration. Skipping."
            return
        }

        $dur_line = $duration_match.Line
        if ($dur_line -match 'Duration:\s*(\d+):(\d+):(\d+(?:\.\d+)?)') {
            $hours = [int]$matches[1]
            $minutes = [int]$matches[2]
            $seconds = [double]$matches[3]
            $original_duration = ($hours * 3600) + ($minutes * 60) + $seconds
            Write-Host ("  Original duration: {0} s" -f $original_duration)
        } else {
            Write-Warning "Duration value not found in line: $dur_line. Skipping."
            return
        }

        # compute target bitrate if requested
        $videoArgs = ""
        if ($AutoVideoBr) {
            $original_size = $_.Length  # bytes
            if ($original_duration -le 0) {
                Write-Warning "Invalid original duration. Skipping bitrate computation."
                return
            }

            # new duration after changing FPS: multiply by src_fps / target_fps
            $new_duration = $original_duration * ($src_fps / $Fps)
            if ($new_duration -le 0) {
                Write-Warning "Invalid new duration. Skipping bitrate computation."
                return
            }

            # compute kbps: (bytes * 8) / seconds / 1000
            $target_bitrate_kbps = [math]::Round((($original_size * 8) / $new_duration) / 1000, 0)
            if ($target_bitrate_kbps -lt 1) { $target_bitrate_kbps = 1 }

            Write-Host "  Computed target bitrate: ${target_bitrate_kbps}k (new duration: $new_duration s)"
            $videoArgs = "-b:v ${target_bitrate_kbps}k -c:v libx264 -preset slow"
        } else {
            $videoArgs = "-c:v libx264 -crf $Crf -preset slow"
        }

        # audio args (preserve/build atempo chain or remove audio)
        $audioArgs = if ($KeepAudio) {
            if ($atempo -ge 0.5 -and $atempo -le 2.0) {
                "-c:a aac -b:a ${AudioBitrate}k -af " + '"' + ("atempo={0}" -f $atempo) + '"'
            } else {
                $filters = @()
                $remaining = $atempo
                while ($remaining -gt 2.0) {
                    $filters += "atempo=2.0"
                    $remaining /= 2.0
                }
                while ($remaining -lt 0.5) {
                    $filters += "atempo=0.5"
                    $remaining *= 2.0
                }
                $filters += ("atempo={0}" -f ([math]::Round($remaining, 3)))
                $audioFilter = $filters -join ","
                "-c:a aac -b:a ${AudioBitrate}k -af " + '"' + $audioFilter + '"'
            }
        } else {
            "-an"
        }

        $cmd = "$FfmpegPath -y -i " + '"' + $source_file_path + '"' +
                " -vf " + '"' + "setpts=$setpts*PTS" + '"' +
                " -r $Fps " + $videoArgs + " " + $audioArgs + " " +
                 '-metadata creation_time="' + $metadata_creation_time + '" "'  +
                 $output_file_path + '"'

        Write-Host "  Running command:"
        Write-Host "  $cmd"

        iex $cmd
        # set date back to original
        if ($metadata_creation_time) {
            # Convert metadata_creation_time (ISO 8601 string) to DateTime
            $metaDate = [datetime]::Parse($metadata_creation_time)
            (Get-Item $output_file_path).CreationTime = $metaDate
            (Get-Item $output_file_path).LastWriteTime = $metaDate
        } else {
            (Get-Item $output_file_path).CreationTime = $creation_time
            (Get-Item $output_file_path).LastWriteTime = $modification_time
        }

        Write-Host "  Saved: $output_file_path"
        Write-Host ""
    }
}

Write-Host "Done."
