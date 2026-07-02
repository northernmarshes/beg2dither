use std::error::Error;
use std::io::{self, stdout};

use ratatui::crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use ratatui::widgets::FrameExt as _;
use ratatui_explorer::{FileExplorerBuilder, Theme};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let res = run(&mut terminal);
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    let layout = Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);
    let theme = Theme::default().add_default_title();
    let mut file_explorer = FileExplorerBuilder::build_with_theme(theme)?;
    loop {
        terminal.draw(|f| {
            let chunks = layout.split(f.area());
            f.render_widget_ref(file_explorer.widget(), chunks[0]);
        })?;

        let event = read()?;
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                break Ok(false);
            }
        }

        file_explorer.handle(&event)?;
    }
}
