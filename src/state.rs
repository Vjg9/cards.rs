use crate::config;
use crate::db::card::Card;
use crate::db::stack::Stack;
use crate::db::{card, init, stack};
use rusqlite::Connection;
use tui::widgets::ListState;

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
    pub fn new() -> App {
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
    pub fn edit_card(&mut self) {
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
    pub fn delete_card(&mut self) {
        let id = self.get_selected_card_id();
        card::delete(self.db.as_ref().unwrap(), id);
    }

    // Get selected card id
    pub fn get_selected_card_id(&mut self) -> i32 {
        let _i = match self.cards_state.selected() {
            Some(i) => return self.cards[i].id,
            None => return 0,
        };
    }

    // List cards
    pub fn list_cards(&mut self) {
        let stack_id = self.get_selected_id();
        self.cards = card::list(self.db.as_ref().unwrap(), stack_id);
    }

    // Add card
    pub fn add_card(&mut self, title: String, text: String) {
        let stack_id = self.get_selected_id();
        card::add(self.db.as_ref().unwrap(), stack_id, title, text);
    }

    // Next card
    pub fn next_card(&mut self) {
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
    pub fn back_card(&mut self) {
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
    pub fn get_items(&mut self) {
        self.items = stack::get_all(self.db.as_ref().unwrap());
    }

    // Add stack
    pub fn add_stack(&mut self, name: String) {
        stack::add(self.db.as_ref().unwrap(), name);
    }

    // Delete stack
    pub fn delete_stack(&mut self, id: i32) {
        stack::delete(self.db.as_ref().unwrap(), id);
    }

    // Edit stack
    pub fn edit_stack(&mut self) {
        let id = self.get_selected_id();
        let name = &self.stack_name_input;
        stack::edit(self.db.as_ref().unwrap(), id, name.to_string());
    }

    // Get id from selected stack
    pub fn get_selected_id(&mut self) -> i32 {
        let _i = match self.state.selected() {
            Some(i) => return self.items[i].id,
            None => return 0,
        };
    }

    // Get name from selected stack
    pub fn get_selected_name(&mut self) -> String {
        let _i = match self.state.selected() {
            Some(i) => return self.items[i].name.to_string(),
            None => return "".to_string(),
        };
    }

    // Select next stack
    pub fn next(&mut self) {
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
    pub fn back(&mut self) {
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
