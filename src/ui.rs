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
            Constraint::Length(5),
            Constraint::Length(30),
            Constraint::Length(5),
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

    // EXPLORER
    f.render_widget_ref(fe.widget(), main_chunks[0]);

    // PREVIEW
    let block = Block::default().borders(Borders::ALL).title("Image");
    let area = block.inner(main_chunks[1]);

    // let dithered = self.render_floyd_steinberg(f);
    // f.render_widget(dithered, area);
    f.render_widget(block, main_chunks[1]);

    // let dither_placeholder = { Span::styled("DITHERED IMAGE", Style::default().fg(Color::Red)) };

    match app.show_image {
        ShowImage::Raw => app.render_resized(f, Resize::Scale(None), area),
        ShowImage::FloydSteinberg => app.render_floyd_steinberg(f, Resize::Scale(None), area),
        // ShowImage::FloydSteinberg => f.render_widget(dither_placeholder, area),
    }

    // FOOTER

    let placeholder = { Span::styled("<space> to dither it!", Style::default().fg(Color::Red)) };
    let current_keys_hint = { Span::styled("(q) to quit", Style::default().fg(Color::Red)) };
    let placeholder_footer = Paragraph::new(Line::from(current_keys_hint))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    let key_notes_footer = Paragraph::new(Line::from(placeholder))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(placeholder_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}
