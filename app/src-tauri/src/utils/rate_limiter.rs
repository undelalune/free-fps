// Rate limiting utilities for Tauri commands
use std::sync::Arc;
use tokio::sync::Semaphore;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limiter using semaphore to limit concurrent operations
#[derive(Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    /// Acquire a permit, blocking if none available
    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.semaphore.acquire().await.expect("Semaphore closed")
    }

    /// Try to acquire a permit without blocking
    #[allow(dead_code)]
    pub fn try_acquire(&self) -> Option<tokio::sync::SemaphorePermit<'_>> {
        self.semaphore.try_acquire().ok()
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(1) // Default: Only one operation at a time
    }
}

#[allow(dead_code)]
/// Time-based rate limiter to prevent rapid successive calls
pub struct TimeBasedRateLimiter {
    last_call: Arc<Mutex<Option<Instant>>>,
    min_interval: Duration,
}
#[allow(dead_code)]

impl TimeBasedRateLimiter {
    pub fn new(min_interval: Duration) -> Self {
        Self {
            last_call: Arc::new(Mutex::new(None)),
            min_interval,
        }
    }

    /// Check if enough time has passed since last call
    pub async fn check_and_update(&self) -> bool {
        let mut last_call = self.last_call.lock().await;
        let now = Instant::now();

        if let Some(last) = *last_call {
            if now.duration_since(last) < self.min_interval {
                return false; // Too soon
            }
        }

        *last_call = Some(now);
        true
    }

    /// Reset the rate limiter
    pub async fn reset(&self) {
        *self.last_call.lock().await = None;
    }
}

