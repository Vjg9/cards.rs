use crate::config;
use crate::ui::App;
use crate::ui::Selected;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, Row, Table},
    Frame,
};

pub fn handle_events(key_code: KeyCode, app: &mut App) {
    match key_code {
        KeyCode::Up => app.next(),
        KeyCode::Down => app.back(),
        KeyCode::Tab => {
            app.selected_window = Selected::Side;
        }
        KeyCode::Char('k') => app.next(),
        KeyCode::Char('j') => app.back(),
        KeyCode::Char('a') => {
            app.state.select(None);
            app.selected_window = Selected::StackNameInput;
        }
        KeyCode::Char('d') => {
            app.selected_window = Selected::DeleteStackPopup;
        }
        KeyCode::Enter => match app.state.selected() {
            Some(_i) => {
                app.selected_window = Selected::Side;
            }
            None => {}
        },
        KeyCode::Char('e') => {
            app.stack_name_input = app.get_selected_name();
            app.selected_window = Selected::EditStackPopup;
        }
        KeyCode::Char('c') => {
            app.config_input_1 = config::get_db_file_raw();
            app.config_input_2 = app.highlight_color.to_string();
            app.selected_window = Selected::ConfigOptions;
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

    // Main block layout
    let main_block_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(block_layout[0]);

    // Draw Main block
    let main_block;
    match app.selected_window {
        Selected::Main => {
            main_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" Stacks ", Style::default().fg(Color::White)))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Indexed(app.highlight_color)));
        }
        _ => {
            main_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" Stacks ", Style::default().fg(Color::White)))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::White));
        }
    }
    f.render_widget(main_block, block_layout[0]);

    // Stacks
    let stacks: Vec<ListItem> = app
        .items
        .iter()
        .map(|i| {
            let text = Span::from(Span::styled(&i.name, Style::default()));
            ListItem::new(text).style(Style::default().fg(Color::White))
        })
        .collect();

    // Render Stacks in a list
    let stacks = List::new(stacks).highlight_style(
        Style::default()
            .bg(Color::White)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    f.render_stateful_widget(stacks, main_block_layout[0], &mut app.state);

    // Render Main block options window
    let main_block_options = Block::default()
        .borders(Borders::ALL)
        .title(" Options ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded);
    f.render_widget(main_block_options, main_block_layout[1]);

    let main_block_options_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_block_layout[1]);

    // Render Options
    let options = Table::new(vec![Row::new(vec![
        "a: Add new",
        "d: Delete",
        "e: Edit",
        "<j, k>: up, down",
    ])
    .style(Style::default())])
    .widths(&[
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ]);
    f.render_widget(options, main_block_options_layout[1]);
}
