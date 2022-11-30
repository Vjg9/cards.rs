use crate::ui::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, _app: &mut App) {
    // Center layout
    let center_col_layout = Layout::default()
        .vertical_margin(1)
        .horizontal_margin(2)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(20), Constraint::Percentage(40)])
        .split(f.size());
    let center_row_layout = Layout::default()
        .constraints([Constraint::Percentage(40), Constraint::Percentage(20), Constraint::Percentage(20)])
        .split(center_col_layout[1]);

    // Block 
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(" Error ", Style::default().add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Indexed(1)));

    // Promt layout
    let promt_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(center_row_layout[1]);

    // Promt
    let promt = Paragraph::new(
        Span::styled("SizeError: Not enough space to render widgets", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
    ) 
        .alignment(Alignment::Center);

    // Render 
    f.render_widget(block, center_col_layout[1]);
    f.render_widget(promt, promt_layout[1])
}
