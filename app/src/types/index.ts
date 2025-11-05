export interface VideoFile {
    path: string;
    name: string;
    size: number;
    // duration?: number;
    convert: boolean;
    progress?: number;
    position?: number;
    status?: ConversionStatus;
    thumbnail?: string; //base64 data url
}

export interface VideoConversionParams {
    input_folder: string; //input folder path
    output_folder: string; //output folder path (if empty, use input folder to create output folder inside with the name "converted_videos_${target_fps}fps")
    target_fps: number; //target fps
    cpu_limit: number; //cpu limit percentage (0-100)
    keep_audio: boolean; // if true keep audio in video
    audio_bitrate: number; // output audio bitrate in video (if keep_audio = true)
    use_custom_video_quality: boolean; // if true use custom video quality - video_quality (crf, 0-51, lower is better quality). If false:
    video_quality: number; // output video quality (crf, 0-51, lower is better quality) (if use_custom_video_quality = true)
    files: string[]; //array of file paths to convert
}

export enum LicenseType {
    FFmpegNotice = "FFmpegNotice",
    FFmpegLicense = "FFmpegLicense",
    FreeFPSLicense = "FreeFPSLicense",
}

export enum ErrorCode {
    None = 0,
    Cancelled = 1,
    FolderNotFound = 2,
    NoVideoFiles = 3,
    Io = 4,
    FfmpegNotFound = 10,
    FfprobeNotFound = 11,
    FfmpegSpawnFailed = 12,
    FfprobeFailed = 13,
    FfmpegFailed = 14,
    InvalidFps = 20,
    InvalidNewDuration = 21,
    EmptyInputFile = 22,
    VideoQualityOutOfRange = 23,
    AudioBitrateInvalid = 24,
    ReadMetadataFailed = 25,
    PathTraversalDetected = 26,
    InvalidInputPath = 27,
    LicenseNotFound = 28,
}

export type AppError = { code: ErrorCode; details?: string };

export enum ConversionStatus {
    Processing = "Processing",
    Success = "Success",
    Error = "Error",
    Cancelled = "Cancelled",
}

export interface ConversionProgress {
    current_file: string;
    current_file_index: number;
    total_files: number;
    percentage: number;
    status: ConversionStatus; //pub enum ConversionStatus {Conversion,Success,Error,None,}
}