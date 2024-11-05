use sysinfo::System;

use std::{
    collections::VecDeque,
    time::Instant,
};

#[derive(Clone, Copy)]
pub enum MemoryAction {
    EmptyWorkingSets,
    EmptySystemWorkingSets,
    EmptyModifiedPageLists,
    EmptyStandbyList,
    EmptyPriorityZeroStandbyList,
}

impl MemoryAction {
    pub fn parameter(&self) -> &str {
        match self {
            Self::EmptyWorkingSets => "-Ew",
            Self::EmptySystemWorkingSets => "-Es",
            Self::EmptyModifiedPageLists => "-Em",
            Self::EmptyStandbyList => "-Et",
            Self::EmptyPriorityZeroStandbyList => "-E0",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::EmptyWorkingSets => "Empty Working Sets",
            Self::EmptySystemWorkingSets => "Empty System Working Sets",
            Self::EmptyModifiedPageLists => "Empty Modified Page Lists",
            Self::EmptyStandbyList => "Empty Standby List",
            Self::EmptyPriorityZeroStandbyList => "Empty Priority 0 Standby List",
        }
    }
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
}
