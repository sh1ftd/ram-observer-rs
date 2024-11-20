use ratatui::style::Color;
use std::time::Duration;

use crate::components::constants::{CRITICAL_THRESHOLD, WARNING_THRESHOLD};

// Format a duration to a human-readable string
pub fn format_timestamp(duration: Duration) -> String {
    // Seconds
    if duration.as_secs() < 60 {
        format!("{:>3}s ago", duration.as_secs())
    }
    // Minutes
    else if duration.as_secs() < 3600 {
        format!("{:>3}m ago", duration.as_secs() / 60)
    }
    // Hours
    else {
        format!("{:>3}h ago", duration.as_secs() / 3600)
    }
}

// Get the color based on the percentage of used RAM
pub fn get_usage_color(percentage: f32) -> Color {
    if percentage >= CRITICAL_THRESHOLD {
        Color::Red
    } else if percentage >= WARNING_THRESHOLD {
        Color::Yellow
    } else {
        Color::Green
    }
}

// Calculate the percentage of used RAM
pub fn calculate_percentage(used: u64, total: u64) -> f32 {
    (used as f32 / total as f32) * 100.0
}

// Convert bytes to gigabytes
pub fn bytes_to_gb(bytes: u64) -> f32 {
    bytes as f32 / 1024.0 / 1024.0 / 1024.0
}
