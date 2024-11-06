use std::time::Duration;
use ratatui::style::Color;

use crate::components::constants::{
    CRITICAL_THRESHOLD,
    WARNING_THRESHOLD
};

pub fn format_timestamp(duration: Duration) -> String {
    if duration.as_secs() < 60 {
        format!("{:>3}s ago", duration.as_secs())
    } else if duration.as_secs() < 3600 {
        format!("{:>3}m ago", duration.as_secs() / 60)
    } else {
        format!("{:>3}h ago", duration.as_secs() / 3600)
    }
}

/// Formats a size in bytes to a human-readable string (B, KB, MB, GB)
/// Currently unused but reserved for future detailed memory information display
#[allow(dead_code)]
pub fn format_size(size_in_bytes: u64) -> String {
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const KB: f64 = 1024.0;

    let size = size_in_bytes as f64;
    
    if size >= GB {
        format!("{:.2} GB", size / GB)
    } else if size >= MB {
        format!("{:.2} MB", size / MB)
    } else if size >= KB {
        format!("{:.2} KB", size / KB)
    } else {
        format!("{} B", size)
    }
}

pub fn get_usage_color(percentage: f32) -> Color {
    if percentage >= CRITICAL_THRESHOLD {
        Color::Red
    } else if percentage >= WARNING_THRESHOLD {
        Color::Yellow
    } else {
        Color::Green
    }
}

pub fn calculate_ram_percentage(used: u64, total: u64) -> f32 {
    (used as f32 / total as f32) * 100.0
}

pub fn bytes_to_gb(bytes: u64) -> f32 {
    bytes as f32 / 1024.0 / 1024.0 / 1024.0
}
