use crate::errors::{AppError, AppErrorCode};
use crate::utils::logger::{log_error, log_ffmpeg_command, rotate_log_if_needed};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Deserialize;
use std::process::Stdio;
use tokio::{fs, io::AsyncBufReadExt, process::Command};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Deserialize)]
struct FfprobeJson {
    streams: Option<Vec<ProbeStream>>,
    format: Option<ProbeFormat>,
}

#[derive(Debug, Deserialize)]
struct ProbeStream {
    avg_frame_rate: Option<String>,
    r_frame_rate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProbeFormat {
    duration: Option<String>,
    tags: Option<ProbeTags>,
}

#[derive(Debug, Deserialize)]
struct ProbeTags {
    creation_time: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VideoProbe {
    pub fps: f64,
    pub duration_sec: f64,
    pub creation_time: Option<String>,
}

fn parse_rational(r: &str) -> Option<f64> {
    if let Some((n, d)) = r.split_once('/') {
        let n: f64 = n.trim().parse().ok()?;
        let d: f64 = d.trim().parse().ok()?;
        if d > 0.0 {
            return Some(n / d);
        }
    }
    r.trim().parse::<f64>().ok()
}

async fn probe_with_ffprobe(ffprobe_bin: &str, input: &str) -> Result<VideoProbe, String> {
    let mut cmd = Command::new(ffprobe_bin);
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let output = cmd
        .args(&[
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_entries",
            "stream=avg_frame_rate,r_frame_rate:format=duration:format_tags=creation_time",
            "-select_streams",
            "v:0",
            "-i",
            input,
        ])
        .output()
        .await
        .map_err(|e| format!("ffprobe spawn failed: {e}"))?;

    if !output.status.success() {
        return Err("ffprobe failed".to_string());
    }

    let json: FfprobeJson =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("ffprobe parse failed: {e}"))?;

    let fps = json
        .streams
        .as_ref()
        .and_then(|s| s.get(0))
        .and_then(|s| s.avg_frame_rate.as_deref().or(s.r_frame_rate.as_deref()))
        .and_then(parse_rational)
        .ok_or_else(|| "ffprobe: FPS not found".to_string())?;

    let duration_sec: f64 = json
        .format
        .as_ref()
        .and_then(|f| f.duration.as_deref())
        .and_then(|s| s.parse::<f64>().ok())
        .ok_or_else(|| "ffprobe: duration not found".to_string())?;

    let creation_time = json
        .format
        .and_then(|f| f.tags)
        .and_then(|t| t.creation_time);

    Ok(VideoProbe {
        fps,
        duration_sec,
        creation_time,
    })
}

async fn probe_with_ffmpeg(ffmpeg_bin: &str, input: &str) -> Result<VideoProbe, String> {
    let mut cmd = Command::new(ffmpeg_bin);
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let output = cmd
        .args(&["-i", input, "-hide_banner"])
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .output()
        .await
        .map_err(|e| format!("ffmpeg probe spawn failed: {e}"))?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    let fps_re = Regex::new(r"(?m)Stream.*Video.*?(\d+(?:\.\d+)?)\s*fps").unwrap();
    let fps = fps_re
        .captures(&stderr)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .ok_or_else(|| "ffmpeg probe: FPS not found".to_string())?;

    let dur_re = Regex::new(r"(?m)^\s*Duration:\s*(\d+):(\d+):(\d+(?:\.\d+)?)").unwrap();
    let duration_sec = if let Some(c) = dur_re.captures(&stderr) {
        let h: f64 = c.get(1).unwrap().as_str().parse().unwrap_or(0.0);
        let m: f64 = c.get(2).unwrap().as_str().parse().unwrap_or(0.0);
        let s: f64 = c.get(3).unwrap().as_str().parse().unwrap_or(0.0);
        h * 3600.0 + m * 60.0 + s
    } else {
        return Err("ffmpeg probe: duration not found".to_string());
    };

    Ok(VideoProbe {
        fps,
        duration_sec,
        creation_time: None,
    })
}

pub async fn probe_video(
    ffprobe_bin: Option<&str>,
    ffmpeg_bin: &str,
    input: &str,
) -> Result<VideoProbe, String> {
    if let Some(bin) = ffprobe_bin {
        if let Ok(p) = probe_with_ffprobe(bin, input).await {
            return Ok(p);
        }
    }
    probe_with_ffmpeg(ffmpeg_bin, input).await
}

fn threads_from_cpu_limit(cpu_limit: Option<u8>) -> usize {
    cpu_limit
        .map(|p| {
            (num_cpus::get() as f32 * (p as f32 / 100.0))
                .ceil()
                .max(1.0) as usize
        })
        .filter(|t| *t > 0)
        .unwrap_or_else(|| num_cpus::get())
}

fn build_atempo_chain(atempo: f64) -> String {
    let mut filters = Vec::new();
    let mut remaining = atempo;

    while remaining > 2.0 {
        filters.push("atempo=2.0".to_string());
        remaining /= 2.0;
    }
    while remaining < 0.5 {
        filters.push("atempo=0.5".to_string());
        remaining *= 2.0;
    }
    filters.push(format!("atempo={:.3}", remaining));
    filters.join(",")
}

fn parse_progress_time(key: &str, val: &str) -> Option<f64> {
    if let Ok(n) = val.parse::<u64>() {
        return Some(match key {
            "out_time_us" => n as f64 / 1_000_000.0,
            "out_time_ms" => n as f64 / 1_000.0,
            _ => n as f64,
        });
    }
    let parts: Vec<&str> = val.split(':').collect();
    if parts.len() == 3 {
        let h: f64 = parts[0].parse().ok()?;
        let m: f64 = parts[1].parse().ok()?;
        let s: f64 = parts[2]
            .parse()
            .ok()
            .or_else(|| parts[2].split('.').next()?.parse().ok())?;
        return Some(h * 3600.0 + m * 60.0 + s);
    }
    None
}

fn system_time_to_rfc3339_z(t: std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = t.into();
    dt.to_rfc3339()
}

fn quote_if_needed(s: &str) -> String {
    if s.contains(' ') || s.contains('"') {
        format!(r#""{}""#, s.replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

pub struct ConvertOptions<'a> {
    pub ffmpeg_bin: &'a str,
    pub ffprobe_bin: Option<&'a str>,
    pub input: &'a str,
    pub output: &'a str,
    pub target_fps: f32,
    pub keep_audio: bool,
    pub audio_bitrate: u32,
    pub use_custom_video_quality: bool,
    pub video_quality: u8, // CRF 0..51
    pub cpu_limit: Option<u8>,
}

// Internal implementation with coded errors.
async fn convert_video_with_progress_impl<F>(
    opts: ConvertOptions<'_>,
    mut on_progress: F,
    cancel: CancellationToken,
) -> Result<Option<String>, AppError>
where
    F: FnMut(f32) + Send + 'static,
{
    // ensure log rotation is checked
    rotate_log_if_needed().await;

    // Probe (map to ffprobe/ffmpeg related codes).
    let probe = match probe_video(opts.ffprobe_bin, opts.ffmpeg_bin, opts.input).await {
        Ok(p) => p,
        Err(e) => {
            let code = if opts.ffprobe_bin.is_some() {
                AppErrorCode::FfprobeFailed
            } else {
                AppErrorCode::FfmpegFailed
            };
            let ctx = format!("probe failed for input {}: {}", opts.input, e);
            let _ = log_error("ProbeFailed", &ctx).await;
            return Err(AppError::new(code, e));
        }
    };

    // Compute timings
    let src_fps = probe.fps;
    let tfps = opts.target_fps as f64;
    if tfps <= 0.0 || src_fps <= 0.0 {
        let _ = log_error("InvalidFps", &format!("src_fps={} tfps={}", src_fps, tfps)).await;
        return Err(AppError::code_only(AppErrorCode::InvalidFps));
    }
    let setpts = (src_fps / tfps).max(0.00001);
    let atempo = (tfps / src_fps).max(0.00001);

    // Use original duration for time-based progress
    let progress_total_secs = probe.duration_sec.max(0.000001);

    // Estimate total frames (works well even with setpts + -r)
    let total_frames_est = (probe.duration_sec * src_fps).round().max(1.0) as u64;

    // Video args
    let new_duration = probe.duration_sec * (src_fps / tfps);
    let mut video_args: Vec<String> = Vec::new();
    if opts.use_custom_video_quality {
        // Validate CRF range 0..=51
        if opts.video_quality > 51 {
            let _ = log_error(
                "VideoQualityOutOfRange",
                &format!("quality={}", opts.video_quality),
            )
            .await;
            return Err(AppError::code_only(AppErrorCode::VideoQualityOutOfRange));
        }
        video_args.extend([
            "-c:v".into(),
            "libx264".into(),
            "-crf".into(),
            opts.video_quality.to_string(),
            "-preset".into(),
            "slow".into(),
            "-pix_fmt".into(),
            "yuv420p".into(),
        ]);
    } else {
        // Ensure input metadata is available and file is not empty
        let meta = fs::metadata(opts.input)
            .await
            .map_err(|e| AppError::new(AppErrorCode::ReadMetadataFailed, e.to_string()))?;
        let size_bytes = meta.len() as f64;
        if size_bytes <= 0.0 {
            let _ = log_error(
                "EmptyInputFile",
                &format!("input={} size={}", opts.input, size_bytes),
            )
            .await;
            return Err(AppError::code_only(AppErrorCode::EmptyInputFile));
        }

        if new_duration <= 0.0 {
            let _ = log_error(
                "InvalidNewDuration",
                &format!("input={} new_duration={}", opts.input, new_duration),
            )
            .await;
            return Err(AppError::code_only(AppErrorCode::InvalidNewDuration));
        }

        let target_kbps = ((size_bytes * 8.0) / new_duration / 1000.0)
            .round()
            .max(1.0) as u64;
        video_args.extend([
            "-b:v".into(),
            format!("{}k", target_kbps),
            "-c:v".into(),
            "libx264".into(),
            "-preset".into(),
            "slow".into(),
            "-pix_fmt".into(),
            "yuv420p".into(),
        ]);
    }

    // Validate audio bitrate when audio is kept
    if opts.keep_audio && opts.audio_bitrate == 0 {
        let _ = log_error("AudioBitrateInvalid", &format!("input={}", opts.input)).await;
        return Err(AppError::code_only(AppErrorCode::AudioBitrateInvalid));
    }
    // Audio args
    let mut audio_args: Vec<String> = Vec::new();
    if opts.keep_audio {
        let chain = build_atempo_chain(atempo);
        audio_args.extend([
            "-c:a".into(),
            "aac".into(),
            "-b:a".into(),
            format!("{}k", opts.audio_bitrate),
            "-af".into(),
            chain,
        ]);
    } else {
        audio_args.push("-an".into());
    }

    // Threads
    let threads_arg = threads_from_cpu_limit(opts.cpu_limit);

    // Creation time for metadata
    let meta_creation_time = if let Some(ct) = &probe.creation_time {
        Some(ct.clone())
    } else {
        fs::metadata(opts.input)
            .await
            .ok()
            .and_then(|m| m.modified().ok())
            .map(system_time_to_rfc3339_z)
    };

    // Build command preview for logging (conservative).
    let mut parts: Vec<String> = Vec::new();
    parts.push(opts.ffmpeg_bin.to_string());
    parts.push("-y".to_string());
    parts.push("-i".to_string());
    parts.push(quote_if_needed(opts.input));
    parts.push("-vf".to_string());
    parts.push(format!("setpts={:.5}*PTS", setpts));
    parts.push("-r".to_string());
    parts.push(opts.target_fps.to_string());
    parts.extend(video_args.clone());
    parts.extend(audio_args.clone());
    parts.push("-threads".to_string());
    parts.push(threads_arg.to_string());
    if let Some(ct) = &meta_creation_time {
        parts.push("-metadata".to_string());
        parts.push(format!(r#"creation_time={}"#, ct));
    }
    parts.push("-progress".to_string());
    parts.push("pipe:1".to_string());
    parts.push("-nostats".to_string());
    parts.push(quote_if_needed(opts.output));
    let cmd_preview = parts.join(" ");
    // Log ffmpeg command
    let _ = log_ffmpeg_command(&cmd_preview).await;

    // Build actual Command
    let mut cmd = Command::new(opts.ffmpeg_bin);
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd.arg("-y")
        .arg("-i")
        .arg(opts.input)
        .arg("-vf")
        .arg(format!("setpts={:.5}*PTS", setpts))
        .arg("-r")
        .arg(format!("{}", opts.target_fps))
        .args(video_args)
        .args(audio_args)
        .arg("-threads")
        .arg(threads_arg.to_string());

    if let Some(ct) = &meta_creation_time {
        cmd.arg("-metadata").arg(format!(r#"creation_time={}"#, ct));
    }

    cmd.arg("-progress")
        .arg("pipe:1")
        .arg("-nostats")
        .arg(opts.output)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            let emsg = format!("ffmpeg spawn failed: {}", e);
            let _ = log_error("FfmpegSpawnFailed", &emsg).await;
            return Err(AppError::new(
                AppErrorCode::FfmpegSpawnFailed,
                e.to_string(),
            ));
        }
    };

    let mut stdout = tokio::io::BufReader::new(child.stdout.take().unwrap()).lines();

    on_progress(0.0);

    // Trackers
    let mut last_pct = 0.0_f32;
    let mut last_frame: Option<u64> = None;
    let mut last_secs: Option<f64> = None;

    // Helper to compute conservative progress from available signals
    let emit_progress =
        |on_progress: &mut F, last_pct: &mut f32, frame: Option<u64>, secs: Option<f64>| {
            let mut candidates: Vec<f64> = Vec::new();
            if let Some(fr) = frame {
                let pf = (fr as f64 / total_frames_est as f64).clamp(0.0, 0.999);
                candidates.push(pf);
            }
            if let Some(s) = secs {
                let pt = (s / progress_total_secs).clamp(0.0, 0.999);
                candidates.push(pt);
            }

            if candidates.is_empty() {
                return;
            }

            // Be conservative: take the minimum to prevent early 100%
            let p01 = candidates.into_iter().fold(1.0, f64::min) as f32;
            let pct = (p01 * 100.0).max(*last_pct); // monotonic

            // Emit only on increase to reduce noise
            if pct > *last_pct {
                *last_pct = pct;
                on_progress(pct);
            }
        };

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                let _ = child.kill().await;
                return Err(AppError::code_only(AppErrorCode::Cancelled));
            }
            line = stdout.next_line() => {
                match line {
                    Ok(Some(l)) => {
                        if let Some((k,v)) = l.split_once('=') {
                            match k {
                                "frame" => {
                                    if let Ok(fr) = v.trim().parse::<u64>() {
                                        last_frame = Some(fr);
                                        emit_progress(&mut on_progress, &mut last_pct, last_frame, last_secs);
                                    }
                                }
                                "out_time_ms" | "out_time_us" | "out_time" => {
                                    if let Some(secs) = parse_progress_time(k, v) {
                                        last_secs = Some(secs);
                                        emit_progress(&mut on_progress, &mut last_pct, last_frame, last_secs);
                                    }
                                }
                                "progress" if v == "end" => {
                                    on_progress(100.0);
                                }
                                _ => {}
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        let _ = child.kill().await;
                        let _ = log_error("FfmpegReadFailed", &format!("ffmpeg read failed: {e}")).await;
                        return Err(AppError::new(AppErrorCode::Io, format!("ffmpeg read failed: {e}")));
                    }
                }
            }
        }
    }

    let status = child
        .wait()
        .await
        .map_err(|e| AppError::new(AppErrorCode::Io, format!("ffmpeg wait failed: {e}")))?;
    if status.success() {
        on_progress(100.0);
        Ok(meta_creation_time)
    } else {
        let emsg = format!(
            "ffmpeg failed with code {:?} (cmd: {})",
            status.code(),
            cmd_preview
        );
        let _ = log_error("FfmpegFailed", &emsg).await;
        Err(AppError::new(
            AppErrorCode::FfmpegFailed,
            format!("ffmpeg failed with code {:?}", status.code()),
        ))
    }
}

// Adapter that preserves the original String error API.
// - "Cancelled" is returned verbatim for upstream logic.
// - Other errors are returned as the numeric error code string (u16).
pub async fn convert_video_with_progress<F>(
    opts: ConvertOptions<'_>,
    on_progress: F,
    cancel: CancellationToken,
) -> Result<Option<String>, String>
where
    F: FnMut(f32) + Send + 'static,
{
    match convert_video_with_progress_impl(opts, on_progress, cancel).await {
        Ok(v) => Ok(v),
        Err(e) => {
            if let AppErrorCode::Cancelled = e.code {
                Err("Cancelled".to_string())
            } else {
                Err((e.code as u16).to_string())
            }
        }
    }
}
