use ratatui::Frame;
use sysinfo::System;

use std::{
    time::Instant,
    collections::VecDeque
};

use crate::components::{
    ui,
    memory_management::Commands,
    utils::{self, bytes_to_gb, calculate_percentage},
    structs::{RamMonitor, LogEntry, ActivityState, Config},
    constants::{LOG_CAPACITY, ACTIVE_TICK_RATE_MS, IDLE_TICK_RATE_MS, IDLE_THRESHOLD_MS, DEFAULT_AUTO_THRESHOLD}
};

impl RamMonitor {
    /// Creates a new RamMonitor instance with default settings
    pub fn new() -> Self {
        let mut monitor = Self {
            system: System::new_all(),
            logs: VecDeque::with_capacity(LOG_CAPACITY),
            auto_threshold: DEFAULT_AUTO_THRESHOLD,
            auto_action: String::from("Empty Working Sets"),
            last_auto_execution: None,
            selected_action: 0,
            last_key_press: None,
            last_action: None,
            last_activity: Instant::now(),
            activity_state: ActivityState::Active,
            config: Config::default(),
        };
        
        monitor.config = Config::load(&mut monitor);
        monitor.auto_threshold = monitor.config.auto_threshold;
        monitor.auto_action = monitor.config.auto_action.clone();
        
        monitor
    }

    /// Adds a new log entry, removing oldest if at capacity
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

    /// Returns current RAM usage as (used_gb, total_gb, percentage)
    pub fn get_ram_usage(&mut self) -> (f32, f32, f32) {
        self.system.refresh_memory();
        let total = bytes_to_gb(self.system.total_memory());
        let used = bytes_to_gb(self.system.used_memory());
        let percentage = calculate_percentage(self.system.used_memory(), self.system.total_memory());
        (used, total, percentage)
    }

    /// Returns page file usage as Option<(used_gb, total_gb, percentage)>
    pub fn get_page_file_usage(&mut self) -> Option<(f32, f32, f32)> {
        self.system.refresh_memory();
        let total = self.system.total_swap();
        if total == 0 {
            return None;
        }
        let used = self.system.used_swap();
        let total_gb = bytes_to_gb(total);
        let used_gb = bytes_to_gb(used);
        let percentage = calculate_percentage(used, total);
        Some((used_gb, total_gb, percentage))
    }

    /// Renders all UI components and checks auto-execution
    pub fn ui(&mut self, f: &mut Frame) {
        let chunks = ui::create_layout(f);
        let (used, total, percentage) = self.get_ram_usage();
        let ram_gauge_color = utils::get_usage_color(percentage);
        let page_file = self.get_page_file_usage();

        ui::render_ram_gauge(f, chunks[1], used, total, percentage, ram_gauge_color);
        if let Some((used, total, percentage)) = page_file {
            let page_file_color = utils::get_usage_color(percentage);
            ui::render_page_file_gauge(f, chunks[2], used, total, percentage, page_file_color);
        }
        ui::render_memory_management(f, chunks[3], self.selected_action);
        ui::render_auto_execution(f, chunks[4], self.auto_threshold, &self.auto_action);
        ui::render_logs(f, chunks[5], self);

        self.check_auto_execution(percentage);
    }

    /// Cycles to the next available auto-execution action
    pub fn cycle_auto_action(&mut self) {
        let current_action = match self.auto_action.as_str() {
            "Empty Working Sets" => Commands::EmptySystemWorkingSets,
            "Empty System Working Sets" => Commands::EmptyModifiedPageLists,
            "Empty Modified Page Lists" => Commands::EmptyStandbyList,
            "Empty Standby List" => Commands::EmptyPriorityZeroStandbyList,
            _ => Commands::EmptyWorkingSets,
        };
        let new_action = String::from(current_action.display_name());
        self.add_log(format!("Auto-execution action changed to: {}", new_action), false);
        self.auto_action = new_action.clone();
        self.config.auto_action = new_action;
        self.save_config();
    }

    /// Cycles auto-threshold between 20% and 95% in 5% increments
    pub fn cycle_auto_threshold(&mut self) {
        let new_threshold = if self.auto_threshold >= 95.0 {
            20.0
        } else {
            self.auto_threshold + 5.0
        };
        self.add_log(format!("Auto-execution threshold changed to: {}%", new_threshold), false);
        self.auto_threshold = new_threshold;
        self.config.auto_threshold = new_threshold;
        self.save_config();
    }

    /// Returns appropriate tick rate based on system activity state
    pub fn get_current_tick_rate(&mut self) -> u64 {
        let is_idle = self.last_activity.elapsed().as_millis() > IDLE_THRESHOLD_MS;
        
        match (is_idle, &self.activity_state) {
            (true, ActivityState::Active) => {
                self.activity_state = ActivityState::Idle;
                self.add_log(format!("Entering idle mode (tick rate: {}ms)", IDLE_TICK_RATE_MS), false);
                IDLE_TICK_RATE_MS
            }
            (false, ActivityState::Idle) => {
                self.activity_state = ActivityState::Active;
                self.add_log(format!("Switching to active mode (tick rate: {}ms)", ACTIVE_TICK_RATE_MS), false);
                ACTIVE_TICK_RATE_MS
            }
            (true, ActivityState::Idle) => IDLE_TICK_RATE_MS,
            (false, ActivityState::Active) => ACTIVE_TICK_RATE_MS,
        }
    }

    fn save_config(&mut self) {
        match self.config.save() {
            Ok(messages) if !messages.is_empty() => {
                for (msg, is_error) in messages {
                    self.add_log(msg, is_error);
                }
            }
            Ok(_) => {}
            Err(e) => {
                self.add_log(format!("Failed to save config: {}", e), true);
            }
        }
    }
}
