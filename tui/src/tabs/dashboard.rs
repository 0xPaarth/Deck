use crate::app::App;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(8),
        Constraint::Length(5),
        Constraint::Min(0),
    ])
    .split(area);

    draw_stats_block(frame, app, chunks[0]);
    draw_progress_block(frame, app, chunks[1]);
    draw_recommendations_block(frame, app, chunks[2]);
}

fn draw_stats_block(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let stats = vec![
        Line::from(vec![
            Span::styled("Rating:  ", Style::default().fg(Color::Yellow)),
            Span::styled(
                app.user_stats.rating.to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Solved:  ", Style::default().fg(Color::Yellow)),
            Span::styled(
                app.user_stats.solved.to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Streak:  ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{} days", app.user_stats.streak),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let block = Block::default()
        .title("Dashboard")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(stats).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_progress_block(frame: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("+120", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" rating this month"),
        ]),
        Line::from("Keep up the momentum!"),
    ];

    let block = Block::default()
        .title("Progress")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_recommendations_block(frame: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let recs = vec![
        Line::from(vec![
            Span::styled("1971D", Style::default().fg(Color::Cyan)),
            Span::raw("  Binary Cut  "),
            Span::styled("1100", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("1971C", Style::default().fg(Color::Cyan)),
            Span::raw("  Clock and Strings  "),
            Span::styled("900", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("1900A", Style::default().fg(Color::Cyan)),
            Span::raw("  Cover in Water  "),
            Span::styled("800", Style::default().fg(Color::Yellow)),
        ]),
    ];

    let block = Block::default()
        .title("Recommended Problems")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(recs).block(block);
    frame.render_widget(paragraph, area);
}
