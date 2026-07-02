use std::io::{self, stdout};

use ratatui::crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use ratatui::widgets::FrameExt as _;
use ratatui_explorer::{FileExplorerBuilder, Theme};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let theme = Theme::default().add_default_title();
    let mut file_explorer = FileExplorerBuilder::build_with_theme(theme)?;

    loop {
        terminal.draw(|f| {
            f.render_widget_ref(file_explorer.widget(), f.area());
        })?;

        let event = read()?;
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }

        file_explorer.handle(&event)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
