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
            app.selected_window = Selected::CardList;
        }
        KeyCode::Enter => {
            app.delete_card();
            app.list_cards();
            app.selected_window = Selected::CardList;
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

    // Delete card box
    let delete_card_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Indexed(app.highlight_color)));

    // Delete card layout
    let delete_card_layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(2)
        .horizontal_margin(3)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(15),
        ])
        .split(center_col_layout[1]);

    // Delete card promt
    let delete_card_promt = Paragraph::new(Span::styled(
        "Are you sure?",
        Style::default().fg(Color::White),
    ))
    .alignment(Alignment::Center);

    // Delete card button box
    let delete_card_button_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    // Delete card button layout
    let delete_card_button_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(delete_card_layout[2]);

    // Delete card button text
    let delete_card_button_text =
        Paragraph::new(Span::styled("Yes", Style::default().fg(Color::White)))
            .alignment(Alignment::Center);

    // Render
    f.render_widget(delete_card_block, center_col_layout[1]);
    f.render_widget(delete_card_promt, delete_card_layout[1]);
    f.render_widget(delete_card_button_block, delete_card_layout[2]);
    f.render_widget(delete_card_button_text, delete_card_button_layout[1]);
}
