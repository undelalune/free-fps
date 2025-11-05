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
    LicenseNotFound = 28,
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
        Self {
            code,
            details: None,
        }
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
