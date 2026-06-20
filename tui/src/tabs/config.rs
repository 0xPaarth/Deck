use crate::app::App;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(7),
        Constraint::Min(0),
    ])
    .split(area);

    draw_general(frame, app, chunks[0]);
    draw_git(frame, app, chunks[1]);
    draw_footer(frame, app, chunks[2]);
}

fn draw_general(frame: &mut Frame, _app: &App, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Language:     ", Style::default().fg(Color::Yellow)),
            Span::styled("cpp", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Editor:       ", Style::default().fg(Color::Yellow)),
            Span::styled("nvim", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Theme:        ", Style::default().fg(Color::Yellow)),
            Span::styled("default", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Handle:       ", Style::default().fg(Color::Yellow)),
            Span::styled("alice_cp", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Rating Goal:  ", Style::default().fg(Color::Yellow)),
            Span::styled("1700", Style::default().fg(Color::Green)),
        ]),
    ];

    let block = Block::default()
        .title("General")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_git(frame: &mut Frame, _app: &App, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Enabled:      ", Style::default().fg(Color::Yellow)),
            Span::styled("true", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Auto-commit:  ", Style::default().fg(Color::Yellow)),
            Span::styled("true", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Auto-push:    ", Style::default().fg(Color::Yellow)),
            Span::styled("false", Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::styled("Repo path:    ", Style::default().fg(Color::Yellow)),
            Span::styled("~/.deck/repo", Style::default().fg(Color::White)),
        ]),
    ];

    let block = Block::default()
        .title("Git")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_footer(frame: &mut Frame, _app: &App, area: Rect) {
    let text = "[e] Edit config.toml  [r] Reload  [i] Reset settings";
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
