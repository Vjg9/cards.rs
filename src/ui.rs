use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Color, Modifier},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Table, Row, Paragraph, Wrap},
    Frame, Terminal
};
use crate::db::{init, stack, card};
use crate::db::stack::Stack;
use crate::db::card::Card;
use rusqlite::Connection;

// Selected Window Enum
enum Selected {
    Main,
    Side,
    StackNameInput,
    DeleteStackPopup,
    AddCard,
}

// Card Input Focus Enum
enum CardInputFocus {
    Title,
    Text,
}

struct App {
    items: Vec<Stack>,
    state: ListState,
    db: Result<Connection, rusqlite::Error>,
    selected_window: Selected,
    stack_name_input: String,
    card_title_input: String,
    card_text_input: String,
    card_input_focus: CardInputFocus,
}

impl App {
    fn new() -> App {
       App {
            items: vec![],
            state: ListState::default(),
            db: init("./dev.db"),
            selected_window: Selected::Main,
            stack_name_input: String::new(), 
            card_title_input: String::new(),
            card_text_input: String::new(),
            card_input_focus: CardInputFocus::Title,
       } 
    } 

    // Add card
    fn add_card(&mut self, title: String, text: String) {
        let stack_id = self.get_selected_id();
        card::add(self.db.as_ref().unwrap(), stack_id, title, text);
    }

    // Get stacks
    fn get_items(&mut self) {
        self.items = stack::get_all(self.db.as_ref().unwrap());
    }

    // Add stack
    fn add_stack(&mut self, name: String) {
        stack::add(self.db.as_ref().unwrap(), name);
    }
    
    // Delete stack
    fn delete_stack(&mut self, id: i32) {
        stack::delete(self.db.as_ref().unwrap(), id);
    }

    // Get id from selected stack
    fn get_selected_id(&mut self) -> i32 {
        let _i = match self.state.selected() {
            Some(i) => {
                return self.items[i].id
            },
            None => {
                return 0
            }
        };
    }

    // Get name from selected stack
    fn get_selected_name(&mut self) -> String {
        let _i = match self.state.selected() {
            Some(i) => {
                return self.items[i].name.to_string()
            },
            None => {
                return "".to_string()
            }
        };
    }

    // Select next stack
    fn next(&mut self) {
        if self.items.len() > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    // Select previous stack
    fn back(&mut self) {
        if self.items.len() > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }
} 

// Run the ui
pub fn run_ui() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

// Runs the app main loop
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    app.get_items();
    if app.items.len() > 0 {
        app.state.select(Some(0));
    }
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.selected_window {
                Selected::Main => match key.code {
                    KeyCode::Char('q') => return Ok(()), 
                    KeyCode::Up => app.next(),
                    KeyCode::Down => app.back(),
                    KeyCode::Tab => {
                        app.selected_window = Selected::Side;
                    },
                    KeyCode::Char('k') => app.next(),
                    KeyCode::Char('j') => app.back(),
                    KeyCode::Char('a') => {
                        app.state.select(None);
                        app.selected_window = Selected::StackNameInput;
                    },
                    KeyCode::Char('d') => {
                        app.selected_window = Selected::DeleteStackPopup;
                    },
                    KeyCode::Enter => {
                        match app.state.selected() {
                            Some(_i) => {
                                app.selected_window = Selected::Side;
                            }
                            None => {}
                        }
                    }
                    _ => {}
                }
                Selected::Side => match key.code {
                    KeyCode::Char('q') => return Ok(()), 
                    KeyCode::Tab => {
                        app.selected_window = Selected::Main;
                    },
                    KeyCode::Char('a') => {
                        app.selected_window = Selected::AddCard
                    }
                    KeyCode::Esc => {
                        app.selected_window = Selected::Main;
                    }
                    _ => {}
                }
                Selected::StackNameInput => match key.code {
                    KeyCode::Char(c) => {
                        if app.stack_name_input.len() < 22 {
                            app.stack_name_input.push(c)
                        }
                    },
                    KeyCode::Esc => {
                        app.stack_name_input = String::new();
                        app.selected_window = Selected::Main;
                    },
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
                Selected::DeleteStackPopup => match key.code {
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
                Selected::AddCard => match key.code {
                    KeyCode::Esc => {
                        app.selected_window = Selected::Side;
                        app.card_text_input = String::new();
                        app.card_title_input = String::new();
                    }
                    KeyCode::Tab => {
                        match &app.card_input_focus {
                            CardInputFocus::Title => {
                                app.card_input_focus = CardInputFocus::Text;
                            }
                            CardInputFocus::Text => {
                                app.card_input_focus = CardInputFocus::Title;
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if app.card_text_input != "".to_string() && app.card_title_input != "".to_string() {
                            app.add_card(app.card_title_input.to_string(), app.card_text_input.to_string());
                            app.selected_window = Selected::Side;
                            app.card_text_input = String::new();
                            app.card_title_input = String::new();
                        }
                    }
                    KeyCode::Backspace => {
                        match &app.card_input_focus {
                            CardInputFocus::Title => {
                                app.card_title_input.pop();
                            }
                            CardInputFocus::Text => {
                                app.card_text_input.pop();
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        match &app.card_input_focus {
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
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

// Ui code
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    // Layout for Main and Side blocks
    let block_layout = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    // Center Layout for pupup window 
    let center_row_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(f.size());
    let center_col_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(center_row_layout[1]);

    // Draw Side block
    let side_block;
    match app.selected_window {
        Selected::Side => {
            side_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" Selected Stack ", Style::default().fg(Color::White)))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan));
        } 
        _ => {
            side_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" Selected Stack ", Style::default().fg(Color::White)))
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::White));
        }
    }
    f.render_widget(side_block, block_layout[1]);
    
    // Side block layout
    let side_block_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(6) .vertical_margin(2)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(12), Constraint::Percentage(5), Constraint::Percentage(65), Constraint::Percentage(2)].as_ref())
        .split(block_layout[1]);

    // Side block name layout 
    let side_block_name_layout = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(43), Constraint::Percentage(13), Constraint::Percentage(1)].as_ref())
        .split(side_block_layout[1]);

    // Side block selected stack name box 
    let side_block_name_box = Block::default()
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Side block selected stack name
    let side_block_name = Block::default()
        .style(Style::default().fg(Color::White))
        .title(Span::styled(app.get_selected_name(), Style::default().add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center);

    // Side block options layout
    let side_block_options_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(4)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25)].as_ref())
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
        .title(Span::styled("a: Add Card", Style::default().add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center);
    let side_block_name_2 = Block::default()
        .title(Span::styled("s: Start Revision", Style::default().add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center);
    let side_block_name_3 = Block::default()
        .title(Span::styled("l: List Cards", Style::default().add_modifier(Modifier::BOLD)))
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
        },
        None => {}
    }

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
                .style(Style::default().fg(Color::Cyan));
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
    let stacks = List::new(stacks)
        .highlight_style(
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
    let options = Table::new(vec![
            Row::new(vec!["a: Add new", "d: Delete", "Enter: Select", "<j, k>: up, down"]).style(Style::default()),
        ])
        .widths(&[Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25)]);
    f.render_widget(options, main_block_options_layout[1]);

    // Add Stack Popub window 
    let add_stack_popup_block = Block::default()
        .style(Style::default().fg(Color::Cyan))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(" Add Stack ", Style::default().fg(Color::White)))
        .title_alignment(Alignment::Center);

    // Add Stack Popup Layout 
    let add_stack_popup_layout_row = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(50), Constraint::Percentage(17)])
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
        .constraints([Constraint::Percentage(33), Constraint::Percentage(42), Constraint::Percentage(33)])
        .split(center_col_layout[1]);

    // Add Stack Input Text 
    let add_stack_input = Paragraph::new(
        Span::from(app.stack_name_input.as_ref())
    )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    // Add Stack "name:" text
    let add_stack_input_text = Paragraph::new(
        Span::from("name:")
    )
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Right);

    // Add Stack input outline
    let add_stack_input_outline = Block::default()
        .style(Style::default().fg(Color::White)) 
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Delete Stack popup 
    let delete_stack_popup_block = Block::default()
        .style(Style::default().fg(Color::Cyan))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Delete Stack question
    let delete_stack_popup_text = Paragraph::new(
        Span::from("Are you Sure?")
    )
        .style(Style::default().fg(Color::White)) 
        .alignment(Alignment::Center);

    // Delete Stack Layout
    let delete_stack_popup_layout_row = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(90), Constraint::Percentage(5)])
        .split(center_col_layout[1]);
    let delete_stack_popup_layout_col_1 = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(30), Constraint::Percentage(30), Constraint::Percentage(15)])
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
    let delete_stack_button_text = Paragraph::new(
        Span::from("Yes")
    )
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));

    // Add card center layout 
    let add_card_center_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(49), Constraint::Percentage(25)].as_ref())
        .split(center_row_layout[1]);

    // Add card block
    let add_card_block = Block::default()
        .style(Style::default().fg(Color::Cyan))
        .borders(Borders::ALL)
        .title(Span::styled(" Add Card ", Style::default().fg(Color::White)))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    // Add card layout 
    let add_card_layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(3)
        .horizontal_margin(7)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(add_card_center_layout[1]);

    // Add card title input box
    let add_card_title_input_box = Block::default()
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // Add card text input box 
    let add_card_text_input_box = Block::default()
        .style(Style::default().fg(Color::White)) 
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

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
    let add_card_title_input_promt = Paragraph::new(
        Span::from("title: ")
    )
        .style(Style::default().add_modifier(Modifier::BOLD));

    // Add card title input box value
    let add_card_title_input_value = Paragraph::new(
        Span::from(app.card_title_input.as_ref())
    );

    // Add card text input box promt
    let add_card_text_input_promt = Paragraph::new(
        Span::from("text: ")
    )
        .style(Style::default().add_modifier(Modifier::BOLD));

    // Add card text input box value
    let add_card_text_input_value = Paragraph::new(
        Span::from(app.card_text_input.as_ref())
    )
        .wrap(Wrap { trim: true });

    // Render Popup windows
    match app.selected_window {
        Selected::StackNameInput => {
            f.render_widget(add_stack_popup_block, center_col_layout[1]);
            f.render_widget(add_stack_input_outline, add_stack_popup_input_layout[1]);
            f.render_widget(add_stack_input, add_stack_popup_layout_col_1[1]);
            f.render_widget(add_stack_input_text, add_stack_popup_layout_col_0[1]);
        },
        Selected::DeleteStackPopup => {
            f.render_widget(delete_stack_popup_block, center_col_layout[1]);
            f.render_widget(delete_stack_popup_text, delete_stack_popup_layout_col_1[1]);
            f.render_widget(delete_stack_button, delete_stack_popup_layout_col_1[2]);
            f.render_widget(delete_stack_button_text, delete_stack_button_layout[1]);
        }
        Selected::AddCard => {
            f.render_widget(add_card_block, add_card_center_layout[1]);
            f.render_widget(add_card_title_input_box, add_card_layout[0]);
            f.render_widget(add_card_text_input_box, add_card_layout[1]);
            f.render_widget(add_card_title_input_promt, add_card_title_input_layout[0]);
            f.render_widget(add_card_text_input_promt, add_card_text_input_layout[0]);
            f.render_widget(add_card_title_input_value, add_card_title_input_layout[1]);
            f.render_widget(add_card_text_input_value, add_card_text_input_layout[1]);
        }
        _ => {}
    } 
}
