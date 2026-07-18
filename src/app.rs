use dithers::{
    dither::{DitherMethod, dither, save_image},
    palette::ColorPalette,
};
use image::{DynamicImage, ImageBuffer, Rgb, imageops::FilterType};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};
use ratatui_explorer::{FileExplorer, FileExplorerBuilder, Theme};
use ratatui_image::{
    Resize, StatefulImage, errors::Errors, picker::Picker, picker::cap_parser::QueryStdioOptions,
    protocol::StatefulProtocol,
};
use std::{env, error::Error, ffi::OsStr, path::Path, path::PathBuf};

pub enum ShowImage {
    Raw,
    FloydSteinberg,
    Stucki,
    Jarvis,
    Atkinson,
    None,
}

pub enum Display {
    Yes,
    No,
}

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone)]
pub struct RawImage {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct App {
    pub path: Option<String>,
    pub display: Display,
    pub input_mode: InputMode,
    pub show_image: ShowImage,
    pub dithered_image: Option<RawImage>,
    pub image_source: Option<DynamicImage>,
    pub picker: Option<Picker>,
    pub image_scale_state: Option<StatefulProtocol>,
    pub output_width: u32,
    pub input: String,
    pub character_index: usize,
    pub algorithm_bar: String,
    pub snackbar: String,
}

impl App {
    pub fn new() -> App {
        let path: Option<String> = Some(
            env::home_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                // TODO: Add default path
                .unwrap(),
        );
        let image_source = None;
        let picker = None;
        let image_scale_state = None;
        let algorithm_bar: String = "No image".to_string();
        let dithered_image = None;
        let snackbar: String = "".to_string();
        let output_width: u32 = 300;
        let input = output_width.to_string();
        App {
            display: Display::No,
            path,
            algorithm_bar,
            input_mode: InputMode::Normal,
            show_image: ShowImage::Raw,
            snackbar,
            dithered_image,
            image_source,
            picker,
            image_scale_state,
            input,
            output_width,
            character_index: 3,
        }
    }

    // Resize the preview
    pub fn get_resized(
        &self,
        path: &str,
        width: u32,
    ) -> Result<RawImage, ratatui_image::errors::Errors> {
        let image = image::ImageReader::open(path)?.decode()?;
        let filter = FilterType::Nearest;
        let height: u32 = width * image.height() / image.width();
        let scaled = image.resize_exact(width, height, filter);
        let buffer: Vec<u8> = scaled.to_rgb8().into_raw();
        Ok(RawImage {
            buffer,
            width,
            height,
        })
    }

    // Dither with chosen algorithm
    pub fn dither_it(
        &mut self,
        mut buffer: Vec<u8>,
        width: u32,
        height: u32,
    ) -> Result<RawImage, Errors> {
        let dither_type = match self.show_image {
            //TODO: Raw should be somehow omitted, doesn't need dither
            ShowImage::Raw => DitherMethod::None,
            ShowImage::FloydSteinberg => DitherMethod::FloydSteinberg,
            ShowImage::Stucki => DitherMethod::Stucki,
            ShowImage::Jarvis => DitherMethod::Jarvis,
            ShowImage::Atkinson => DitherMethod::Atkinson,
            ShowImage::None => DitherMethod::None,
        };

        dither(
            &mut buffer,
            dither_type,
            ColorPalette::Monochrome,
            width,
            height,
        );
        Ok(RawImage {
            buffer,
            width,
            height,
        })
    }

    // Render RAW resized
    pub fn render_resized(&mut self, f: &mut Frame<'_>, resize: Resize, area: Rect) {
        let state = self.image_scale_state.as_mut().unwrap();
        let block = block("Image");
        let inner_area = block.inner(area);
        f.render_stateful_widget(StatefulImage::new().resize(resize), inner_area, state);
    }

    // Render DITHERED resized
    pub fn render_dithered(&mut self, f: &mut Frame<'_>, resize: Resize, area: Rect) {
        let block = block("Image");
        let inner_area = block.inner(area);
        let path = self.path.clone();

        // Resize
        let RawImage {
            buffer,
            width,
            height,
        } = self.get_resized(&path.unwrap(), self.output_width).unwrap();

        // Dither
        let RawImage {
            buffer,
            width,
            height,
            // This panics when there is a problem with decoding, need better error handling
        } = self.dither_it(buffer, width, height).unwrap();

        // Update App
        self.dithered_image = Some(RawImage {
            buffer: buffer.clone(),
            width,
            height,
        });

        let dithered: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, buffer).unwrap();
        let dynamic = DynamicImage::from(dithered);
        let dithered_protocol: &mut StatefulProtocol = &mut self
            .picker
            .as_mut()
            .unwrap()
            .new_resize_protocol(dynamic.clone());

        f.render_stateful_widget(
            StatefulImage::new().resize(resize),
            inner_area,
            dithered_protocol,
        );
    }

    // Save DITHERED image
    pub fn save_dither(&mut self, image: RawImage) {
        let RawImage {
            buffer,
            width,
            height,
        } = image;
        self.snackbar = "Image saved as 'b2d_output.png' :3".to_string();
        let path_exists = PathBuf::from("b2d_output.png").exists();
        if path_exists {
            self.snackbar = "File 'b2d_output.png' already exists".to_string();
        } else {
            save_image(
                buffer.clone(),
                PathBuf::from("b2d_output.png"),
                width,
                height,
            );
        }
    }

    // Get file explorer showing only pictures and directories
    pub fn get_explorer() -> Result<FileExplorer, Box<dyn Error>> {
        // TODO: extensions should be moved to App struct as a vec
        const SUPPORTED_FORMATS: [&str; 3] = ["jpg", "png", "JPG"];
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

    // Update path with files bearing image extension
    pub fn update(&mut self, fe: &FileExplorer) {
        // TODO: extensions should be moved to App struct as a vec
        let image_extensions: [&str; 3] = ["jpg", "png", "JPG"];
        let img_path = &fe.current().path.display().to_string();
        let extension = Path::new(img_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("/");
        if image_extensions.contains(&extension) {
            self.path = Some(img_path.clone()); // updating the path
            self.display = Display::Yes;
            let picker: Picker = Picker::from_query_stdio_with_options(QueryStdioOptions {
                terminal_background_color_osc: true,
                text_sizing_protocol: true,
                ..Default::default()
            })
            .unwrap();
            self.picker = Some(picker.clone()); // updating the picker
            self.image_source = Some(
                image::ImageReader::open(img_path.clone())
                    .unwrap()
                    .decode()
                    .unwrap_or_default(),
            );
            let image = self.image_source.clone();
            self.image_scale_state = Some(picker.new_resize_protocol(image.unwrap()));
        }
        if fe.current().path.is_dir() {
            self.display = Display::No;
            self.algorithm_bar = "None".to_string();
        }
    }

    // Input section
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn is_valid(&self) -> bool {
        self.input.bytes().all(|c| c.is_ascii_digit())
    }

    pub fn submit_message(&mut self) {
        let isvalid = self.is_valid();
        if isvalid {
            let input = self.input.parse().unwrap_or(300);
            if input > 30 && input < 9000 {
                self.output_width = self.input.parse().unwrap_or(300);
                self.snackbar = "Image successfully resized".to_string()
            } else {
                self.snackbar = "The value has to be between 30 and 9000!".to_string();
            }
        } else {
            self.snackbar = "The value has to be a number!".to_string();
        };
        self.input_mode = InputMode::Normal;
    }
}

fn block(name: &str) -> Block<'_> {
    Block::default().borders(Borders::ALL).title(name)
}
