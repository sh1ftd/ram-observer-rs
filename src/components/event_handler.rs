use std::time::Instant;

use crossterm::event::{ KeyCode, KeyEvent };

use crate::components::{
    structs::{ RamMonitor, MemoryAction },
    constants::{
        NAV_COOLDOWN_MS,
        ACTION_COOLDOWN_MS
    }
};

pub fn handle_key_events(
    ram_monitor: &mut RamMonitor,
    key: KeyEvent,
    current_time: Instant,
) -> bool {  // returns true if should exit
    let can_process_nav = ram_monitor.last_key_press
        .map_or(true, |last| current_time.duration_since(last).as_millis() > NAV_COOLDOWN_MS);
    let can_process_action = ram_monitor.last_action
        .map_or(true, |last| current_time.duration_since(last).as_millis() > ACTION_COOLDOWN_MS);

    match key.code {
        KeyCode::Char('q') => return true,
        
        KeyCode::Up | KeyCode::Down => {
            handle_navigation(ram_monitor, key.code, can_process_nav, current_time);
        }
        
        KeyCode::Enter => {
            handle_action_execution(ram_monitor, can_process_action, current_time);
        }
        
        KeyCode::Char(c) => {
            handle_number_action(ram_monitor, c, can_process_action, current_time);
        }
        
        _ => {}
    }

    false
}

pub fn handle_navigation(
    ram_monitor: &mut RamMonitor,
    key_code: KeyCode,
    can_process: bool,
    current_time: Instant,
) {
    if !can_process {
        return;
    }

    match key_code {
        KeyCode::Up => {
            ram_monitor.selected_action = ram_monitor.selected_action.saturating_sub(1);
            ram_monitor.last_key_press = Some(current_time);
        }
        KeyCode::Down => {
            ram_monitor.selected_action = (ram_monitor.selected_action + 1).min(4);
            ram_monitor.last_key_press = Some(current_time);
        }
        _ => {}
    }
}

pub fn handle_action_execution(
    ram_monitor: &mut RamMonitor,
    can_process: bool,
    current_time: Instant,
) {
    if !can_process {
        return;
    }

    let action = match ram_monitor.selected_action {
        0 => MemoryAction::EmptyWorkingSets,
        1 => MemoryAction::EmptySystemWorkingSets,
        2 => MemoryAction::EmptyModifiedPageLists,
        3 => MemoryAction::EmptyStandbyList,
        4 => MemoryAction::EmptyPriorityZeroStandbyList,
        _ => return,
    };

    ram_monitor.execute_memory_action(action);
    ram_monitor.last_action = Some(current_time);
}

pub fn handle_number_action(
    ram_monitor: &mut RamMonitor,
    key: char,
    can_process: bool,
    current_time: Instant,
) {
    if !can_process {
        return;
    }

    let action = match key {
        '1' => Some(MemoryAction::EmptyWorkingSets),
        '2' => Some(MemoryAction::EmptySystemWorkingSets),
        '3' => Some(MemoryAction::EmptyModifiedPageLists),
        '4' => Some(MemoryAction::EmptyStandbyList),
        '5' => Some(MemoryAction::EmptyPriorityZeroStandbyList),
        _ => None,
    };

    if let Some(action) = action {
        ram_monitor.execute_memory_action(action);
        ram_monitor.last_action = Some(current_time);
    }
}
