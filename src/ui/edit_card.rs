use crate::state::CardInputFocus;
use crate::ui::App;
use crate::ui::Selected;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn handle_events(key_code: KeyCode, app: &mut App) {
    match key_code {
        KeyCode::Esc => {
            app.selected_window = Selected::CardList;
            app.card_text_input = String::new();
            app.card_title_input = String::new();
            app.card_input_focus = CardInputFocus::Title;
        }
        KeyCode::Tab => match &app.card_input_focus {
            CardInputFocus::Title => {
                app.card_input_focus = CardInputFocus::Text;
            }
            CardInputFocus::Text => {
                app.card_input_focus = CardInputFocus::Title;
            }
        },
        KeyCode::Enter => {
            if app.card_text_input != "".to_string() && app.card_title_input != "".to_string() {
                app.edit_card();
                app.card_text_input = String::new();
                app.card_title_input = String::new();
                app.card_input_focus = CardInputFocus::Title;
                app.list_cards();
                app.selected_window = Selected::CardList;
            }
        }
        KeyCode::Backspace => match &app.card_input_focus {
            CardInputFocus::Title => {
                app.card_title_input.pop();
            }
            CardInputFocus::Text => {
                app.card_text_input.pop();
            }
        },
        KeyCode::Char(c) => match &app.card_input_focus {
            CardInputFocus::Title => {
                if app.card_title_input.len() < 30 {
                    app.card_title_input.push(c)
                }
            }
            CardInputFocus::Text => {
                if app.card_text_input.len() < 100 {
                    app.card_text_input.push(c)
                }
            }
        },
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

    // Add card center layout
    let add_card_center_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(49),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(center_row_layout[1]);

    // Add card layout
    let add_card_layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(3)
        .horizontal_margin(7)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(add_card_center_layout[1]);

    // Add card title input box
    let add_card_title_input_box = match app.card_input_focus {
        CardInputFocus::Title => Block::default()
            .style(Style::default().fg(Color::Indexed(app.highlight_color)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        CardInputFocus::Text => Block::default()
            .style(Style::default().fg(Color::White))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    };

    // Add card text input box
    let add_card_text_input_box = match app.card_input_focus {
        CardInputFocus::Text => Block::default()
            .style(Style::default().fg(Color::Indexed(app.highlight_color)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        CardInputFocus::Title => Block::default()
            .style(Style::default().fg(Color::White))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    };

    // Add card title input box layout
    let add_card_title_input_layout_half = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(add_card_layout[0]);
    let add_card_title_input_layout = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(2)
        .constraints([Constraint::Percentage(17), Constraint::Percentage(83)])
        .split(add_card_title_input_layout_half[1]);

    // Add card text input box layout
    let add_card_text_input_promt_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(22), Constraint::Percentage(78)])
        .split(add_card_layout[1]);
    let add_card_text_input_layout = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(2)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(add_card_text_input_promt_layout[1]);

    // Add card title input box promt
    let add_card_title_input_promt = match app.card_input_focus {
        CardInputFocus::Title => Paragraph::new(Span::from("title: "))
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .style(Style::default().fg(Color::Indexed(app.highlight_color))),
            ),
        CardInputFocus::Text => Paragraph::new(Span::from("title: ")).style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    };

    // Add card title input box value
    let add_card_title_input_value = match app.card_input_focus {
        CardInputFocus::Title => Paragraph::new(Span::from(app.card_title_input.as_ref()))
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .style(Style::default().fg(Color::Indexed(app.highlight_color))),
            ),
        CardInputFocus::Text => Paragraph::new(Span::from(app.card_title_input.as_ref()))
            .style(Style::default().fg(Color::White)),
    };

    // Add card text input box promt
    let add_card_text_input_promt = match app.card_input_focus {
        CardInputFocus::Text => Paragraph::new(Span::from("text: "))
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .style(Style::default().fg(Color::Indexed(app.highlight_color))),
            ),
        CardInputFocus::Title => Paragraph::new(Span::from("text: ")).style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    };

    // Add card text input box value
    let add_card_text_input_value = match app.card_input_focus {
        CardInputFocus::Text => Paragraph::new(Span::from(app.card_text_input.as_ref()))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .style(Style::default().fg(Color::Indexed(app.highlight_color))),
            ),
        CardInputFocus::Title => Paragraph::new(Span::from(app.card_text_input.as_ref()))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White)),
    };

    // Edit card box
    let edit_card_block = Block::default()
        .style(Style::default().fg(Color::Indexed(app.highlight_color)))
        .borders(Borders::ALL)
        .title(Span::styled(
            " Edit Card ",
            Style::default().fg(Color::White),
        ))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    // Render
    f.render_widget(edit_card_block, add_card_center_layout[1]);
    f.render_widget(add_card_title_input_box, add_card_layout[0]);
    f.render_widget(add_card_text_input_box, add_card_layout[1]);
    f.render_widget(add_card_title_input_promt, add_card_title_input_layout[0]);
    f.render_widget(add_card_text_input_promt, add_card_text_input_layout[0]);
    f.render_widget(add_card_title_input_value, add_card_title_input_layout[1]);
    f.render_widget(add_card_text_input_value, add_card_text_input_layout[1]);
}
