use crate::ui::App;
use crate::ui::Selected;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

pub fn handle_events(key_code: KeyCode, app: &mut App) {
    match key_code {
        KeyCode::Esc => {
            app.selected_window = Selected::Side;
        }
        KeyCode::Char('j') => app.next_card(),
        KeyCode::Char('k') => app.back_card(),
        KeyCode::Up => app.back_card(),
        KeyCode::Down => app.next_card(),
        KeyCode::Char('d') => {
            if app.cards.len() > 0 {
                app.selected_window = Selected::DeleteCard;
            }
        }
        KeyCode::Char('e') => {
            if app.cards.len() > 0 {
                match app.cards_state.selected() {
                    Some(i) => {
                        app.card_title_input = app.cards[i].title.as_str().to_string();
                        app.card_text_input = app.cards[i].text.as_str().to_string();
                    }
                    None => {}
                }
                app.selected_window = Selected::EditCard;
            }
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
    // Card list box
    let card_list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Indexed(app.highlight_color)))
        .title(Span::styled(" Cards ", Style::default().fg(Color::White)))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    // Card list layout
    let card_list_layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(2)
        .horizontal_margin(3)
        .constraints([Constraint::Percentage(100)])
        .split(center_col_layout[1]);

    // Card list list
    let cards: Vec<ListItem> = app
        .cards
        .iter()
        .map(|i| {
            let text = Span::from(Span::styled(&i.title, Style::default()));
            ListItem::new(text).style(Style::default().fg(Color::White))
        })
        .collect();

    // Render Cards in a list
    let cards = List::new(cards).highlight_style(
        Style::default()
            .bg(Color::White)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    // Render
    f.render_widget(card_list_block, center_col_layout[1]);
    f.render_stateful_widget(cards, card_list_layout[0], &mut app.cards_state);
}
