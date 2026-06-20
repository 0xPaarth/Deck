use crate::app::App;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(6),
    ])
    .split(area);

    draw_header(frame, app, chunks[0]);
    draw_topics(frame, app, chunks[1]);
    draw_plan(frame, app, chunks[2]);
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let current = app.user_stats.rating;
    let target = (current / 100 + 1) * 100;
    let pct = (current as f32 / target as f32 * 100.0).min(100.0) as u32;
    let filled = (pct / 10).min(10) as usize;
    let bar = "█".repeat(filled) + &"░".repeat(10 - filled);

    let text = vec![Line::from(vec![
        Span::styled("🎯 Target: ", Style::default().fg(Color::Cyan)),
        Span::styled(
            target.to_string(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" (Current: {})  |  Progress: {} {}%", current, bar, pct)),
    ])];

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_topics(frame: &mut Frame, _app: &App, area: Rect) {
    let topics = vec![
        ("✅", "Greedy", 75, 85, Color::Green, "3 problems/week"),
        ("⏳", "DP", 60, 75, Color::Yellow, "5 problems/week"),
        ("⏳", "Graphs", 40, 65, Color::Red, "5 problems/week 🔥"),
    ];

    let mut lines: Vec<Line> = Vec::new();
    for (icon, name, current, target, color, plan) in topics {
        lines.push(Line::from(vec![
            Span::styled(icon, Style::default().fg(color)),
            Span::raw(" "),
            Span::styled(
                format!("{:<10}", name),
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("({}% → {}%)", current, target),
                Style::default().fg(color),
            ),
            Span::raw(" — "),
            Span::styled(plan, Style::default().fg(Color::Gray)),
        ]));
    }

    let block = Block::default()
        .title("Topics to Master")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn draw_plan(frame: &mut Frame, _app: &App, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Plan:", Style::default().fg(Color::Yellow)),
        ]),
        Line::from("  1. This week: 5 Graph problems (1400-1600)"),
        Line::from("  2. Next week: 5 DP problems (1400-1600)"),
        Line::from("  3. Contest: CF #1234 (practice)"),
    ];

    let block = Block::default()
        .title("Weekly Practice")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
