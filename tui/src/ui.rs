use crate::app::{App, AppState};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(area);

    draw_title_bar(frame, app, chunks[0]);
    draw_subitle_bar(frame, app, chunks[1]);
    draw_tab_bar(frame, app, chunks[2]);
    draw_content(frame, app, chunks[3]);
    draw_footer(frame, app, chunks[4]);

    if app.show_help {
        draw_help_popup(frame, app);
    }

    if app.state == AppState::Error(String::new()) || matches!(app.state, AppState::Error(_)) {
        if let AppState::Error(ref msg) = app.state {
            draw_error_banner(frame, msg);
        }
    }
}

fn draw_title_bar(frame: &mut Frame, app: &App, area: Rect) {
    let title = "Deck v0.1.0";
    let stats = format!(
        "	{}-day streak 	{}",
        app.user_stats.streak, app.user_stats.rating
    );
    let text = Line::from(vec![
        Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(stats, Style::default().fg(Color::Yellow)),
    ]);
    let paragraph = Paragraph::new(text).alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn draw_subitle_bar(frame: &mut Frame, app: &App, area: Rect) {
    let text = format!(
        "{}  |  Team: {}  |  Git: {}",
        app.user_stats.handle,
        app.user_stats.team,
        if app.user_stats.git_enabled {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn draw_tab_bar(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = app
        .tabs
        .iter()
        .map(|t| Line::from(t.title.clone()))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .select(app.current_tab)
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "));

    frame.render_widget(tabs, area);
}

fn draw_content(frame: &mut Frame, app: &App, area: Rect) {
    match app.current_tab {
        0 => crate::tabs::dashboard::draw(frame, app, area),
        1 => crate::tabs::problems::draw(frame, app, area),
        2 => crate::tabs::analytics::draw(frame, app, area),
        3 => crate::tabs::team::draw(frame, app, area),
        4 => crate::tabs::contest::draw(frame, app, area),
        5 => crate::tabs::target::draw(frame, app, area),
        6 => crate::tabs::config::draw(frame, app, area),
        _ => unreachable!(),
    }
}

fn draw_footer(frame: &mut Frame, _app: &App, area: Rect) {
    let text = "[?] Help  [Tab] Switch  [q] Quit";
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn draw_help_popup(frame: &mut Frame, _app: &App) {
    let area = centered_rect(60, 50, frame.area());
    let block = Block::default()
        .title("Keyboard Shortcuts")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let text = Paragraph::new(
        "Tab / Shift+Tab  - Navigate tabs\n\
         j / k            - Navigate list (Problems tab)\n\
         n / Enter        - Open selected problem\n\
         r                - Refresh data\n\
         /                - Search (placeholder)\n\
         q                - Quit\n\
         ?                - Toggle this help",
    )
    .block(block)
    .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(text, area);
}

fn draw_error_banner(frame: &mut Frame, msg: &str) {
    let area = centered_rect(80, 20, frame.area());
    let block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::Red));
    let text = Paragraph::new(msg.to_string())
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(Clear, area);
    frame.render_widget(text, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
