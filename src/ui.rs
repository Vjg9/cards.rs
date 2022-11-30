use crate::state::App;
use crate::state::Selected;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
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
pub mod size_error;

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
    
    // Check if terminal is big enough
    if crossterm::terminal::size().unwrap().0 > 172 && crossterm::terminal::size().unwrap().1 > 47 {
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
    } else {
        crate::ui::size_error::render(f, app);
    }
}
