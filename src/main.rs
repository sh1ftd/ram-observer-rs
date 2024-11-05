mod components;

use std::{
    io::{ self, stdout },
    time::{ Duration, Instant }
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
    constants::TICK_RATE,
    structs::RamMonitor,
    event_handler
};
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut ram_monitor = RamMonitor::new();
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ram_monitor.ui(f))?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let current_time = Instant::now();
                
                if event_handler::handle_key_events(&mut ram_monitor, key, current_time) {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            last_tick = Instant::now();
        }
    }

    // Clean up
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}