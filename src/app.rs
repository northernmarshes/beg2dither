use ratatui::layout::Size;
use ratatui_explorer::{FileExplorer, FileExplorerBuilder, Theme};
use ratatui_image::picker::cap_parser::QueryStdioOptions;
use ratatui_image::{Resize, picker::Picker, protocol::Protocol};
use std::error::Error;

pub struct App {
    pub title: String,
    pub path: String,
    pub image: Protocol,
}

fn size() -> Size {
    Size::new(30, 16)
}

impl App {
    pub fn new() -> App {
        let title = "RATADOT".to_string();
        let path: String = "./assets/01.png".to_string();
        let image = App::render(&path).unwrap();
        App { title, path, image }
    }

    pub fn render(path: &String) -> Result<Protocol, ratatui_image::errors::Errors> {
        let image_source = image::ImageReader::open(path)?.decode()?;

        let picker = Picker::from_query_stdio_with_options(QueryStdioOptions {
            terminal_background_color_osc: true,
            text_sizing_protocol: true,
            ..Default::default()
        })?;

        let image_static = picker
            .new_protocol(image_source.clone(), size(), Resize::Fit(None))
            .expect("demo gets a protocol from image");
        Ok(image_static)
    }

    /// Get file explorer showing only pictures and directories
    pub fn get_explorer() -> Result<FileExplorer, Box<dyn Error>> {
        const SUPPORTED_FORMATS: [&str; 2] = ["jpg", "png"];
        let theme = Theme::default().add_default_title();
        let mut file_explorer = FileExplorerBuilder::build_with_theme(theme)?;
        file_explorer.set_filter_map(|file| {
            let keep = match file.path.extension() {
                Some(extension) => {
                    let extension = extension.to_str().unwrap_or_default();
                    SUPPORTED_FORMATS.contains(&extension)
                }
                None => file.is_dir,
            };
            if keep { Some(file) } else { None }
        })?;
        Ok(file_explorer)
    }
}
