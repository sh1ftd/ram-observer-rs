use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::{
    time::Instant,
    collections::VecDeque
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub auto_threshold: f32,
    pub auto_action: String,
}

pub enum ActivityState {
    Active,
    Idle,
}

pub struct LogEntry {
    pub message: String,
    pub timestamp: Instant,
    pub is_error: bool,
}

pub struct RamMonitor {
    pub system: System,
    pub logs: VecDeque<LogEntry>,
    pub auto_threshold: f32,
    pub auto_action: String,
    pub last_auto_execution: Option<Instant>,
    pub selected_action: usize,
    pub last_key_press: Option<Instant>,
    pub last_action: Option<Instant>,
    pub last_activity: Instant,
    pub activity_state: ActivityState,
    pub config: Config,
}
