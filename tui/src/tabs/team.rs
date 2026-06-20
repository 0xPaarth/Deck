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
        Constraint::Min(6),
        Constraint::Length(3),
    ])
    .split(area);

    draw_header(frame, app, chunks[0]);
    draw_members(frame, app, chunks[1]);
    draw_team_stats(frame, app, chunks[2]);
}

fn draw_header(frame: &mut Frame, _app: &App, area: Rect) {
    let text = Line::from(vec![
        Span::styled("👥 Team: ", Style::default().fg(Color::Cyan)),
        Span::styled(
            "CP-Squad",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" (3 members)    Last active: 2m ago"),
    ]);
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_members(frame: &mut Frame, _app: &App, area: Rect) {
    let members = vec![
        (
            "@alice",
            1600u32,
            12u32,
            "████████░░",
            vec![true, true, false, true, false],
        ),
        (
            "@bob",
            1400,
            5,
            "████████░░",
            vec![true, true, false, false, false],
        ),
        (
            "@charlie",
            1700,
            8,
            "██████████",
            vec![true, true, true, true, false],
        ),
    ];

    let mut lines: Vec<Line> = Vec::new();
    for (name, rating, streak, bar, today) in members {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {:<10}", name),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                format!("{:>5}", rating),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw("  "),
            Span::styled(bar, Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled(
                format!("{}-day streak", streak),
                Style::default().fg(Color::Gray),
            ),
        ]));
        let today_str = today
            .iter()
            .map(|b| if *b { "✅" } else { "⬜" })
            .collect::<Vec<_>>()
            .join("");
        lines.push(Line::from(vec![
            Span::raw("    Today: "),
            Span::styled(today_str, Style::default().fg(Color::White)),
            Span::raw(format!(
                "  ({}/{})",
                today.iter().filter(|b| **b).count(),
                today.len()
            )),
        ]));
        lines.push(Line::from(""));
    }

    let block = Block::default()
        .title("Member Progress")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn draw_team_stats(frame: &mut Frame, _app: &App, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("Team Stats: ", Style::default().fg(Color::Yellow)),
            Span::raw("11/20 solved this week  |  Total: 513"),
        ]),
        Line::from(vec![
            Span::styled("Weak Tags: ", Style::default().fg(Color::Yellow)),
            Span::raw("DP, Graphs, Binary"),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
