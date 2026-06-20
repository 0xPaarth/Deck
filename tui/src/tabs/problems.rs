use crate::app::App;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{ Block, Borders, Cell, HighlightSpacing, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let header_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let header = Row::new(vec![
        Cell::from("ID"),
        Cell::from("Title"),
        Cell::from("Rating"),
        Cell::from("Tags"),
        Cell::from("Status"),
    ])
    .style(header_style)
    .height(1);

    let rows: Vec<Row> = app
        .problems
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let is_selected = i == app.selected_index;
            let row_style = if is_selected {
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let rating_text = p.rating.map(|r| r.to_string()).unwrap_or_default();
            let status = if p.solved { "solved" } else { "unsolved" };
            let status_style = if p.solved {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            Row::new(vec![
                Cell::from(p.id.clone()),
                Cell::from(p.title.clone()),
                Cell::from(rating_text),
                Cell::from(p.tags.join(", ")),
                Cell::from(Span::styled(status, status_style)),
            ])
            .style(row_style)
            .height(1)
        })
        .collect();

    let widths = [
        Constraint::Length(10),
        Constraint::Percentage(35),
        Constraint::Length(10),
        Constraint::Percentage(30),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title("Problems")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_spacing(HighlightSpacing::Always)
        .column_spacing(2);

    // Split area for table + detail panel
    let chunks = ratatui::layout::Layout::horizontal([
        Constraint::Percentage(70),
        Constraint::Percentage(30),
    ])
    .split(area);

    frame.render_widget(table, chunks[0]);
    draw_detail_panel(frame, app, chunks[1]);
}

fn draw_detail_panel(frame: &mut Frame, app: &App, area: Rect) {
    let content = if let Some(p) = app.selected_problem() {
        vec![
            Line::from(vec![
                Span::styled("ID:     ", Style::default().fg(Color::Yellow)),
                Span::raw(p.id.clone()),
            ]),
            Line::from(vec![
                Span::styled("Title:  ", Style::default().fg(Color::Yellow)),
                Span::raw(p.title.clone()),
            ]),
            Line::from(vec![
                Span::styled("Rating: ", Style::default().fg(Color::Yellow)),
                Span::raw(p.rating.map(|r| r.to_string()).unwrap_or_default()),
            ]),
            Line::from(vec![
                Span::styled("Tags:   ", Style::default().fg(Color::Yellow)),
                Span::raw(p.tags.join(", ")),
            ]),
            Line::from(vec![
                Span::styled("Time:   ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{} ms", p.time_limit)),
            ]),
            Line::from(vec![
                Span::styled("Memory: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{} MB", p.memory_limit)),
            ]),
        ]
    } else {
        vec![Line::from("No problem selected")]
    };

    let block = Block::default()
        .title("Details")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
