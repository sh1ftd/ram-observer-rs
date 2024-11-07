use std::time::Instant;
use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };

use crate::components::{
    structs::RamMonitor,
    memory_management::Commands,
    constants::{
        NAV_COOLDOWN_MS,
        ACTION_COOLDOWN_MS
    }
};

/// Determines if enough time has passed since the last action to allow a new action
/// 
/// # Arguments
/// * `last_time` - When the action was last performed (if ever)
/// * `current_time` - The current timestamp
/// * `cooldown` - Minimum milliseconds that must pass between actions
/// 
/// # Returns
/// * `true` if enough time has passed (or if this is the first action)
/// * `false` if not enough time has passed since last action
fn can_process(last_time: Option<Instant>, current_time: Instant, cooldown: u128) -> bool {
    match last_time {
        // If there's no last action, always allow
        None => true,
        // If there is a last action, check if enough time has passed
        Some(last) => {
            let time_passed = current_time.duration_since(last).as_millis();
            time_passed > cooldown
        }
    }
}

/// Handles keyboard input events for the RAM monitor
/// 
/// # Arguments
/// * `ram_monitor` - Mutable reference to the RAM monitor state
/// * `key` - The keyboard event to process
/// * `current_time` - Current timestamp for cooldown calculations
/// 
/// # Returns
/// * `true` if the program should exit (q key pressed)
/// * `false` if the program should continue running
/// 
/// # Controls
/// * `q` - Exit program
/// * `Up/Down` - Navigate through available actions
/// * `Enter` - Execute selected action
/// * `Shift + A` - Cycle auto-action setting
/// * `Shift + T` - Cycle auto-threshold setting
/// * `1-5` - Hotkeys for direct action execution
pub fn handle_key_events(
    ram_monitor: &mut RamMonitor,
    key: KeyEvent,
    current_time: Instant,
) -> bool {
    let can_nav = can_process(ram_monitor.last_key_press, current_time, NAV_COOLDOWN_MS);
    let can_act = can_process(ram_monitor.last_action, current_time, ACTION_COOLDOWN_MS);

    match (key.code, key.modifiers) {
        // Exit program
        (KeyCode::Char('q'), _) => return true,
        
        // Navigate up through actions
        (KeyCode::Up, _) if can_nav => {
            ram_monitor.selected_action = ram_monitor.selected_action.saturating_sub(1); // Ensure we don't go below 0
            ram_monitor.last_key_press = Some(current_time);
        }
        
        // Navigate down through actions
        (KeyCode::Down, _) if can_nav => {
            ram_monitor.selected_action = (ram_monitor.selected_action + 1)
                .min(Commands::ACTION_MAP.len() - 1); // Ensure we don't go out of bounds
            ram_monitor.last_key_press = Some(current_time);
        }
        
        // Execute selected action via enter key
        (KeyCode::Enter, _) if can_act => {
            if let Some(command) = Commands::from_index(ram_monitor.selected_action) {
                ram_monitor.run_rammap(command);
                ram_monitor.last_action = Some(current_time);
            }
        }
        
        // Cycle auto action
        (KeyCode::Char('A'), m) if m.contains(KeyModifiers::SHIFT) && can_nav => {
            ram_monitor.cycle_auto_action();
            ram_monitor.last_key_press = Some(current_time);
        }
        
        // Cycle auto threshold
        (KeyCode::Char('T'), m) if m.contains(KeyModifiers::SHIFT) && can_nav => {
            ram_monitor.cycle_auto_threshold();
            ram_monitor.last_key_press = Some(current_time);
        }
        
        // Execute action via hotkey
        (KeyCode::Char(c), _) if can_act => {
            if let Some(command) = Commands::from_char(c) {
                ram_monitor.run_rammap(command);
                ram_monitor.last_action = Some(current_time);
            }
        }
        
        _ => {}
    }

    false
}
