use crate::ui::App;
use crate::ui::Selected;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn handle_events(key_code: KeyCode, app: &mut App) {
    match key_code {
        KeyCode::Char(c) => {
            if app.stack_name_input.len() < 22 {
                app.stack_name_input.push(c)
            }
        }
        KeyCode::Esc => {
            app.stack_name_input = String::new();
            app.selected_window = Selected::Main;
        }
        KeyCode::Enter => {
            if app.stack_name_input.to_string() != "" {
                app.add_stack(app.stack_name_input.to_string());
                app.get_items();
                app.stack_name_input = String::new();
                app.selected_window = Selected::Main;
            }
        }
        KeyCode::Backspace => {
            app.stack_name_input.pop();
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

    // Add Stack Popub window
    let add_stack_popup_block = Block::default()
        .style(Style::default().fg(Color::Indexed(app.highlight_color)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            " Add Stack ",
            Style::default().fg(Color::White),
        ))
        .title_alignment(Alignment::Center);

    // Add Stack Popup Layout
    let add_stack_popup_layout_row = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(50),
            Constraint::Percentage(17),
        ])
        .split(center_col_layout[1]);
    let add_stack_popup_layout_col_1 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(add_stack_popup_layout_row[1]);
    let add_stack_popup_layout_col_0 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(add_stack_popup_layout_row[0]);

    // Add Stack Input box layout
    let add_stack_popup_input_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(6)
        .vertical_margin(2)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(42),
            Constraint::Percentage(33),
        ])
        .split(center_col_layout[1]);

    // Add Stack Input Text
    let add_stack_input = Paragraph::new(Span::from(app.stack_name_input.as_ref()))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    // Add Stack "name:" text
    let add_stack_input_text = Paragraph::new(Span::from("name:"))
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Right);

    // Add Stack input outline
    let add_stack_input_outline = Block::default()
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Render
    f.render_widget(add_stack_popup_block, center_col_layout[1]);
    f.render_widget(add_stack_input_outline, add_stack_popup_input_layout[1]);
    f.render_widget(add_stack_input, add_stack_popup_layout_col_1[1]);
    f.render_widget(add_stack_input_text, add_stack_popup_layout_col_0[1]);
}
