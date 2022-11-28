use crate::config;
use crate::db::init;
use crate::ui::App;
use crate::state::ConfigFocus;
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
        KeyCode::Backspace => match app.config_input_focus {
            ConfigFocus::DbFile => {
                app.config_input_1.pop();
            }
            ConfigFocus::HighlightColor => {
                app.config_input_2.pop();
            }
        },
        KeyCode::Tab => match app.config_input_focus {
            ConfigFocus::DbFile => {
                app.config_input_focus = ConfigFocus::HighlightColor;
            }
            ConfigFocus::HighlightColor => {
                app.config_input_focus = ConfigFocus::DbFile;
            }
        },
        KeyCode::Char(c) => match app.config_input_focus {
            ConfigFocus::DbFile => {
                app.config_input_1.push(c);
            }
            ConfigFocus::HighlightColor => {
                app.config_input_2.push(c);
            }
        },
        KeyCode::Enter => {
            if app.config_input_1 != "" && app.config_input_2 != "" {
                config::set_config(
                    app.config_input_1.as_str().to_string(),
                    app.config_input_2.parse::<u8>().unwrap(),
                );
                app.db = init(format!("{}", config::get_db_file()).as_str());
                app.state.select(None);
                app.get_items();
                app.highlight_color = config::get_highlight_color();
                app.config_input_focus = ConfigFocus::DbFile;
                app.selected_window = Selected::Main;
            }
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

    // Config box
    let config_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Indexed(app.highlight_color)))
        .title(Span::styled(" Config ", Style::default().fg(Color::White)))
        .title_alignment(Alignment::Center);

    // Config layout
    let config_layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .horizontal_margin(3)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(35),
                Constraint::Percentage(35),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(center_col_layout[1]);

    // Config input center layout 1
    let config_input_center_layout_1 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(config_layout[1]);

    // Config input center layout 2
    let config_input_center_layout_2 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(config_layout[2]);

    // Config input block 1
    let config_input_block_1 = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    // Config input block 2
    let config_input_block_2 = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    // Config input layout 1
    let config_input_layout_1 = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(70)].as_ref())
        .split(config_input_center_layout_1[1]);

    // Config input layout 2
    let config_input_layout_2 = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(70)].as_ref())
        .split(config_input_center_layout_2[1]);

    // Config promt 1
    let config_promt_1 = Paragraph::new(Span::styled(
        "db_file:",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);

    // Config promt 2
    let config_promt_2 = Paragraph::new(Span::styled(
        "highlight_color:",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);

    // Config value 1
    let config_value_1 = Paragraph::new(Span::styled(
        app.config_input_1.as_str(),
        Style::default().fg(Color::White),
    ))
    .alignment(Alignment::Left);

    // Config value 2
    let config_value_2 = Paragraph::new(Span::styled(
        app.config_input_2.as_str(),
        Style::default().fg(Color::White),
    ))
    .alignment(Alignment::Left);

    // Render
    f.render_widget(config_block, center_col_layout[1]);
    f.render_widget(config_input_block_1, config_layout[1]);
    f.render_widget(config_input_block_2, config_layout[2]);
    f.render_widget(config_promt_1, config_input_layout_1[0]);
    f.render_widget(config_promt_2, config_input_layout_2[0]);
    f.render_widget(config_value_1, config_input_layout_1[1]);
    f.render_widget(config_value_2, config_input_layout_2[1]);
}
