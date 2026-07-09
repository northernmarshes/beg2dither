use crate::app::{App, InputMode, ShowImage};
use crate::ui::ui;
use ratatui::crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::error::Error;
use std::io::{self, stdout};

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
                app.should_quit = true;
                break Ok(false);
            }
            if key.code == KeyCode::Char('s') {
                app.save_dither(app.dithered_image.clone());
            }
            if key.code == KeyCode::Char('e') {
                app.input_mode = InputMode::Editing;
            }
            if key.code == KeyCode::Esc {
                app.input_mode = InputMode::Normal;
            }
            if key.code == KeyCode::Char('1') {
                app.algorithm = "Raw".to_string();
                app.show_image = ShowImage::Raw;
            }
            if key.code == KeyCode::Char('2') {
                app.algorithm = "Floyd Steinberg".to_string();
                app.show_image = ShowImage::FloydSteinberg;
            }
            if key.code == KeyCode::Char('3') {
                app.algorithm = "Stucki".to_string();
                app.show_image = ShowImage::Stucki;
            }
        }
        file_explorer.handle(&event)?;
        app.update(&file_explorer);
    }
}
