#!/usr/bin/env bash
# File: scripts/unix/convert_fps.sh
# Usage: see README.md; run `chmod +x scripts/unix/convert_fps.sh` to make executable

set -euo pipefail

# defaults
DIR="."
FPS=25
KEEP_AUDIO=false
CRF=18
FFMPEG_PATH="/Volumes/Work/dev/ffmpeg"
FFPROBE_PATH="/Volumes/Work/dev/ffprobe"
OUTPUT_FOLDER=""
AUDIO_BITRATE=192
USE_BITRATE=true
USE_GPU=true
GPU_TYPE="auto"

print_usage() {
  cat <<'USAGE'
Usage: convert_fps.sh [-d dir] [-f fps] [-k] [-c crf] [-p ffmpeg_path] [-P ffprobe_path] [-o output_folder] [-b audio_bitrate] [-u] [-g] [-G gpu_type]

  -d DIR            Input directory (default: .)
  -f FPS            Target FPS (default: 25)
  -k                Keep audio (default: remove audio)
  -c CRF            CRF for libx264 (default: 18), ignored if -u is used
  -p FFMPEG_PATH    ffmpeg binary path (default: ffmpeg on PATH)
  -P FFPROBE_PATH   ffprobe binary path (default: ffprobe on PATH)
  -o OUTPUT_FOLDER  Output folder name inside input dir (default: converted_fps_<FPS>)
  -b AUDIO_BITRATE  Audio bitrate in kbps (default: 192)
  -u                Use bitrate mode (compute target bitrate and use -b:v instead of -crf)
  -g                Disable GPU acceleration (default: enabled)
  -G GPU_TYPE       Force GPU type: nvidia, amd, intel, cpu (default: auto-detect)
USAGE
}

# parse options
while getopts ":d:f:kc:p:P:o:b:ugG:h" opt; do
  case ${opt} in
    d) DIR="${OPTARG}" ;;
    f) FPS="${OPTARG}" ;;
    k) KEEP_AUDIO=true ;;
    c) CRF="${OPTARG}" ;;
    p) FFMPEG_PATH="${OPTARG}" ;;
    P) FFPROBE_PATH="${OPTARG}" ;;
    o) OUTPUT_FOLDER="${OPTARG}" ;;
    b) AUDIO_BITRATE="${OPTARG}" ;;
    u) USE_BITRATE=true ;;
    g) USE_GPU=false ;;
    G) GPU_TYPE="${OPTARG}" ;;
    h) print_usage; exit 0 ;;
    \?) echo "Invalid option: -${OPTARG}" >&2; print_usage; exit 1 ;;
    :) echo "Option -${OPTARG} requires an argument." >&2; exit 1 ;;
  esac
done

# check ffmpeg
if ! command -v "${FFMPEG_PATH}" >/dev/null 2>&1 && [ ! -x "${FFMPEG_PATH}" ]; then
  echo "ffmpeg not found at: ${FFMPEG_PATH}" >&2
  exit 1
fi

# check ffprobe
if ! command -v "${FFPROBE_PATH}" >/dev/null 2>&1 && [ ! -x "${FFPROBE_PATH}" ]; then
  echo "ffprobe not found at: ${FFPROBE_PATH}" >&2
  exit 1
fi

# ========== GPU DETECTION ==========
# Returns: TYPE:ENCODER:QUALITY_PARAM:PRESET_PARAM:PRESET
detect_gpu_encoder() {
  local encoders
  encoders="$("${FFMPEG_PATH}" -hide_banner -encoders 2>&1)"

  # Check NVIDIA first (highest priority)
  if echo "${encoders}" | grep -q "h264_nvenc"; then
    if "${FFMPEG_PATH}" -f lavfi -i "color=black:s=320x240:d=1" -c:v h264_nvenc -f null - >/dev/null 2>&1; then
      echo "nvidia:h264_nvenc:-cq:-preset:p4"
      return
    fi
  fi

  # Check AMD
  if echo "${encoders}" | grep -q "h264_amf"; then
    if "${FFMPEG_PATH}" -f lavfi -i "color=black:s=320x240:d=1" -c:v h264_amf -f null - >/dev/null 2>&1; then
      echo "amd:h264_amf:-qp_i:-quality:balanced"
      return
    fi
  fi

  # Check Intel QuickSync
  if echo "${encoders}" | grep -q "h264_qsv"; then
    if "${FFMPEG_PATH}" -f lavfi -i "color=black:s=320x240:d=1" -c:v h264_qsv -f null - >/dev/null 2>&1; then
      echo "intel:h264_qsv:-global_quality:-preset:medium"
      return
    fi
  fi

  # Fallback to CPU
  echo "cpu:libx264:-crf:-preset:slow"
}

# portable filesize
filesize() {
  local file="$1"
  if size="$(stat -c%s "$file" 2>/dev/null)"; then
    printf "%s" "$size"
  else
    stat -f%z "$file"
  fi
}

INPUT_DIR="$(realpath "${DIR}")"
if [ -z "${OUTPUT_FOLDER}" ]; then
  OUTPUT_DIR="${INPUT_DIR}/converted_fps_${FPS}"
else
  OUTPUT_DIR="${INPUT_DIR}/${OUTPUT_FOLDER}"
fi
mkdir -p "${OUTPUT_DIR}"

echo "ffmpeg: ${FFMPEG_PATH}"
echo "ffprobe: ${FFPROBE_PATH}"
echo "Input folder: ${INPUT_DIR}"
echo "Output folder: ${OUTPUT_DIR}"
echo "Target FPS: ${FPS}"
echo "Keep audio: ${KEEP_AUDIO}"
echo "CRF: ${CRF}"
echo "Use bitrate: ${USE_BITRATE}"

# ========== GPU DETECTION ==========
GPU_INFO=""
if [ "${USE_GPU}" = true ]; then
  if [ "${GPU_TYPE}" = "auto" ]; then
    echo "Detecting GPU..."
    GPU_INFO="$(detect_gpu_encoder)"
  else
    case "${GPU_TYPE}" in
      nvidia) GPU_INFO="nvidia:h264_nvenc:-cq:-preset:p4" ;;
      amd)    GPU_INFO="amd:h264_amf:-qp_i:-quality:balanced" ;;
      intel)  GPU_INFO="intel:h264_qsv:-global_quality:-preset:medium" ;;
      *)      GPU_INFO="cpu:libx264:-crf:-preset:slow" ;;
    esac
  fi
else
  GPU_INFO="cpu:libx264:-crf:-preset:slow"
fi

# Parse GPU info
IFS=':' read -r GPU_ENCODER_TYPE GPU_ENCODER GPU_QUALITY_PARAM GPU_PRESET_PARAM GPU_PRESET <<< "${GPU_INFO}"
echo "GPU Encoder: ${GPU_ENCODER_TYPE} (${GPU_ENCODER})"
echo ""

exts=(mp4 MP4 mov MOV mkv MKV avi AVI webm WEBM)

for ext in "${exts[@]}"; do
  shopt -s nullglob 2>/dev/null || true
  for src in "${INPUT_DIR}"/*."${ext}"; do
    [ -f "${src}" ] || continue
    filename="$(basename -- "${src}")"
    base="${filename%.*}"
    ext_lc="$(printf "%s" "${ext}" | tr '[:upper:]' '[:lower:]')"
    out="${OUTPUT_DIR}/${base}_${FPS}fps.${ext_lc}"

    # modification time fallback (UTC ISO 8601)
    if date -u -r "${src}" "+%Y-%m-%dT%H:%M:%S.%3NZ" >/dev/null 2>&1; then
      file_mtime_iso="$(date -u -r "${src}" "+%Y-%m-%dT%H:%M:%S.%3NZ")"
    else
      # macOS stat fallback
      file_mtime_iso="$(stat -f "%Sm" -t "%Y-%m-%dT%H:%M:%S.000Z" "${src}")"
    fi

    # extract creation_time via ffprobe (value only)
    meta_creation_time="$("${FFPROBE_PATH}" -v quiet -print_format default=nk=1:nw=1 -show_entries format_tags=creation_time -i "${src}" 2>/dev/null || true)"
    if [ -z "${meta_creation_time}" ]; then
      meta_creation_time="${file_mtime_iso}"
    fi

    echo "----------------------------------------"
    echo "Processing: ${filename}"
    echo "  Metadata creation_time: ${meta_creation_time} (fallback mtime: ${file_mtime_iso})"

    # detect source fps
    src_fps_line="$("${FFMPEG_PATH}" -hide_banner -i "${src}" 2>&1 | grep -m1 -E 'Stream.*Video' || true)"
    if [ -z "${src_fps_line}" ]; then
      echo "  Warning: Unable to detect FPS. Skipping." >&2
      continue
    fi
    src_fps="$(printf "%s\n" "${src_fps_line}" | sed -nE 's/.* ([0-9]+(\.[0-9]+)?) fps.*/\1/p' || true)"
    if [ -z "${src_fps}" ]; then
      echo "  Warning: FPS value not found. Skipping." >&2
      continue
    fi

    # compute setpts and atempo
    setpts="$(awk -v s="${src_fps}" -v t="${FPS}" 'BEGIN{printf "%.5f", s / t}')"
    atempo="$(awk -v s="${src_fps}" -v t="${FPS}" 'BEGIN{printf "%.5f", t / s}')"
    echo "  FPS: ${src_fps} → setpts: ${setpts} / atempo: ${atempo}"

    # compute bitrate mode args if requested
    videoArgs=""
    if [ "${USE_BITRATE}" = true ]; then
      # strictly match the real "Duration:" header (case‑sensitive, anchored)
      dur_line="$("${FFMPEG_PATH}" -hide_banner -i "${src}" 2>&1 | grep -m1 -E '^[[:space:]]*Duration:[[:space:]]*[0-9]+:[0-9]+:[0-9]+(\.[0-9]+)?' || true)"
      if [ -n "${dur_line}" ]; then
        read hours minutes seconds <<< "$(printf "%s\n" "${dur_line}" | sed -nE 's/.*Duration:[[:space:]]*([0-9]+):([0-9]+):([0-9]+(\.[0-9]+)?).*/\1 \2 \3/p')"
        if [ -n "${hours}" ]; then
          original_duration="$(awk -v h="${hours}" -v m="${minutes}" -v s="${seconds}" 'BEGIN{printf "%.6f", h*3600 + m*60 + s}')"
          if awk -v od="${original_duration}" 'BEGIN{exit (od<=0)}'; then
            new_duration="$(awk -v od="${original_duration}" -v sf="${src_fps}" -v tf="${FPS}" 'BEGIN{printf "%.6f", od * (sf / tf)}')"
            if awk -v nd="${new_duration}" 'BEGIN{exit (nd<=0)}'; then
              USE_BITRATE=false
            else
              original_size="$(filesize "${src}")" || original_size=0
              target_bitrate_kbps="$(awk -v sz="${original_size}" -v nd="${new_duration}" 'BEGIN{ if (nd>0) printf "%.0f", ((sz*8)/nd)/1000; else print 1 }')"
              [ -z "${target_bitrate_kbps}" ] && target_bitrate_kbps=1
              echo "  Computed target bitrate: ${target_bitrate_kbps}k (new duration: ${new_duration} s)"
              videoArgs="-b:v ${target_bitrate_kbps}k -c:v ${GPU_ENCODER} ${GPU_PRESET_PARAM} ${GPU_PRESET} -pix_fmt yuv420p"
            fi
          else
            USE_BITRATE=false
          fi
        else
          USE_BITRATE=false
        fi
      else
        USE_BITRATE=false
      fi
    fi

    if [ -z "${videoArgs}" ]; then
      if [ "${GPU_ENCODER_TYPE}" = "amd" ]; then
        # AMD uses -qp_i, -qp_p, -qp_b
        videoArgs="-c:v ${GPU_ENCODER} -qp_i ${CRF} -qp_p ${CRF} -qp_b ${CRF} ${GPU_PRESET_PARAM} ${GPU_PRESET} -pix_fmt yuv420p"
      else
        videoArgs="-c:v ${GPU_ENCODER} ${GPU_QUALITY_PARAM} ${CRF} ${GPU_PRESET_PARAM} ${GPU_PRESET} -pix_fmt yuv420p"
      fi
    fi

    if [ "${KEEP_AUDIO}" = true ]; then
      cmp_ge() { awk -v a="$1" -v b="$2" 'BEGIN{exit !(a>=b)}'; }
      cmp_le() { awk -v a="$1" -v b="$2" 'BEGIN{exit !(a<=b)}'; }
      if cmp_ge "${atempo}" 0.5 && cmp_le "${atempo}" 2.0; then
        audioArgs="-c:a aac -b:a ${AUDIO_BITRATE}k -af \"atempo=${atempo}\""
      else
        remaining="${atempo}"
        filters=()
        awk_remaining=$(awk -v r="${remaining}" 'BEGIN{printf "%.10f", r}')
        remaining="${awk_remaining}"
        while awk -v r="${remaining}" 'BEGIN{exit !(r > 2.0)}'; do
          filters+=("atempo=2.0")
          remaining="$(awk -v r="${remaining}" 'BEGIN{printf "%.10f", r/2.0}')"
        done
        while awk -v r="${remaining}" 'BEGIN{exit !(r < 0.5)}'; do
          filters+=("atempo=0.5")
          remaining="$(awk -v r="${remaining}" 'BEGIN{printf "%.10f", r*2.0}')"
        done
        remaining="$(awk -v r="${remaining}" 'BEGIN{printf "%.3f", r}')"
        filters+=("atempo=${remaining}")
        audioFilter="$(IFS=,; echo "${filters[*]}")"
        audioArgs="-c:a aac -b:a ${AUDIO_BITRATE}k -af \"${audioFilter}\""
      fi
    else
      audioArgs="-an"
    fi

    metadataArg="-metadata creation_time=\"${meta_creation_time}\""

    cmd="${FFMPEG_PATH} -y -i \"${src}\" -vf \"setpts=${setpts}*PTS\" -r ${FPS} ${videoArgs} ${audioArgs} ${metadataArg} \"${out}\""
    echo "  Running command:"
    echo "  ${cmd}"
    eval "${cmd}"

    # set filesystem timestamp (mtime) to metadata creation time
    iso="${meta_creation_time}"
    iso="${iso%Z}"
    iso_no_frac="${iso%%.*}"
    date_part="${iso_no_frac%T*}"
    time_part="${iso_no_frac#*T}"
    year="${date_part%%-*}"
    mrest="${date_part#*-}"
    month="${mrest%%-*}"
    day="${mrest##*-}"
    hour="${time_part%%:*}"
    trest="${time_part#*:}"
    minute="${trest%%:*}"
    second="${trest##*:}"

    if [ -n "${year}" ] && [ -n "${second}" ]; then
      if [ "$(uname)" = "Darwin" ]; then
        # touch -t [[CC]YY]MMDDhhmm[.SS]
        touch_ts="${year}${month}${day}${hour}${minute}.${second}"
        touch -t "${touch_ts}" "${out}" || true
      else
        touch -d "${year}-${month}-${day} ${hour}:${minute}:${second}" "${out}" || true
      fi
    fi

    echo "  Saved: ${out}"
    echo ""
  done
done

echo "Done."
