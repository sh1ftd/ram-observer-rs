use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};

use crate::components::{
    memory_management::Commands, structs::RamMonitor, utils::format_timestamp,
};

pub fn create_layout(frame: &Frame<'_>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(2)
        .constraints([
            Constraint::Length(1), // Top margin
            Constraint::Length(4), // RAM gauge
            Constraint::Length(4), // Page File gauge
            Constraint::Length(7), // Memory management
            Constraint::Length(4), // Auto execution
            Constraint::Min(2),    // Logs
            Constraint::Length(1), // Bottom margin
        ])
        .split(frame.area())
        .to_vec()
}

pub fn render_ram_gauge(
    f: &mut Frame<'_>,
    area: Rect,
    used: f32,
    total: f32,
    percentage: f32,
    color: Color,
) {
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Span::styled("RAM Usage", Style::default().fg(Color::Cyan)))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .gauge_style(Style::default().fg(color))
        .ratio((percentage / 100.0) as f64)
        .label(Span::styled(
            format!("{used:.1}GB / {total:.1}GB ({percentage:.1}%)"),
            Style::default().fg(Color::White),
        ));
    f.render_widget(gauge, area);
}

pub fn render_page_file_gauge(
    f: &mut Frame<'_>,
    area: Rect,
    used: f32,
    total: f32,
    percentage: f32,
    color: Color,
) {
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Span::styled(
                    "Page File Usage",
                    Style::default().fg(Color::Cyan),
                ))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .gauge_style(Style::default().fg(color))
        .ratio((percentage / 100.0) as f64)
        .label(Span::styled(
            format!("{used:.1}GB / {total:.1}GB ({percentage:.1}%)"),
            Style::default().fg(Color::White),
        ));
    f.render_widget(gauge, area);
}

pub fn render_memory_management(f: &mut Frame<'_>, area: Rect, selected_action: usize) {
    let items: Vec<ListItem<'_>> = [
        Commands::EmptyWorkingSets,
        Commands::EmptySystemWorkingSets,
        Commands::EmptyModifiedPageLists,
        Commands::EmptyStandbyList,
        Commands::EmptyPriorityZeroStandbyList,
    ]
    .iter()
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

    let list = List::new(items).block(
        Block::default()
            .title(Span::styled(
                "Memory Management",
                Style::default().fg(Color::Cyan),
            ))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(list, area);
}

pub fn render_auto_execution(f: &mut Frame<'_>, area: Rect, threshold: f32, action: &str) {
    let threshold_line = Line::from(vec![
        Span::raw(format!("Threshold: {threshold}% ")),
        Span::styled("(Shift+T to change)", Style::default().fg(Color::DarkGray)),
    ]);

    let action_line = Line::from(vec![
        Span::raw(format!("Action: {action} ")),
        Span::styled("(Shift+A to change)", Style::default().fg(Color::DarkGray)),
    ]);

    let text = Text::from(vec![threshold_line, action_line]);

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .title(Span::styled(
                "Auto Execution",
                Style::default().fg(Color::Cyan),
            ))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(paragraph, area);
}

pub fn render_logs(f: &mut Frame<'_>, area: Rect, monitor: &RamMonitor) {
    let logs: Vec<ListItem<'_>> = monitor
        .logs
        .iter()
        .map(|log| {
            let time_str = format_timestamp(log.timestamp.elapsed());
            let style = if log.is_error {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };

            ListItem::new(format!("[{}] {}", time_str, log.message)).style(style)
        })
        .collect();

    let list = List::new(logs).block(
        Block::default()
            .title(Span::styled("Logs", Style::default().fg(Color::Cyan)))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(list, area);
}
