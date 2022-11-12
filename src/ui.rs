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
    text::{Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Table, Row},
    Frame, Terminal
};
use crate::db::init;
use crate::db::stack::{Stack, get_all};
use rusqlite::{Connection};

struct App {
    items: Vec<Stack>,
    state: ListState,
    db: Result<Connection, rusqlite::Error>,
}

impl App {
    fn new() -> App {
       App {
            items: vec![],
            state: ListState::default(),
            db: init("./dev.db"),
       } 
    } 

    fn get_items(&mut self) {
        self.items = get_all(self.db.as_ref().unwrap());
    }

    fn get_selected(&mut self) -> String {
        let _i = match self.state.selected() {
            Some(i) => {
                return self.items[i].name.to_string()
            },
            None => {
                return "".to_string()
            }
        };
    }

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
    if app.items.len() >= 0 {
        app.state.select(Some(0));
    }
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Up => app.next(),
                KeyCode::Down => app.back(),
                KeyCode::Char('k') => app.next(),
                KeyCode::Char('j') => app.back(),
                _ => {}
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

    // Draw Main block
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(" Stacks ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(main_block, block_layout[0]);

    // Draw Side block
    let side_block = Block::default()
        .borders(Borders::ALL)
        .title(" Selected Stack ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(side_block, block_layout[1]);
    
    // Main block layout
    let main_block_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(block_layout[0]);

    // Side block layout
    let side_block_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(2)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(10), Constraint::Percentage(70)].as_ref())
        .split(block_layout[1]);

    // Side block selected stack name
    let side_block_name = Block::default()
        .title(Span::styled(app.get_selected(), Style::default().add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center);
    f.render_widget(side_block_name, side_block_layout[1]);

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
    f.render_widget(options, main_block_options_layout[1]) }
