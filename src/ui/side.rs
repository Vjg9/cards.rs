use crate::ui::App;
use crate::ui::Selected;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn handle_events(key_code: KeyCode, app: &mut App) {
    match key_code {
        KeyCode::Tab => {
            app.selected_window = Selected::Main;
        }
        KeyCode::Char('a') => app.selected_window = Selected::AddCard,
        KeyCode::Char('l') => {
            app.list_cards();
            if app.cards.len() > 0 {
                app.cards_state.select(Some(0));
            }
            app.selected_window = Selected::CardList;
        }
        KeyCode::Char('s') => {
            app.list_cards();
            if app.cards.len() > 0 {
                app.selected_window = Selected::RevisionTitle;
            }
        }
        KeyCode::Esc => {
            app.selected_window = Selected::Main;
        }
        _ => {}
    }
}

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Layout for Main and Side blocks
    let block_layout = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    // Draw Side block
    let side_block;
    match app.selected_window {
        Selected::Side => {
            side_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Selected Stack ",
                    Style::default().fg(Color::White),
                ))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Indexed(app.highlight_color)));
        }
        _ => {
            side_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Selected Stack ",
                    Style::default().fg(Color::White),
                ))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::White));
        }
    }
    f.render_widget(side_block, block_layout[1]);

    // Side block layout
    let side_block_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(6)
        .vertical_margin(2)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(12),
                Constraint::Percentage(5),
                Constraint::Percentage(65),
                Constraint::Percentage(2),
            ]
            .as_ref(),
        )
        .split(block_layout[1]);

    // Side block name layout
    let side_block_name_layout = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(2)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(43),
                Constraint::Percentage(13),
                Constraint::Percentage(1),
            ]
            .as_ref(),
        )
        .split(side_block_layout[1]);

    // Side block selected stack name box
    let side_block_name_box = Block::default()
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Side block selected stack name
    let side_block_name = Block::default()
        .style(Style::default().fg(Color::White))
        .title(Span::styled(
            app.get_selected_name(),
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    // Side block options layout
    let side_block_options_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(4)
        .vertical_margin(2)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(side_block_layout[3]);

    // Side block option blocks
    let side_block_option_1 = Block::default()
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let side_block_option_2 = Block::default()
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let side_block_option_3 = Block::default()
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Side block options text layout
    let side_block_options_text_layout_1 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(7)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(side_block_options_layout[1]);
    let side_block_options_text_layout_2 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(7)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(side_block_options_layout[3]);
    let side_block_options_text_layout_3 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(7)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(side_block_options_layout[2]);

    // Side block option text
    let side_block_name_1 = Block::default()
        .title(Span::styled(
            "a: Add Card",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    let side_block_name_2 = Block::default()
        .title(Span::styled(
            "s: Start Revision",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    let side_block_name_3 = Block::default()
        .title(Span::styled(
            "l: List Cards",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    // Render side block widgets
    match app.state.selected() {
        Some(_i) => {
            f.render_widget(side_block_name_box, side_block_layout[1]);
            f.render_widget(side_block_name, side_block_name_layout[1]);
            f.render_widget(side_block_option_1, side_block_options_layout[1]);
            f.render_widget(side_block_option_2, side_block_options_layout[3]);
            f.render_widget(side_block_option_3, side_block_options_layout[2]);
            f.render_widget(side_block_name_1, side_block_options_text_layout_1[1]);
            f.render_widget(side_block_name_2, side_block_options_text_layout_2[1]);
            f.render_widget(side_block_name_3, side_block_options_text_layout_3[1]);
        }
        None => {}
    }
}
