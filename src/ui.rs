use crate::config;
use crate::db::card::Card;
use crate::db::stack::Stack;
use crate::db::{card, init, stack};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::Connection;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::ListState,
    Frame, Terminal,
};

// Window modules
pub mod main;
pub mod side;
pub mod stack_name_input;
pub mod delete_stack_popup;
pub mod edit_stack_popup;
pub mod add_card;
pub mod card_list;
pub mod delete_card;
pub mod edit_card;
pub mod revision_title;
pub mod revision_text;
pub mod config_options;

// Selected Window Enum
pub enum Selected {
    Main,
    Side,
    StackNameInput,
    DeleteStackPopup,
    EditStackPopup,
    AddCard,
    CardList,
    DeleteCard,
    EditCard,
    RevisionTitle,
    RevisionText,
    ConfigOptions,
}

// Card Input Focus Enum
pub enum CardInputFocus {
    Title,
    Text,
}

// Config focus
pub enum ConfigFocus {
    DbFile,
    HighlightColor,
}

pub struct App {
    pub items: Vec<Stack>,
    pub state: ListState,
    pub db: Result<Connection, rusqlite::Error>,
    pub selected_window: Selected,
    pub stack_name_input: String,
    pub card_title_input: String,
    pub card_text_input: String,
    pub card_input_focus: CardInputFocus,
    pub cards: Vec<Card>,
    pub cards_state: ListState,
    pub revision_index: usize,
    pub config_input_1: String,
    pub config_input_2: String,
    pub config_input_focus: ConfigFocus,
    pub highlight_color: u8,
}

impl App {
    fn new() -> App {
        App {
            items: vec![],
            state: ListState::default(),
            db: init(format!("{}", config::get_db_file()).as_str()),
            selected_window: Selected::Main,
            stack_name_input: String::new(),
            card_title_input: String::new(),
            card_text_input: String::new(),
            card_input_focus: CardInputFocus::Title,
            cards: vec![],
            cards_state: ListState::default(),
            revision_index: 0,
            config_input_1: String::new(),
            config_input_2: String::new(),
            config_input_focus: ConfigFocus::DbFile,
            highlight_color: config::get_highlight_color(),
        }
    }

    // Edit card
    fn edit_card(&mut self) {
        let id = self.get_selected_card_id();
        let title = &self.card_title_input;
        let text = &self.card_text_input;
        card::edit(
            self.db.as_ref().unwrap(),
            id,
            title.to_string(),
            text.to_string(),
        )
    }

    // Delete card
    fn delete_card(&mut self) {
        let id = self.get_selected_card_id();
        card::delete(self.db.as_ref().unwrap(), id);
    }

    // Get selected card id
    fn get_selected_card_id(&mut self) -> i32 {
        let _i = match self.cards_state.selected() {
            Some(i) => return self.cards[i].id,
            None => return 0,
        };
    }

    // List cards
    fn list_cards(&mut self) {
        let stack_id = self.get_selected_id();
        self.cards = card::list(self.db.as_ref().unwrap(), stack_id);
    }

    // Add card
    fn add_card(&mut self, title: String, text: String) {
        let stack_id = self.get_selected_id();
        card::add(self.db.as_ref().unwrap(), stack_id, title, text);
    }

    // Next card
    fn next_card(&mut self) {
        if self.cards.len() > 0 {
            let i = match self.cards_state.selected() {
                Some(i) => {
                    if i >= self.cards.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.cards_state.select(Some(i));
        }
    }

    // Previous card
    fn back_card(&mut self) {
        if self.cards.len() > 0 {
            let i = match self.cards_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.cards.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.cards_state.select(Some(i));
        }
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

    // Edit stack
    fn edit_stack(&mut self) {
        let id = self.get_selected_id();
        let name = &self.stack_name_input;
        stack::edit(self.db.as_ref().unwrap(), id, name.to_string());
    }

    // Get id from selected stack
    fn get_selected_id(&mut self) -> i32 {
        let _i = match self.state.selected() {
            Some(i) => return self.items[i].id,
            None => return 0,
        };
    }

    // Get name from selected stack
    fn get_selected_name(&mut self) -> String {
        let _i = match self.state.selected() {
            Some(i) => return self.items[i].name.to_string(),
            None => return "".to_string(),
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
                    _ => crate::ui::main::handle_events(key.code, &mut app),
                },
                Selected::Side => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => crate::ui::side::handle_events(key.code, &mut app), 
                },
                Selected::StackNameInput => crate::ui::stack_name_input::handle_events(key.code, &mut app),
                Selected::DeleteStackPopup => crate::ui::delete_stack_popup::handle_events(key.code, &mut app),
                Selected::EditStackPopup => crate::ui::edit_stack_popup::handle_events(key.code, &mut app),
                Selected::AddCard => crate::ui::add_card::handle_events(key.code, &mut app),
                Selected::CardList => crate::ui::card_list::handle_events(key.code, &mut app),
                Selected::DeleteCard => crate::ui::delete_card::handle_events(key.code, &mut app), 
                Selected::EditCard => crate::ui::edit_card::handle_events(key.code, &mut app),
                Selected::RevisionTitle => crate::ui::revision_title::handle_events(key.code, &mut app),
                Selected::RevisionText => crate::ui::revision_text::handle_events(key.code, &mut app),
                Selected::ConfigOptions => crate::ui::config_options::handle_events(key.code, &mut app),
            }
        }
    }
}

// Ui code
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    
    // Render Main window
    crate::ui::main::render(f, app);

    // Render Side window 
    crate::ui::side::render(f, app);

    // Render Popup windows
    match app.selected_window {
        Selected::StackNameInput => crate::ui::stack_name_input::render(f, app),
        Selected::DeleteStackPopup => crate::ui::delete_stack_popup::render(f, app),
        Selected::AddCard => crate::ui::add_card::render(f, app),
        Selected::CardList => crate::ui::card_list::render(f, app),
        Selected::DeleteCard => crate::ui::delete_card::render(f, app),
        Selected::RevisionTitle => crate::ui::revision_title::render(f, app),
        Selected::RevisionText => crate::ui::revision_text::render(f, app),
        Selected::EditStackPopup => crate::ui::edit_stack_popup::render(f, app),
        Selected::EditCard => crate::ui::edit_card::render(f, app),
        Selected::ConfigOptions => crate::ui::config_options::render(f, app),
        _ => {}
    }
}
