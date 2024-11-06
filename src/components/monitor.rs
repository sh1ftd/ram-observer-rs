use sysinfo::System;
use ratatui::Frame;

use std::{
    collections::VecDeque,
    time::Instant,
};

use crate::components::{
    structs::{RamMonitor, LogEntry, MemoryAction},
    constants::{LOG_CAPACITY, DEFAULT_AUTO_THRESHOLD},
    utils::{self, bytes_to_gb, calculate_ram_percentage},
    ui,
};

impl RamMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            logs: VecDeque::with_capacity(LOG_CAPACITY),
            auto_threshold: DEFAULT_AUTO_THRESHOLD,
            auto_action: String::from(MemoryAction::EmptyWorkingSets.display_name()),
            last_auto_execution: None,
            selected_action: 0,
            last_key_press: None,
            last_action: None,
        }
    }

    pub fn add_log(&mut self, message: String, is_error: bool) {
        let entry = LogEntry {
            message,
            timestamp: Instant::now(),
            is_error,
        };
        
        if self.logs.len() >= LOG_CAPACITY {
            self.logs.pop_back();
        }
        self.logs.push_front(entry);
    }

    pub fn get_ram_usage(&mut self) -> (f32, f32, f32) {
        self.system.refresh_memory();
        let total = bytes_to_gb(self.system.total_memory());
        let used = bytes_to_gb(self.system.used_memory());
        let percentage = calculate_ram_percentage(self.system.used_memory(), self.system.total_memory());
        (used, total, percentage)
    }

    pub fn ui(&mut self, f: &mut Frame) {
        let chunks = ui::create_layout(f);
        let (used, total, percentage) = self.get_ram_usage();
        let gauge_color = utils::get_usage_color(percentage);

        ui::render_title(f, chunks[0]);
        ui::render_ram_gauge(f, chunks[1], used, total, percentage, gauge_color);
        ui::render_memory_management(f, chunks[2], self.selected_action);
        ui::render_auto_execution(f, chunks[3], self.auto_threshold, &self.auto_action);
        ui::render_logs(f, chunks[4], self);

        self.check_auto_execution(percentage);
    }

    pub fn cycle_auto_action(&mut self) {
        let current_action = match self.auto_action.as_str() {
            "Empty Working Sets" => MemoryAction::EmptySystemWorkingSets,
            "Empty System Working Sets" => MemoryAction::EmptyModifiedPageLists,
            "Empty Modified Page Lists" => MemoryAction::EmptyStandbyList,
            "Empty Standby List" => MemoryAction::EmptyPriorityZeroStandbyList,
            _ => MemoryAction::EmptyWorkingSets,
        };
        self.auto_action = String::from(current_action.display_name());
        self.add_log(format!("Auto-execution action changed to: {}", self.auto_action), false);
    }

    pub fn cycle_auto_threshold(&mut self) {
        self.auto_threshold = if self.auto_threshold >= 95.0 {
            20.0
        } else {
            self.auto_threshold + 5.0
        };
        self.add_log(format!("Auto-execution threshold changed to: {}%", self.auto_threshold), false);
    }
}
