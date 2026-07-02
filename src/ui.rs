use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::FrameExt as _;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_explorer::FileExplorer;

pub fn ui(f: &mut Frame, fe: &mut FileExplorer) {
    // CHUNKS

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

    let title = Paragraph::new(Text::styled("RATADOT", Style::default().fg(Color::Green)))
        .block(title_block);

    f.render_widget(title, chunks[0]);

    // MAIN CHUNKS
    // EXPLORER
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    f.render_widget_ref(fe.widget(), main_chunks[0]);

    // PREVIEW

    let lorem = { Span::styled("Lorem ipsum", Style::default().fg(Color::Red)) };

    let placeholder_main =
        Paragraph::new(Line::from(lorem)).block(Block::default().borders(Borders::ALL));

    f.render_widget(placeholder_main, main_chunks[1]);

    // FOOTER

    let placeholder = { Span::styled("<space> to dither it!", Style::default().fg(Color::Red)) };

    let current_keys_hint = { Span::styled("(q) to quit", Style::default().fg(Color::Red)) };

    let placeholder_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let key_notes_footer =
        Paragraph::new(Line::from(placeholder)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(placeholder_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}
