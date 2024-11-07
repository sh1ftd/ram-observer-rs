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

pub fn handle_key_events(
    ram_monitor: &mut RamMonitor,
    key: KeyEvent,
    current_time: Instant,
) -> bool { // Returns true if the program should exit
    let can_process = |last_time: Option<Instant>, cooldown: u128| -> bool {
        last_time.map_or(true, |last| current_time.duration_since(last).as_millis() > cooldown)
    };

    let can_nav = can_process(ram_monitor.last_key_press, NAV_COOLDOWN_MS);
    let can_act = can_process(ram_monitor.last_action, ACTION_COOLDOWN_MS);

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
