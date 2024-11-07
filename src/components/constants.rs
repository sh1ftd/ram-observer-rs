pub const LOG_CAPACITY: usize = 100;

// Cooldown timings
pub const NAV_COOLDOWN_MS: u128 = 150;
pub const ACTION_COOLDOWN_MS: u128 = 1000;
pub const AUTO_EXECUTION_COOLDOWN_SECS: u64 = 300;

// RAM thresholds
pub const DEFAULT_AUTO_THRESHOLD: f32 = 90.0;
pub const CRITICAL_THRESHOLD: f32 = 90.0;
pub const WARNING_THRESHOLD: f32 = 75.0;

// Tickrates
pub const ACTIVE_TICK_RATE_MS: u64 = 25;
pub const IDLE_TICK_RATE_MS: u64 = 3000;
pub const IDLE_THRESHOLD_MS: u128 = 30000;