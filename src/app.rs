// use ratatui_image::{Image, Resize, picker::Picker, protocol::Protocol};
// use ratatui_image::protocol::Protocol;

pub struct App {
    // image: Protocol,
    pub title: String,
}

impl App {
    pub fn new() -> App {
        App {
            title: "ratadot".to_string(),
        }
    }
}
