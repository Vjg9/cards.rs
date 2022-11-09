use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    text::Span,
    style::{Style, Color, Modifier},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

// Run the ui
pub fn run_ui() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

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
fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

// Ui code
fn ui<B: Backend>(f: &mut Frame<B>) {

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

    // Stack
    let stack_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(stack_block, main_block_row_1[0]);

    // Stack layout
    let stack_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_block_row_1[0]);

    // Stack text
    let stack_text_block = Block::default()
        .title(Span::styled(
                "text",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    f.render_widget(stack_text_block, stack_layout[1]);
}
