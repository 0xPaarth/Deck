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
        Constraint::Length(3), // Stats bar
        Constraint::Length(3), // Time analytics
        Constraint::Min(6),    // Topic proficiency / weak tags
        Constraint::Length(5), // Predictions
    ])
    .split(area);

    draw_stats_bar(frame, app, chunks[0]);
    draw_time_analytics(frame, app, chunks[1]);
    draw_proficiency_and_weak_tags(frame, app, chunks[2]);
    draw_predictions(frame, app, chunks[3]);
}

fn draw_stats_bar(frame: &mut Frame, app: &App, area: Rect) {
    let text = vec![Line::from(vec![
        Span::styled("Rating: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            app.user_stats.rating.to_string(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("    "),
        Span::styled("Solved: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            app.user_stats.solved.to_string(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("    "),
        Span::styled("Streak: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("{} days", app.user_stats.streak),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("    "),
        Span::styled("Max Rating: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            app.user_stats.rating.max(1700).to_string(),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
    ])];

    let block = Block::default()
        .title("Stats")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_time_analytics(frame: &mut Frame, _app: &App, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("Avg solve time: ", Style::default().fg(Color::Yellow)),
            Span::styled("35 mins", Style::default().fg(Color::White)),
            Span::raw("  ↓ 5% from last month"),
        ]),
        Line::from(vec![
            Span::styled("Distribution: ", Style::default().fg(Color::Yellow)),
            Span::raw("0-15m: "),
            Span::styled("████████░░  8", Style::default().fg(Color::Green)),
            Span::raw("  15-30m: "),
            Span::styled("████████████  16", Style::default().fg(Color::Green)),
            Span::raw("  30-60m: "),
            Span::styled("████████░░  12", Style::default().fg(Color::Yellow)),
            Span::raw("  60m+: "),
            Span::styled("██░░░░░░░░  4", Style::default().fg(Color::Red)),
        ]),
    ];

    let block = Block::default()
        .title("Time Analytics")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_proficiency_and_weak_tags(frame: &mut Frame, _app: &App, area: Rect) {
    let chunks = Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Topic proficiency (mock grid)
    let lines = vec![
        Line::from("  Rating DP Greedy Binary Graphs Math String"),
        Line::from(vec![
            Span::raw(" 800    "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("     "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("      "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("      "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("      "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("    "),
            Span::styled("●", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("1200   "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("     "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("      "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("      "),
            Span::styled("⚠", Style::default().fg(Color::Yellow)),
            Span::raw("      "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("    "),
            Span::styled("●", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("1600   "),
            Span::styled("⚠", Style::default().fg(Color::Yellow)),
            Span::raw("     "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("      "),
            Span::styled("⚠", Style::default().fg(Color::Yellow)),
            Span::raw("      "),
            Span::styled("⚠", Style::default().fg(Color::Yellow)),
            Span::raw("      "),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::raw("    "),
            Span::styled("●", Style::default().fg(Color::Green)),
        ]),
        Line::from(format!(
            "{}",
            "  ● = Strong  ⚠ = Weak  ❌ = Critical"
        )),
    ];
    let block = Block::default()
        .title("Topic Proficiency")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[0]);

    // Weak tags
    let weak_tags_lines = vec![
        Line::from(vec![
            Span::styled("1. Graphs  ", Style::default().fg(Color::Red)),
            Span::styled("40%", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" accuracy (critical)"),
        ]),
        Line::from(vec![
            Span::styled("2. DP  ", Style::default().fg(Color::Yellow)),
            Span::styled("60%", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" accuracy (needs work)"),
        ]),
        Line::from(vec![
            Span::styled("3. Binary  ", Style::default().fg(Color::Yellow)),
            Span::styled("55%", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" accuracy (needs work)"),
        ]),
        Line::from(vec![
            Span::styled("4. Greedy  ", Style::default().fg(Color::Green)),
            Span::styled("75%", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" accuracy (good)"),
        ]),
    ];
    let block = Block::default()
        .title("Weak Tags")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(weak_tags_lines)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[1]);
}

fn draw_predictions(frame: &mut Frame, _app: &App, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("📈 Predicted rating in 3m: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                "1800",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("🎯 Next milestone: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                "1700 in 2 weeks",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("💡 Focus: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                "Graphs (lowest accuracy)",
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let block = Block::default()
        .title("Predictions")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
