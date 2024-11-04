use std::{
    io::{self, stdout},
    time::{Duration, Instant},
    process::Command,
    collections::VecDeque,
    fs,
    path::Path,
    io::Write,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::*,
    style::{Style, Color},
};
use sysinfo::System;

struct LogEntry {
    message: String,
    timestamp: Instant,
    is_error: bool,
}

struct RamMonitor {
    system: System,
    logs: VecDeque<LogEntry>,
    auto_threshold: f32,
    auto_action: String,
    last_auto_execution: Option<Instant>,
    selected_action: usize,
    last_key_press: Option<Instant>,
}

impl RamMonitor {
    fn new() -> Self {
        Self {
            system: System::new_all(),
            logs: VecDeque::with_capacity(100),
            auto_threshold: 85.0,
            auto_action: String::from("Empty Working Sets"),
            last_auto_execution: None,
            selected_action: 0,
            last_key_press: None,
        }
    }

    fn add_log(&mut self, message: String, is_error: bool) {
        self.logs.push_front(LogEntry {
            message,
            timestamp: Instant::now(),
            is_error,
        });

        while self.logs.len() > 100 {
            self.logs.pop_back();
        }
    }

    fn ensure_rammap_exists(&mut self) -> io::Result<()> {
        if !Path::new("RAMMap64.exe").exists() {
            self.add_log("RAMMap64.exe not found. Downloading...".to_string(), false);
            
            // Download the zip file
            let response = reqwest::blocking::get("https://download.sysinternals.com/files/RAMMap.zip")
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            
            let bytes = response.bytes()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            
            // Save zip file temporarily
            let mut temp_file = fs::File::create("rammap_temp.zip")?;
            temp_file.write_all(&bytes)?;
            
            // Extract RAMMap64.exe from the zip
            let file = fs::File::open("rammap_temp.zip")?;
            let mut archive = zip::ZipArchive::new(file)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                
                if file.name() == "RAMMap64.exe" {
                    let mut outfile = fs::File::create("RAMMap64.exe")?;
                    io::copy(&mut file, &mut outfile)?;
                    break;
                }
            }
            
            // Clean up the temporary zip file
            fs::remove_file("rammap_temp.zip")?;
            self.add_log("Successfully downloaded RAMMap64.exe".to_string(), false);
        }
        Ok(())
    }

    fn run_rammap(&mut self, parameter: &str, action_name: &str) {
        // First ensure RAMMap exists
        if let Err(e) = self.ensure_rammap_exists() {
            self.add_log(format!("Failed to download RAMMap: {}", e), true);
            return;
        }

        self.add_log(format!("Executing: {}...", action_name), false);
        match Command::new("RAMMap64.exe").arg(parameter).spawn() {
            Ok(_) => {
                self.add_log(format!("Successfully executed: {}", action_name), false);
            },
            Err(e) => {
                let error_msg = format!("Failed to execute RAMMap64: {}", e);
                self.add_log(error_msg, true);
            },
        }
    }

    fn check_auto_execution(&mut self, current_percentage: f32) {
        if current_percentage >= self.auto_threshold {
            if self.last_auto_execution.map_or(true, |time| time.elapsed().as_secs() > 300) {
                let parameter = match self.auto_action.as_str() {
                    "Empty Working Sets" => "-Ew",
                    "Empty System Working Sets" => "-Es",
                    "Empty Modified Page Lists" => "-Em",
                    "Empty Standby List" => "-Et",
                    "Empty Priority 0 Standby List" => "-E0",
                    _ => "-Ew"  // default
                };
                let action_name = self.auto_action.clone();
                self.run_rammap(parameter, &action_name);
                self.last_auto_execution = Some(Instant::now());
            }
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),  // Title
                Constraint::Length(4),  // RAM gauge (increased height)
                Constraint::Length(7),  // Memory management (increased for borders)
                Constraint::Length(4),  // Auto execution (increased for borders)
                Constraint::Min(2),     // Logs
            ])
            .margin(2)  // Add margin around the entire UI
            .split(f.area());

        // Title
        let title = Paragraph::new("RAM Monitor")
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(Color::Cyan)))
            .style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD));
        f.render_widget(title, chunks[0]);

        // RAM Stats
        self.system.refresh_memory();
        let total = (self.system.total_memory() as f32) / 1024.0 / 1024.0 / 1024.0;
        let used = (self.system.used_memory() as f32) / 1024.0 / 1024.0 / 1024.0;
        let percentage = (used / total) * 100.0;

        self.check_auto_execution(percentage);

        let gauge_color = if percentage >= 90.0 {
            Color::Red
        } else if percentage >= 70.0 {
            Color::Yellow
        } else {
            Color::Green
        };

        let ram_gauge = Gauge::default()
            .block(Block::default()
                .title(Span::styled("RAM Usage", Style::default().add_modifier(Modifier::BOLD)))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(gauge_color)))
            .gauge_style(Style::default()
                .fg(gauge_color)
                .bg(Color::Rgb(30, 30, 30))  // Custom dark gray using RGB values
                .add_modifier(Modifier::BOLD))
            .ratio((used / total) as f64)
            .label(Span::styled(
                format!("{:.1}GB / {:.1}GB ({:.1}%)", used, total, percentage),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            ));
        f.render_widget(ram_gauge, chunks[1]);

        // Memory Management
        let actions = vec![
            Line::from(vec![
                Span::styled(
                    if self.selected_action == 0 { "▶ [1]" } else { "  [1]" },
                    Style::default().fg(if self.selected_action == 0 { Color::Cyan } else { Color::Yellow })
                ),
                Span::raw(" Empty Working Sets"),
            ]),
            Line::from(vec![
                Span::styled(
                    if self.selected_action == 1 { "▶ [2]" } else { "  [2]" },
                    Style::default().fg(if self.selected_action == 1 { Color::Cyan } else { Color::Yellow })
                ),
                Span::raw(" Empty System Working Sets"),
            ]),
            Line::from(vec![
                Span::styled(
                    if self.selected_action == 2 { "▶ [3]" } else { "  [3]" },
                    Style::default().fg(if self.selected_action == 2 { Color::Cyan } else { Color::Yellow })
                ),
                Span::raw(" Empty Modified Page Lists"),
            ]),
            Line::from(vec![
                Span::styled(
                    if self.selected_action == 3 { "▶ [4]" } else { "  [4]" },
                    Style::default().fg(if self.selected_action == 3 { Color::Cyan } else { Color::Yellow })
                ),
                Span::raw(" Empty Standby List"),
            ]),
            Line::from(vec![
                Span::styled(
                    if self.selected_action == 4 { "▶ [5]" } else { "  [5]" },
                    Style::default().fg(if self.selected_action == 4 { Color::Cyan } else { Color::Yellow })
                ),
                Span::raw(" Empty Priority 0 Standby List"),
            ]),
        ];
        let actions_widget = Paragraph::new(actions)
            .block(Block::default()
                .title(Span::styled("Memory Management", Style::default().add_modifier(Modifier::BOLD)))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        f.render_widget(actions_widget, chunks[2]);

        // Auto Execution
        let auto_exec = vec![
            Line::from(vec![
                Span::raw("Threshold: "),
                Span::styled(
                    format!("{:.1}%", self.auto_threshold),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::raw("Action: "),
                Span::styled(
                    &self.auto_action,
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];
        let auto_widget = Paragraph::new(auto_exec)
            .block(Block::default()
                .title(Span::styled("Auto Execution", Style::default().add_modifier(Modifier::BOLD)))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        f.render_widget(auto_widget, chunks[3]);

        // Logs
        let logs: Vec<ListItem> = self.logs.iter()
            .map(|log| {
                let elapsed = log.timestamp.elapsed().as_secs_f32();
                let time_text = if elapsed < 60.0 {
                    format!("{:.0}s ago", elapsed)
                } else {
                    format!("{:.0}m ago", elapsed / 60.0)
                };
                
                let color = if log.is_error { Color::Red } else { Color::Green };
                ListItem::new(Line::from(vec![
                    Span::styled("• ", Style::default().fg(color)),
                    Span::raw(&log.message),
                    Span::styled(format!(" ({})", time_text), Style::default().fg(Color::DarkGray)),
                ]))
            })
            .collect();

        let logs_widget = List::new(logs)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("Logs", Style::default().add_modifier(Modifier::BOLD)))
                .title_alignment(Alignment::Center));
        f.render_widget(logs_widget, chunks[4]);
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut ram_monitor = RamMonitor::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs(1);

    loop {
        terminal.draw(|f| ram_monitor.ui(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let current_time = Instant::now();
                let nav_cooldown = 150;
                let action_cooldown = 1000;
                
                let can_process_nav = ram_monitor.last_key_press
                    .map_or(true, |last| current_time.duration_since(last).as_millis() > nav_cooldown);
                let can_process_action = ram_monitor.last_key_press
                    .map_or(true, |last| current_time.duration_since(last).as_millis() > action_cooldown);

                match key.code {
                    // Exit the program
                    KeyCode::Char('q') => break,

                    // Up and Down keys control the selected action
                    KeyCode::Up => {
                        if can_process_nav {
                            ram_monitor.selected_action = ram_monitor.selected_action.saturating_sub(1);
                            ram_monitor.last_key_press = Some(current_time);
                        }
                    }
                    KeyCode::Down => {
                        if can_process_nav {
                            ram_monitor.selected_action = (ram_monitor.selected_action + 1).min(4);
                            ram_monitor.last_key_press = Some(current_time);
                        }
                    }
                    // Trigger with Enter key
                    KeyCode::Enter => {
                        if can_process_action {
                            match ram_monitor.selected_action {
                                0 => ram_monitor.run_rammap("-Ew", "Empty Working Sets"),
                                1 => ram_monitor.run_rammap("-Es", "Empty System Working Sets"),
                                2 => ram_monitor.run_rammap("-Em", "Empty Modified Page Lists"),
                                3 => ram_monitor.run_rammap("-Et", "Empty Standby List"),
                                4 => ram_monitor.run_rammap("-E0", "Empty Priority 0 Standby List"),
                                _ => {}
                            }
                            ram_monitor.last_key_press = Some(current_time);
                        }
                    }
                    // Trigger with number keys
                    KeyCode::Char(c) => {
                        if can_process_action {
                            match c {
                                '1' => ram_monitor.run_rammap("-Ew", "Empty Working Sets"),
                                '2' => ram_monitor.run_rammap("-Es", "Empty System Working Sets"),
                                '3' => ram_monitor.run_rammap("-Em", "Empty Modified Page Lists"),
                                '4' => ram_monitor.run_rammap("-Et", "Empty Standby List"),
                                '5' => ram_monitor.run_rammap("-E0", "Empty Priority 0 Standby List"),
                                _ => {}
                            }
                            // Update the last key press time
                            ram_monitor.last_key_press = Some(current_time);
                        }
                    }
                    _ => {}
                }
            }
        }
        // Update the last tick time
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // Clean up
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}