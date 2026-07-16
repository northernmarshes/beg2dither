use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Position},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, FrameExt as _, Paragraph},
};
use ratatui_explorer::FileExplorer;
use ratatui_image::Resize;

use crate::app::App;
use crate::app::Display;
use crate::app::InputMode;
use crate::app::ShowImage;

pub fn ui(f: &mut Frame, fe: &mut FileExplorer, app: &mut App) {
    // VERTICAL CHUNKS
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(50),
            Constraint::Length(3),
            Constraint::Length(4),
        ])
        .split(f.area());

    // MAIN CHUNKS
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[0]);

    let explorer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // current algorithm
            Constraint::Length(41), // file explorer
            Constraint::Length(3),  // width
        ])
        .split(main_chunks[0]);

    // CURRENT ALGORITHM
    let title_block = Block::default()
        .borders(Borders::ALL)
        .title("Current Algorithm");
    let word = app.algorithm_bar.to_string();
    let title = Paragraph::new(Text::styled(word, Style::default().fg(Color::Green)))
        .alignment(Alignment::Center)
        .block(title_block);
    f.render_widget(title, explorer_chunks[0]);

    // WIDTH INPUT
    let input = Paragraph::new(app.input.to_string())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Width (r)"));
    f.render_widget(input, explorer_chunks[2]);
    match app.input_mode {
        InputMode::Normal => {}
        #[expect(clippy::cast_possible_truncation)]
        InputMode::Editing => f.set_cursor_position(Position::new(
            explorer_chunks[2].x + app.character_index as u16 + 1,
            explorer_chunks[2].y + 1,
        )),
    }

    // EXPLORER
    f.render_widget_ref(fe.widget(), explorer_chunks[1]);

    // PREVIEW
    let block = Block::default().borders(Borders::ALL).title("Image");
    let area = block.inner(main_chunks[1]);
    f.render_widget(block, main_chunks[1]);

    match app.display {
        Display::Yes => match app.show_image {
            ShowImage::Raw => app.render_resized(f, Resize::Scale(None), area),
            _ => app.render_dithered(f, Resize::Scale(None), area),
        },
        Display::No => {}
    }

    // SNACKBAR
    let snackbar = { Span::styled(app.snackbar.to_string(), Style::default().fg(Color::Green)) };
    let snackbar_paragraph = Paragraph::new(Line::from(snackbar))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(snackbar_paragraph, chunks[1]);

    // FOOTER
    let algorithms = Span::styled(
        "Raw (1) | Floyd Steinberg (2) | Stucki (3) | Jarvis (4)| Atkinson (5) | None (6)",
        Style::default(),
    );
    let functions = Span::styled("Resize (r) | Save (s) | Exit (q)", Style::default());
    let controls: Vec<Line<'_>> = vec![algorithms.into(), functions.into()];
    f.render_widget(
        Paragraph::new(controls)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        chunks[2],
    );
}
