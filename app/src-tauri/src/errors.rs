use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum AppErrorCode {
    // General
    None = 0,
    Cancelled = 1,
    FolderNotFound = 2,
    NoVideoFiles = 3,
    Io = 4,

    // Tooling
    FfmpegNotFound = 10,
    FfprobeNotFound = 11,
    FfmpegSpawnFailed = 12,
    FfprobeFailed = 13,
    FfmpegFailed = 14,

    // Validation
    InvalidFps = 20,
    InvalidNewDuration = 21,
    EmptyInputFile = 22,
    VideoQualityOutOfRange = 23,
    AudioBitrateInvalid = 24,
    ReadMetadataFailed = 25,
    PathTraversalDetected = 26,
    InvalidInputPath = 27,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppError {
    pub code: AppErrorCode,
    pub details: Option<String>,
}

impl AppError {
    pub fn new(code: AppErrorCode, details: impl Into<String>) -> Self {
        Self {
            code,
            details: Some(details.into()),
        }
    }
    pub fn code_only(code: AppErrorCode) -> Self {
        Self { code, details: None }
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::new(AppErrorCode::Io, e.to_string())
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::new(AppErrorCode::None, s)
    }
}
impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::new(AppErrorCode::None, s)
    }
}
