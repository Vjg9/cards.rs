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
        KeyCode::Enter => {
            let id = app.get_selected_id();
            app.state.select(None);
            app.delete_stack(id);
            app.get_items();
            app.selected_window = Selected::Main;
        }
        KeyCode::Esc => {
            app.selected_window = Selected::Main;
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

    // Delete Stack popup
    let delete_stack_popup_block = Block::default()
        .style(Style::default().fg(Color::Indexed(app.highlight_color)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Delete Stack question
    let delete_stack_popup_text = Paragraph::new(Span::from("Are you Sure?"))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    // Delete Stack Layout
    let delete_stack_popup_layout_row = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(center_col_layout[1]);
    let delete_stack_popup_layout_col_1 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(15),
        ])
        .split(delete_stack_popup_layout_row[1]);

    // Delete Stack Button
    let delete_stack_button = Block::default()
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Delete Stack Button Layout
    let delete_stack_button_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(delete_stack_popup_layout_col_1[2]);

    // Delete Stack Button text
    let delete_stack_button_text = Paragraph::new(Span::from("Yes"))
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));

    // Render
    f.render_widget(delete_stack_popup_block, center_col_layout[1]);
    f.render_widget(delete_stack_popup_text, delete_stack_popup_layout_col_1[1]);
    f.render_widget(delete_stack_button, delete_stack_popup_layout_col_1[2]);
    f.render_widget(delete_stack_button_text, delete_stack_button_layout[1]);
}
