use crate::app::App;
use crate::ui::ui;
use ratatui::crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use ratatui_explorer::{FileExplorerBuilder, Theme};
use std::error::Error;
use std::io::{self, stdout};

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();
    let res = run(&mut terminal, &mut app);
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    let theme = Theme::default().add_default_title();
    let mut file_explorer = FileExplorerBuilder::build_with_theme(theme)?;
    loop {
        terminal.draw(|f| ui(f, &mut file_explorer, app))?;

        let event = read()?;
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                break Ok(false);
            }
        }
        file_explorer.handle(&event)?;
    }
}
