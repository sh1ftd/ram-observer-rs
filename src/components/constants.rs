use std::time::Duration;

// Log and UI constants
pub const LOG_CAPACITY: usize = 100;
pub const TICK_RATE: Duration = Duration::from_millis(500);

// Cooldown timings
pub const NAV_COOLDOWN_MS: u128 = 150;
pub const ACTION_COOLDOWN_MS: u128 = 1000;
pub const AUTO_EXECUTION_COOLDOWN_SECS: u64 = 300;

// RAM thresholds
pub const DEFAULT_AUTO_THRESHOLD: f32 = 90.0;
pub const CRITICAL_THRESHOLD: f32 = 90.0;
pub const WARNING_THRESHOLD: f32 = 75.0;
