mod components;

use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::CrosstermBackend, Terminal};

use components::{event_handler, structs::RamMonitor};

/// RAM Monitor Application Entry Point
/// Controls:
/// - Up/Down: Navigate actions
/// - Enter: Execute selected action
/// - 1-5: Quick execute actions
/// - Shift+A: Cycle auto-action
/// - Shift+T: Cycle threshold
/// - Q: Quit
fn main() -> io::Result<()> {
    // Initialize terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut ram_monitor = RamMonitor::new();

    // Main event loop
    loop {
        // Render UI
        terminal.draw(|f| ram_monitor.ui(f))?;

        // Get appropriate tick rate based on activity
        let current_tick_rate = ram_monitor.get_current_tick_rate();

        // Handle input events
        if event::poll(Duration::from_millis(current_tick_rate))?
            && let Event::Key(key) = event::read()? {
                ram_monitor.last_activity = Instant::now();
                if event_handler::handle_key_events(&mut ram_monitor, key, Instant::now()) {
                    break;
                }
            }
    }

    // Cleanup and restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
