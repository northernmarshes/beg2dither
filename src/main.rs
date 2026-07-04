use crate::app::App;
use crate::ui::ui;
use ratatui::crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::error::Error;
use std::ffi::OsStr;
use std::io::{self, stdout};
use std::path::Path;

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    let mut file_explorer = App::get_explorer().unwrap();
    loop {
        terminal.draw(|f| ui(f, &mut file_explorer, app))?;

        let event = read()?;
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                break Ok(false);
            }
            // placeholder keybinding
            if key.code == KeyCode::Char('w') {
                break Ok(false);
            }
        }
        file_explorer.handle(&event)?;

        // TODO: pass only files' path not folders
        let image_extensions = ["jpg", "png"];
        let img_path = file_explorer.current().path.display().to_string();
        let extension = get_extension_from_filename(&img_path).unwrap_or("../assets/01.png");
        if image_extensions.contains(&extension) {
            app.path = img_path;
        }
    }
}
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
