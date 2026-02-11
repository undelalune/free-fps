// Free FPS - Video Frame Rate Converter
// Copyright (C) 2025 undelalune
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::errors::{AppError, AppErrorCode};
use crate::utils::logger::{log_error, log_ffmpeg_command, rotate_log_if_needed};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Deserialize;
use std::process::Stdio;
use tokio::{
    fs,
    io::AsyncBufReadExt,
    process::Command,
    time::{timeout, Duration},
};
use tokio_util::sync::CancellationToken;

const DEFAULT_CONVERSION_TIMEOUT_SECS: u64 = 10800; // 3 hours

// ===== ffprobe parsing =====

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

// ===== Utilities =====

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

#[cfg(windows)]
fn apply_no_window(cmd: &mut Command) {
    use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
    cmd.creation_flags(CREATE_NO_WINDOW);
}
#[cfg(not(windows))]
fn apply_no_window(_: &mut Command) {}

// ===== Probe via ffprobe/ffmpeg =====

async fn probe_with_ffprobe(ffprobe_bin: &str, input: &str) -> Result<VideoProbe, String> {
    let mut cmd = Command::new(ffprobe_bin);
    apply_no_window(&mut cmd);

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
    apply_no_window(&mut cmd);

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

// ===== Conversion helpers =====

fn threads_from_cpu_limit(cpu_limit: Option<u8>) -> usize {
    let max_cpus = num_cpus::get();
    match cpu_limit {
        Some(pct) => {
            let pct = pct.clamp(1, 100) as f32 / 100.0;
            let n = ((max_cpus as f32) * pct).ceil() as usize;
            n.max(1).min(max_cpus)
        }
        None => max_cpus,
    }
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

struct Timings {
    setpts: f64,
    atempo: f64,
    new_duration: f64,
    progress_total_secs: f64,
    total_frames_est: u64,
}

async fn compute_timings(probe: &VideoProbe, target_fps: f32) -> Result<Timings, AppError> {
    let src_fps = probe.fps;
    let tfps = target_fps as f64;

    if tfps <= 0.0 || src_fps <= 0.0 {
        let _ = log_error("InvalidFps", &format!("src_fps={} tfps={}", src_fps, tfps)).await;
        return Err(AppError::code_only(AppErrorCode::InvalidFps));
    }

    let setpts = (src_fps / tfps).max(0.00001);
    let atempo = (tfps / src_fps).max(0.00001);

    let progress_total_secs = probe.duration_sec.max(0.000001);
    let total_frames_est = (probe.duration_sec * src_fps).round().max(1.0) as u64;
    let new_duration = probe.duration_sec * (src_fps / tfps);

    Ok(Timings {
        setpts,
        atempo,
        new_duration,
        progress_total_secs,
        total_frames_est,
    })
}

/// Map CPU preset to NVIDIA NVENC preset (p1-p7)
fn map_preset_nvidia(_preset: &str) -> &'static str {
    // For now, use p4 (medium) as default for balanced speed/quality
    // NVENC presets: p1 (fastest) to p7 (slowest/best quality)
    "p4"
}

/// Map CPU preset to AMD AMF quality setting
fn map_preset_amd(_preset: &str) -> &'static str {
    // AMF quality settings: speed, balanced, quality
    "balanced"
}

/// Map CPU preset to Intel QSV preset
fn map_preset_intel(_preset: &str) -> &'static str {
    // QSV presets similar to x264: veryfast, faster, fast, medium, slow, slower, veryslow
    "medium"
}

/// Build video encoding arguments with GPU support
async fn build_video_args(
    input: &str,
    use_custom_quality: bool,
    crf: u8,
    new_duration: f64,
    use_gpu: bool,
    gpu_type: Option<&str>,
) -> Result<Vec<String>, AppError> {
    // Validate CRF if custom quality is used
    if use_custom_quality && crf > 51 {
        let _ = log_error("VideoQualityOutOfRange", &format!("crf={}", crf)).await;
        return Err(AppError::code_only(AppErrorCode::VideoQualityOutOfRange));
    }

    // If GPU encoding is requested
    if use_gpu {
        if let Some(gpu) = gpu_type {
            match gpu.to_lowercase().as_str() {
                "nvidia" => {
                    let mut args = vec![
                        "-c:v".into(),
                        "h264_nvenc".into(),
                    ];

                    if use_custom_quality {
                        // NVENC uses -cq for constant quality mode (similar to CRF)
                        args.extend([
                            "-cq".into(),
                            crf.to_string(),
                            "-preset".into(),
                            map_preset_nvidia("slow").into(),
                        ]);
                    } else {
                        // Auto bitrate mode - calculate target bitrate
                        let target_kbps = calculate_target_bitrate(input, new_duration).await?;
                        args.extend([
                            "-b:v".into(),
                            format!("{}k", target_kbps),
                            "-preset".into(),
                            map_preset_nvidia("slow").into(),
                        ]);
                    }

                    args.extend(["-pix_fmt".into(), "yuv420p".into()]);
                    return Ok(args);
                }

                "amd" => {
                    let mut args = vec![
                        "-c:v".into(),
                        "h264_amf".into(),
                    ];

                    if use_custom_quality {
                        // AMF uses -qp_i, -qp_p, -qp_b for quality control
                        args.extend([
                            "-qp_i".into(),
                            crf.to_string(),
                            "-qp_p".into(),
                            crf.to_string(),
                            "-qp_b".into(),
                            crf.to_string(),
                            "-quality".into(),
                            map_preset_amd("slow").into(),
                        ]);
                    } else {
                        let target_kbps = calculate_target_bitrate(input, new_duration).await?;
                        args.extend([
                            "-b:v".into(),
                            format!("{}k", target_kbps),
                            "-quality".into(),
                            map_preset_amd("slow").into(),
                        ]);
                    }

                    args.extend(["-pix_fmt".into(), "yuv420p".into()]);
                    return Ok(args);
                }

                "intel" => {
                    let mut args = vec![
                        "-c:v".into(),
                        "h264_qsv".into(),
                    ];

                    if use_custom_quality {
                        // QSV uses -global_quality for quality control
                        args.extend([
                            "-global_quality".into(),
                            crf.to_string(),
                            "-preset".into(),
                            map_preset_intel("slow").into(),
                        ]);
                    } else {
                        let target_kbps = calculate_target_bitrate(input, new_duration).await?;
                        args.extend([
                            "-b:v".into(),
                            format!("{}k", target_kbps),
                            "-preset".into(),
                            map_preset_intel("slow").into(),
                        ]);
                    }

                    args.extend(["-pix_fmt".into(), "yuv420p".into()]);
                    return Ok(args);
                }

                _ => {
                    // Unknown GPU type, fall through to CPU encoding
                }
            }
        }
    }

    // CPU encoding (default/fallback)
    if use_custom_quality {
        return Ok(vec![
            "-c:v".into(),
            "libx264".into(),
            "-crf".into(),
            crf.to_string(),
            "-preset".into(),
            "slow".into(),
            "-pix_fmt".into(),
            "yuv420p".into(),
        ]);
    }

    // Derive approximate target bitrate to preserve size
    let target_kbps = calculate_target_bitrate(input, new_duration).await?;

    Ok(vec![
        "-b:v".into(),
        format!("{}k", target_kbps),
        "-c:v".into(),
        "libx264".into(),
        "-preset".into(),
        "slow".into(),
        "-pix_fmt".into(),
        "yuv420p".into(),
    ])
}

/// Calculate target bitrate based on input file size and expected duration
async fn calculate_target_bitrate(input: &str, new_duration: f64) -> Result<u64, AppError> {
    let meta = fs::metadata(input)
        .await
        .map_err(|e| AppError::new(AppErrorCode::ReadMetadataFailed, e.to_string()))?;
    let size_bytes = meta.len() as f64;

    if size_bytes <= 0.0 {
        let _ = log_error(
            "EmptyInputFile",
            &format!("input={} size={}", input, size_bytes),
        )
        .await;
        return Err(AppError::code_only(AppErrorCode::EmptyInputFile));
    }

    if new_duration <= 0.0 {
        let _ = log_error(
            "InvalidNewDuration",
            &format!("input={} new_duration={}", input, new_duration),
        )
        .await;
        return Err(AppError::code_only(AppErrorCode::InvalidNewDuration));
    }

    Ok(((size_bytes * 8.0) / new_duration / 1000.0).round().max(1.0) as u64)
}

async fn build_audio_args(
    keep_audio: bool,
    audio_bitrate: u32,
    atempo: f64,
) -> Result<Vec<String>, AppError> {
    if !keep_audio {
        return Ok(vec!["-an".into()]);
    }
    if audio_bitrate == 0 {
        let _ = log_error("AudioBitrateInvalid", "keep_audio=true with bitrate=0").await;
        return Err(AppError::code_only(AppErrorCode::AudioBitrateInvalid));
    }
    let chain = build_atempo_chain(atempo);
    Ok(vec![
        "-c:a".into(),
        "aac".into(),
        "-b:a".into(),
        format!("{}k", audio_bitrate),
        "-af".into(),
        chain,
    ])
}

async fn creation_time_for_input(probe: &VideoProbe, input: &str) -> Option<String> {
    if let Some(ct) = &probe.creation_time {
        return Some(ct.clone());
    }
    fs::metadata(input)
        .await
        .ok()
        .and_then(|m| m.modified().ok())
        .map(system_time_to_rfc3339_z)
}

fn build_command_preview(
    ffmpeg_bin: &str,
    input: &str,
    output: &str,
    target_fps: f32,
    setpts: f64,
    threads: Option<usize>,
    video_args: &[String],
    audio_args: &[String],
    meta_creation_time: Option<&String>,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    parts.push(ffmpeg_bin.to_string());
    parts.push("-y".to_string());
    parts.push("-i".to_string());
    parts.push(quote_if_needed(input));
    parts.push("-vf".to_string());
    parts.push(format!("setpts={:.5}*PTS", setpts));
    parts.push("-r".to_string());
    parts.push(target_fps.to_string());
    parts.extend(video_args.iter().cloned());
    parts.extend(audio_args.iter().cloned());
    if let Some(t) = threads {
        parts.push("-threads".to_string());
        parts.push(t.to_string());
    }
    if let Some(ct) = meta_creation_time {
        parts.push("-metadata".to_string());
        parts.push(format!(r#"creation_time={}"#, ct));
    }
    parts.push("-progress".to_string());
    parts.push("pipe:1".to_string());
    parts.push("-nostats".to_string());
    parts.push(quote_if_needed(output));
    parts.join(" ")
}

fn build_ffmpeg_command(
    ffmpeg_bin: &str,
    input: &str,
    output: &str,
    target_fps: f32,
    setpts: f64,
    threads: Option<usize>,
    video_args: Vec<String>,
    audio_args: Vec<String>,
    meta_creation_time: Option<String>,
) -> Command {
    let mut cmd = Command::new(ffmpeg_bin);
    apply_no_window(&mut cmd);

    cmd.arg("-y")
        .arg("-i")
        .arg(input)
        .arg("-vf")
        .arg(format!("setpts={:.5}*PTS", setpts))
        .arg("-r")
        .arg(format!("{}", target_fps))
        .args(video_args)
        .args(audio_args);

    if let Some(t) = threads {
        cmd.arg("-threads").arg(t.to_string());
    }

    if let Some(ct) = meta_creation_time {
        cmd.arg("-metadata").arg(format!(r#"creation_time={}"#, ct));
    }

    cmd.arg("-progress")
        .arg("pipe:1")
        .arg("-nostats")
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    cmd
}

struct ProgressTracker {
    last_pct: f32,
    last_frame: Option<u64>,
    last_secs: Option<f64>,
    total_frames_est: u64,
    total_secs: f64,
}

impl ProgressTracker {
    fn new(total_frames_est: u64, total_secs: f64) -> Self {
        Self {
            last_pct: 0.0,
            last_frame: None,
            last_secs: None,
            total_frames_est,
            total_secs,
        }
    }

    fn update_kv(&mut self, key: &str, val: &str) -> Option<f32> {
        match key {
            "frame" => {
                if let Ok(f) = val.parse::<u64>() {
                    self.last_frame = Some(f);
                }
            }
            "out_time_ms" | "out_time_us" | "out_time" => {
                if let Some(s) = parse_progress_time(key, val) {
                    self.last_secs = Some(s);
                }
            }
            _ => {}
        }
        self.emit()
    }

    fn emit(&mut self) -> Option<f32> {
        let mut candidates: Vec<f64> = Vec::new();
        if let Some(fr) = self.last_frame {
            let pf = (fr as f64 / self.total_frames_est as f64).clamp(0.0, 0.999);
            candidates.push(pf);
        }
        if let Some(s) = self.last_secs {
            let pt = (s / self.total_secs).clamp(0.0, 0.999);
            candidates.push(pt);
        }
        if candidates.is_empty() {
            return None;
        }
        let p01 = candidates.into_iter().fold(1.0, f64::min) as f32;
        let pct = (p01 * 100.0).max(self.last_pct);
        if pct > self.last_pct {
            self.last_pct = pct;
            Some(pct)
        } else {
            None
        }
    }
}

// ===== Public API =====

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
    pub use_gpu: bool,
    pub gpu_type: Option<String>,
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
    rotate_log_if_needed().await;

    // Probe
    let probe = match probe_video(opts.ffprobe_bin, opts.ffmpeg_bin, opts.input).await {
        Ok(p) => p,
        Err(e) => {
            let code = if opts.ffprobe_bin.is_some() {
                AppErrorCode::FfprobeFailed
            } else {
                AppErrorCode::FfprobeFailed
            };
            let ctx = format!("probe failed for input {}: {}", opts.input, e);
            let _ = log_error("ProbeFailed", &ctx).await;
            return Err(AppError::new(code, e));
        }
    };

    // Timings
    let timings = compute_timings(&probe, opts.target_fps).await?;

    // Args
    let video_args = build_video_args(
        opts.input,
        opts.use_custom_video_quality,
        opts.video_quality,
        timings.new_duration,
        opts.use_gpu,
        opts.gpu_type.as_deref(),
    )
        .await?;

    let audio_args = build_audio_args(opts.keep_audio, opts.audio_bitrate, timings.atempo).await?;

    let threads_opt = if opts.cpu_limit == Some(100) {
        None
    } else {
        Some(threads_from_cpu_limit(opts.cpu_limit))
    };
    let meta_creation_time = creation_time_for_input(&probe, opts.input).await;

    // Preview + log
    let preview = build_command_preview(
        opts.ffmpeg_bin,
        opts.input,
        opts.output,
        opts.target_fps,
        timings.setpts,
        threads_opt,
        &video_args,
        &audio_args,
        meta_creation_time.as_ref(),
    );
    let _ = log_ffmpeg_command(&preview).await;

    // Command
    let mut cmd = build_ffmpeg_command(
        opts.ffmpeg_bin,
        opts.input,
        opts.output,
        opts.target_fps,
        timings.setpts,
        threads_opt,
        video_args,
        audio_args,
        meta_creation_time.clone(),
    );

    // Spawn
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

    // Progress tracking
    let mut tracker = ProgressTracker::new(timings.total_frames_est, timings.progress_total_secs);

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                let _ = child.kill().await;
                let _ = child.wait().await;
                let _ = log_error("Cancelled", "conversion cancelled by user").await;
                return Err(AppError::code_only(AppErrorCode::Cancelled));
            }
            line = stdout.next_line() => {
                match line {
                    Ok(Some(l)) => {
                        if let Some((k,v)) = l.split_once('=') {
                            if let Some(pct) = tracker.update_kv(k.trim(), v.trim()) {
                                on_progress(pct);
                            }
                            // ffmpeg emits "progress=end"
                            if k.trim() == "progress" && v.trim() == "end" {
                                break;
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        let _ = log_error("ProgressReadFailed", &e.to_string()).await;
                        break;
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
            preview
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
    // Add timeout protection (3 hours default)
    let conversion_future = convert_video_with_progress_impl(opts, on_progress, cancel.clone());
    let timeout_duration = Duration::from_secs(DEFAULT_CONVERSION_TIMEOUT_SECS);

    match timeout(timeout_duration, conversion_future).await {
        Ok(result) => match result {
            Ok(v) => Ok(v),
            Err(e) => {
                if let AppErrorCode::Cancelled = e.code {
                    Err("Cancelled".to_string())
                } else {
                    Err((e.code as u16).to_string())
                }
            }
        },
        Err(_) => {
            let _ = log_error(
                "ConversionTimeout",
                &format!(
                    "Conversion exceeded {} seconds",
                    DEFAULT_CONVERSION_TIMEOUT_SECS
                ),
            )
                .await;
            cancel.cancel(); // Trigger cancellation
            Err((AppErrorCode::FfmpegFailed as u16).to_string())
        }
    }
}