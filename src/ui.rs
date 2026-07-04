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
use ratatui_image::Image;
// use ratatui_image::{Image, Resize, picker::Picker};
// use ratatui::layout::Size;

use crate::app::App;

pub fn ui(f: &mut Frame, fe: &mut FileExplorer, app: &App) {
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
    // Trying to resize the image
    // let picker = Picker::halfblocks();
    // let dyn_img = image::ImageReader::open(&app.path)
    //     .unwrap()
    //     .decode()
    //     .unwrap();
    // let font_size = picker.font_size();
    // let size = Size::new(
    //     dyn_img.width().div_ceil(font_size.width as u32) as u16,
    //     dyn_img.height().div_ceil(font_size.height as u32) as u16,
    // );
    // let protocol = picker
    //     .new_protocol(dyn_img, size, Resize::Fit(None))
    //     .unwrap();

    let protocol = App::render(&app.path).unwrap();
    let image = Image::new(&protocol);
    f.render_widget(image, main_chunks[1]);

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
