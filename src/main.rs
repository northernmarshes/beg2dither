use crate::app::{App, Display, InputMode, ShowImage};
use crate::ui::ui;
use crossterm::event::KeyEventKind;
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
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        break Ok(false);
                    }
                    KeyCode::Char('s') => {
                        let image = app.dithered_image.clone();
                        match image {
                            Some(image) => app.save_dither(image),
                            None => {
                                app.snackbar = "To save choose a dithering algorithm!".to_string()
                            }
                        }
                    }
                    KeyCode::Char('r') => match app.display {
                        Display::Yes => {
                            app.input_mode = InputMode::Editing;
                            app.snackbar = "Please insert pixel width".to_string();
                        }
                        Display::No => app.snackbar = "Chose image to resize".to_string(),
                    },
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('1') => {
                        app.algorithm_bar = "Raw".to_string();
                        app.show_image = ShowImage::Raw;
                        app.dithered_image = None;
                    }
                    KeyCode::Char('2') => {
                        app.algorithm_bar = "Floyd Steinberg".to_string();
                        app.show_image = ShowImage::FloydSteinberg;
                    }
                    KeyCode::Char('3') => {
                        app.algorithm_bar = "Stucki".to_string();
                        app.show_image = ShowImage::Stucki;
                    }
                    KeyCode::Char('4') => {
                        app.algorithm_bar = "Jarvis".to_string();
                        app.show_image = ShowImage::Jarvis;
                    }
                    KeyCode::Char('5') => {
                        app.algorithm_bar = "Atkinson".to_string();
                        app.show_image = ShowImage::Atkinson;
                    }
                    KeyCode::Char('6') => {
                        app.algorithm_bar = "None".to_string();
                        app.show_image = ShowImage::None;
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_message(),
                    KeyCode::Char('r') => app.submit_message(),
                    KeyCode::Char(to_insert) => app.enter_char(to_insert),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    _ => {}
                },
                InputMode::Editing => {}
            }
        }
        match app.input_mode {
            InputMode::Normal => file_explorer.handle(&event)?,
            InputMode::Editing => {}
        }
        app.update(&file_explorer);
    }
}
