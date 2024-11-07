mod components;

use std::{
    io::{ self, stdout },
    time::{ Instant, Duration }
};

use crossterm::{
    ExecutableCommand,
    event::{ self, Event },
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen }
};

use ratatui::{
    Terminal,
    prelude::CrosstermBackend
};

use components::{
    event_handler,
    structs::RamMonitor
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut ram_monitor = RamMonitor::new();

    loop {
        terminal.draw(|f| ram_monitor.ui(f))?;

        let current_tick_rate = ram_monitor.get_current_tick_rate();

        if event::poll(Duration::from_millis(current_tick_rate))? {
            if let Event::Key(key) = event::read()? {
                ram_monitor.last_activity = Instant::now();
                if event_handler::handle_key_events(&mut ram_monitor, key, std::time::Instant::now()) {
                    break;
                }
            }
        }
    }

    // Clean up
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}