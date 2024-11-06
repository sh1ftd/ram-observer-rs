use ratatui::{
    Frame,
    layout::{ Layout, Direction, Constraint, Rect, Alignment},
    widgets::{ Block, Borders, Paragraph, Gauge, List, ListItem },
    style::{ Style, Color, Modifier},
    text::{ Text, Line, Span },
};

use crate::components::{
    utils::format_timestamp,
    structs::{ RamMonitor, MemoryAction }
};

pub fn create_layout(frame: &Frame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(2)
        .constraints([
            Constraint::Length(2),  // Title
            Constraint::Length(4),  // RAM gauge
            Constraint::Length(4),  // Page File gauge
            Constraint::Length(7),  // Memory management
            Constraint::Length(4),  // Auto execution
            Constraint::Min(2),     // Logs
            Constraint::Length(1),  // Bottom margin
        ])
        .split(frame.area())
        .to_vec()
}

pub fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("RAM Monitor")
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Cyan)))
        .style(Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD));
    f.render_widget(title, area);
}

pub fn render_ram_gauge(f: &mut Frame, area: Rect, used: f32, total: f32, percentage: f32, color: Color) {
    let gauge = Gauge::default()
        .block(Block::default()
            .title("RAM Usage")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL))
        .gauge_style(Style::default().fg(color))
        .ratio((percentage / 100.0) as f64)
        .label(format!("{:.1}GB / {:.1}GB ({:.1}%)", used, total, percentage));
    f.render_widget(gauge, area);
}

pub fn render_memory_management(f: &mut Frame, area: Rect, selected_action: usize) {
    let items: Vec<ListItem> = vec![
        MemoryAction::EmptyWorkingSets,
        MemoryAction::EmptySystemWorkingSets,
        MemoryAction::EmptyModifiedPageLists,
        MemoryAction::EmptyStandbyList,
        MemoryAction::EmptyPriorityZeroStandbyList,
    ].iter()
    .enumerate()
    .map(|(i, action)| {
        let prefix = if i == selected_action { ">> " } else { "   " };
        let content = format!("{}{}", prefix, action.display_name());
        let style = if i == selected_action {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        ListItem::new(content).style(style)
    })
    .collect();

    let list = List::new(items)
        .block(Block::default()
            .title("Memory Management")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL));
    f.render_widget(list, area);
}

pub fn render_auto_execution(f: &mut Frame, area: Rect, threshold: f32, action: &str) {
    let threshold_line = Line::from(vec![
        Span::raw(format!("Threshold: {}% ", threshold)),
        Span::styled("(Shift+T to change)", Style::default().fg(Color::DarkGray)),
    ]);

    let action_line = Line::from(vec![
        Span::raw(format!("Action: {} ", action)),
        Span::styled("(Shift+A to change)", Style::default().fg(Color::DarkGray)),
    ]);

    let text = Text::from(vec![threshold_line, action_line]);

    let paragraph = Paragraph::new(text)
        .block(Block::default()
            .title("Auto Execution")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

pub fn render_logs(f: &mut Frame, area: Rect, monitor: &RamMonitor) {
    let logs: Vec<ListItem> = monitor.logs.iter()
        .map(|log| {
            let time_str = format_timestamp(log.timestamp.elapsed());
            let style = if log.is_error {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };

            ListItem::new(format!("[{}] {}", time_str, log.message))
                .style(style)
        })
        .collect();

    let list = List::new(logs)
        .block(Block::default()
            .title("Logs")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL));
    f.render_widget(list, area);
}

pub fn render_page_file_gauge(f: &mut Frame, area: Rect, used: f32, total: f32, percentage: f32, color: Color) {
    let gauge = Gauge::default()
        .block(Block::default()
            .title("Page File Usage")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL))
        .gauge_style(Style::default().fg(color))
        .ratio((percentage / 100.0) as f64)
        .label(format!("{:.1}GB / {:.1}GB ({:.1}%)", used, total, percentage));
    f.render_widget(gauge, area);
}
