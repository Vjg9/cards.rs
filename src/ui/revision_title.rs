use crate::ui::App;
use crate::ui::Selected;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn handle_events(key_code: KeyCode, app: &mut App) {
    match key_code {
        KeyCode::Esc => {
            app.selected_window = Selected::Side;
            app.revision_index = 0;
        }
        KeyCode::Enter => {
            app.selected_window = Selected::RevisionText;
        }
        _ => {}
    }
}

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Center Layout for pupup window
    let center_row_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());
    let center_col_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(center_row_layout[1]);

    // Revision title box
    let revision_title_box = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(" Front ", Style::default().fg(Color::White)))
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Indexed(app.highlight_color)));

    // Revision title box layout
    let revision_title_layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(2)
        .horizontal_margin(3)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(center_col_layout[1]);

    // Revision title promt
    let revision_title_promt;
    if app.cards.len() > 0 {
        revision_title_promt = Paragraph::new(Span::styled(
            app.cards[app.revision_index].title.as_str(),
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Center);
    } else {
        revision_title_promt =
            Paragraph::new(Span::styled("No title", Style::default().fg(Color::White)))
                .alignment(Alignment::Center);
    }

    // Revision cards index layout col
    let revision_cards_index_layout_col = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(39),
            Constraint::Percentage(20),
            Constraint::Percentage(1),
        ])
        .split(center_col_layout[1]);

    // Revision cards index layout
    let revision_cards_index_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(revision_cards_index_layout_col[3]);

    // Revision cards index box
    let revision_cards_index_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Revision cards index box layout
    let revision_cards_index_block_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(revision_cards_index_layout[1]);

    // Revision cards index promt
    let revision_cards_index_promt = Paragraph::new(Span::styled(
        format!("{}/{}", app.revision_index + 1, app.cards.len()),
        Style::default().fg(Color::White),
    ))
    .alignment(Alignment::Center);

    // Render
    f.render_widget(revision_title_box, center_col_layout[1]);
    f.render_widget(revision_title_promt, revision_title_layout[1]);
    f.render_widget(revision_cards_index_block, revision_cards_index_layout[1]);
    f.render_widget(
        revision_cards_index_promt,
        revision_cards_index_block_layout[1],
    );
}
