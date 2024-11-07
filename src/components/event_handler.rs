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

// Define a mapping of actions at module level
const ACTION_MAP: [(char, Commands); 5] = [
    ('1', Commands::EmptyWorkingSets),
    ('2', Commands::EmptySystemWorkingSets),
    ('3', Commands::EmptyModifiedPageLists),
    ('4', Commands::EmptyStandbyList),
    ('5', Commands::EmptyPriorityZeroStandbyList),
];

pub fn handle_key_events(
    ram_monitor: &mut RamMonitor,
    key: KeyEvent,
    current_time: Instant,
) -> bool {
    // Combine cooldown checks into a single function
    let can_process = |last_time: Option<Instant>, cooldown: u128| -> bool {
        last_time.map_or(true, |last| current_time.duration_since(last).as_millis() > cooldown)
    };

    let can_nav = can_process(ram_monitor.last_key_press, NAV_COOLDOWN_MS);
    let can_act = can_process(ram_monitor.last_action, ACTION_COOLDOWN_MS);

    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => return true,
        
        (KeyCode::Up, _) if can_nav => {
            ram_monitor.selected_action = ram_monitor.selected_action.saturating_sub(1);
            ram_monitor.last_key_press = Some(current_time);
        }
        
        (KeyCode::Down, _) if can_nav => {
            ram_monitor.selected_action = (ram_monitor.selected_action + 1).min(ACTION_MAP.len() - 1);
            ram_monitor.last_key_press = Some(current_time);
        }
        
        (KeyCode::Enter, _) if can_act => {
            if let Some(action) = ACTION_MAP.get(ram_monitor.selected_action) {
                ram_monitor.run_rammap(action.1);
                ram_monitor.last_action = Some(current_time);
            }
        }
        
        (KeyCode::Char('A'), m) if m.contains(KeyModifiers::SHIFT) && can_nav => {
            ram_monitor.cycle_auto_action();
            ram_monitor.last_key_press = Some(current_time);
        }
        
        (KeyCode::Char('T'), m) if m.contains(KeyModifiers::SHIFT) && can_nav => {
            ram_monitor.cycle_auto_threshold();
            ram_monitor.last_key_press = Some(current_time);
        }
        
        (KeyCode::Char(c), _) if can_act => {
            if let Some((_, command)) = ACTION_MAP.iter()
                .find(|(key, _)| *key == c) {
                ram_monitor.run_rammap(*command);
                ram_monitor.last_action = Some(current_time);
            }
        }
        
        _ => {}
    }

    false
}
