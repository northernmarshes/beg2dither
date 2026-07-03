// use image;
use ratatui::layout::Size;
use ratatui_image::{Resize, picker::Picker, protocol::Protocol};

use ratatui_image::picker::cap_parser::QueryStdioOptions;
use std::env;

pub struct App {
    pub title: String,
    pub image: Protocol,
}

fn size() -> Size {
    Size::new(30, 16)
}

impl App {
    pub fn new() -> App {
        let title = "RATADOT".to_string();
        let image = App::render().unwrap();
        App { title, image }
    }

    pub fn render() -> Result<Protocol, ratatui_image::errors::Errors> {
        let image = if env::args().any(|arg| arg == "--tmp-demo-ready") {
            "./assets/02.png"
        } else {
            "./assets/01.png"
        };

        let image_source = image::ImageReader::open(image)?.decode()?;

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
}
