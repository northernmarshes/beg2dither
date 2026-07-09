use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::FrameExt as _;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_explorer::FileExplorer;
use ratatui_image::Resize;

use crate::app::App;
use crate::app::ShowImage;

pub fn ui(f: &mut Frame, fe: &mut FileExplorer, app: &mut App) {
    // VERTICSL CHUNKS
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(30),
            Constraint::Length(3),
        ])
        .split(f.area());

    // TITLE
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let word = &app.title;
    let title = Paragraph::new(Text::styled(word, Style::default().fg(Color::Green)))
        .alignment(Alignment::Center)
        .block(title_block);

    f.render_widget(title, chunks[0]);

    // MAIN CHUNKS
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    let explorer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(30),
            Constraint::Length(3),
        ])
        .split(main_chunks[0]);

    // CURRENT ALGORITHM
    let title_block = Block::default()
        .borders(Borders::ALL)
        .title("Current Algorithm");
    let word = app.algorithm.to_string();
    let title = Paragraph::new(Text::styled(word, Style::default().fg(Color::Green)))
        .alignment(Alignment::Center)
        .block(title_block);

    f.render_widget(title, explorer_chunks[0]);

    // SIZE INPUT

    let title_block = Block::default().borders(Borders::ALL).title("Output size");
    let word = "400 x 300".to_string();
    let title = Paragraph::new(Text::styled(word, Style::default().fg(Color::Green)))
        .alignment(Alignment::Center)
        .block(title_block);

    f.render_widget(title, explorer_chunks[2]);

    // EXPLORER
    f.render_widget_ref(fe.widget(), explorer_chunks[1]);

    // PREVIEW
    let block = Block::default().borders(Borders::ALL).title("Image");
    let area = block.inner(main_chunks[1]);
    f.render_widget(block, main_chunks[1]);

    match app.show_image {
        ShowImage::Raw => app.render_resized(f, Resize::Scale(None), area),
        ShowImage::FloydSteinberg => app.render_floyd_steinberg(f, Resize::Scale(None), area),
        ShowImage::Stucki => app.render_stucki(f, Resize::Scale(None), area),
    }

    // FOOTER

    let current_keys_hint = {
        Span::styled(
            "Raw (1) | Floyd Steinberg (2) | Stucki (3) | Save (s) | Exit (q)",
            Style::default().fg(Color::Red),
        )
    };
    let placeholder_footer = Paragraph::new(Line::from(current_keys_hint))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[2]);

    f.render_widget(placeholder_footer, footer_chunks[0]);
}
