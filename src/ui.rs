use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, slice::SliceIndex};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Span,
    style::{Style, Color, Modifier},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

struct App<'a> {
   items: Vec<&'a str>,
   selected: i32,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
       App {
            items: vec![
                "sus",
                "hello",
                "sussy",
                "sus",
                "hello",
                "sussy",
                "sus",
                "hello",
                "sussy",
            ],
            selected: 0,
       } 
    } 

    fn right(&mut self) {
        if self.selected < i32::try_from(self.items.len()).unwrap() - 1 {
            self.selected += 1
        } else {
            self.selected = 0
        }
    }

    fn left(&mut self) {
        if self.selected == 0 {
            self.selected = i32::try_from(self.items.len()).unwrap() - 1
        } else {
            self.selected -= 1
        }
    }

    fn up(&mut self) {
        if self.selected <= 2 {
            self.selected = self.selected
        } else {
            self.selected -= 3
        }
    }

    fn down(&mut self) {
        if self.selected > i32::try_from(self.items.len()).unwrap() - 4 {
            self.selected = self.selected
        } else {
            self.selected += 3
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
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.right(),
                KeyCode::Left => app.left(),
                KeyCode::Up => app.up(),
                KeyCode::Down => app.down(),
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

    // Block columns
    let main_block_columns = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(block_layout[0]);

    // Block row 1
    let main_block_row_1 = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(main_block_columns[0]);

    // Block row 2
    let main_block_row_2 = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(main_block_columns[1]);

    // Block row 3
    let main_block_row_3 = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(main_block_columns[2]);

    let selected_style = Style::default().fg(Color::Cyan);
    let normal_style = Style::default();

    let mut index = 0;

    for item in app.items.iter() {
        let stack_layout: Vec<Rect>;
        let style: Style;
        if app.selected.to_string() == index.to_string() {
            style = selected_style;
        } else {
            style = normal_style;
        }
        match index {
            0 | 1 | 2 => {
                let stack_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(style);
                f.render_widget(stack_block, main_block_row_1[index]);

                // Stack layout
                stack_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .horizontal_margin(3)
                    .vertical_margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(main_block_row_1[index]);
            }
            3 | 4 | 5 => {
                let stack_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(style);
                f.render_widget(stack_block, main_block_row_2[index - 3]);

                // Stack layout
                stack_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .horizontal_margin(3)
                    .vertical_margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(main_block_row_2[index - 3]);
            }
            6 | 7 | 8 => {
                let stack_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(style);
                f.render_widget(stack_block, main_block_row_3[index - 6]);

                // Stack layout
                stack_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .horizontal_margin(3)
                    .vertical_margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(main_block_row_3[index - 6]);
            }
            _ => {
                stack_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .horizontal_margin(3)
                    .vertical_margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(main_block_row_1[0]);
            }
        }

    
        // Stack text
        let stack_text_block = Block::default()
            .title(Span::styled(
                    item.to_string(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center);
        f.render_widget(stack_text_block, stack_layout[1]);
        index += 1;
    }
}
