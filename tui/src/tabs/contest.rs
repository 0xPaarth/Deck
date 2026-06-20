use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(3),
    ])
    .split(area);

    draw_header(frame, app, chunks[0]);
    draw_grid(frame, app, chunks[1]);
    draw_standings(frame, app, chunks[2]);
    draw_footer(frame, app, chunks[3]);
}

fn draw_header(frame: &mut Frame, _app: &App, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("🏆 ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "Codeforces Round #1234",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("    "),
            Span::styled("⏱️ ", Style::default().fg(Color::Red)),
            Span::styled(
                "1:45:23 remaining",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_grid(frame: &mut Frame, _app: &App, area: Rect) {
    let problems = vec![
        ("A", "solved", "+500"),
        ("B", "attempting", "0/??"),
        ("C", "skipped", "0/??"),
        ("D", "skipped", "0/??"),
        ("E", "skipped", "0/??"),
    ];

    let mut items: Vec<Line> = Vec::new();
    for (letter, status, score) in problems {
        let color = match status {
            "solved" => Color::Green,
            "attempting" => Color::Yellow,
            _ => Color::DarkGray,
        };
        let icon = match status {
            "solved" => "✅",
            "attempting" => "⏳",
            _ => "❌",
        };
        items.push(Line::from(vec![
            Span::styled(letter, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::raw(": "),
            Span::styled(icon, Style::default().fg(color)),
            Span::raw(" "),
            Span::styled(score, Style::default().fg(Color::Gray)),
        ]));
    }

    let block = Block::default()
        .title("Problems")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(items)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn draw_standings(frame: &mut Frame, _app: &App, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("Score: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "500",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("    "),
            Span::styled("Rank: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "#234",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("    "),
            Span::styled("Rating Δ: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "+45",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_footer(frame: &mut Frame, _app: &App, area: Rect) {
    let text = "[o] Open  [r] Refresh  [t] Timer  [q] Quit Contest";
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
