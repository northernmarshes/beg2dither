use dithers::dither::{DitherMethod, dither, save_image};
use dithers::palette::ColorPalette;
use image::DynamicImage;
use image::ImageBuffer;
use image::Rgb;
use image::imageops::FilterType;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui_explorer::{FileExplorer, FileExplorerBuilder, Theme};
use ratatui_image::StatefulImage;
use ratatui_image::errors::Errors;
use ratatui_image::picker::cap_parser::QueryStdioOptions;
use ratatui_image::protocol::StatefulProtocol;
use ratatui_image::{Resize, picker::Picker};
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

pub enum ShowImage {
    Raw,
    FloydSteinberg,
    Stucki,
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
    pub should_quit: bool,
    pub path: String,
    pub algorithm: String,
    pub input_mode: InputMode,
    pub show_image: ShowImage,
    pub dithered_image: Option<RawImage>,
    pub image_source: DynamicImage,
    pub snackbar: String,
    pub picker: Picker,
    pub image_scale_state: StatefulProtocol,
    pub output_width: u32,
    pub input: String,
    pub character_index: usize,
}

impl App {
    pub fn new() -> App {
        let path: String = "./assets/dc01.JPG".to_string();
        let algorithm: String = "Raw".to_string();
        let image_source: DynamicImage = image::ImageReader::open(&path).unwrap().decode().unwrap();
        let dithered_image = None;
        let snackbar: String = "".to_string();
        let picker: Picker = Picker::from_query_stdio_with_options(QueryStdioOptions {
            terminal_background_color_osc: true,
            text_sizing_protocol: true,
            ..Default::default()
        })
        .unwrap();
        // let dithered_image:
        let image_scale_state = picker.new_resize_protocol(image_source.clone());
        let output_width: u32 = 300;
        let input = output_width.to_string();
        App {
            should_quit: false,
            path,
            algorithm,
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

    // Render resized
    pub fn render_resized(&mut self, f: &mut Frame<'_>, resize: Resize, area: Rect) {
        let state = &mut self.image_scale_state;
        let block = block("Image");
        let inner_area = block.inner(area);
        f.render_stateful_widget(StatefulImage::new().resize(resize), inner_area, state);
    }

    // Resize te preview to have nice display
    pub fn get_resized(
        &self,
        path: &str,
        width: u32,
    ) -> Result<RawImage, ratatui_image::errors::Errors> {
        let image = image::ImageReader::open(path)?.decode()?;
        let filter = FilterType::Nearest;
        let height: u32 = width * image.height() / image.width();
        let scaled = image.resize(width, height, filter);
        let buffer: Vec<u8> = scaled.into_rgb8().into_raw();
        Ok(RawImage {
            buffer,
            width,
            height,
        })
    }

    pub fn dither_it(
        &mut self,
        mut buffer: Vec<u8>,
        width: u32,
        height: u32,
    ) -> Result<RawImage, Errors> {
        dither(
            &mut buffer,
            DitherMethod::FloydSteinberg,
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

    // Render Floyd Steinberg
    pub fn render_floyd_steinberg(&mut self, f: &mut Frame<'_>, resize: Resize, area: Rect) {
        let block = block("Image");
        let inner_area = block.inner(area);

        let RawImage {
            buffer,
            width,
            height,
        } = self.get_resized(&self.path, self.output_width).unwrap();

        let RawImage {
            buffer,
            width,
            height,
        } = self.dither_it(buffer.clone(), width, height).unwrap();

        self.dithered_image = Some(RawImage {
            buffer: buffer.clone(),
            width,
            height,
        });

        let dithered: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, buffer).unwrap();
        let dynamic = DynamicImage::from(dithered);
        let dithered_protocol: &mut StatefulProtocol =
            &mut self.picker.new_resize_protocol(dynamic.clone());

        f.render_stateful_widget(
            StatefulImage::new().resize(resize),
            inner_area,
            dithered_protocol,
        );
    }

    // TODO: the whole function is redundant, merge all to render_dither(algorithm)

    // Render Stucki
    pub fn render_stucki(&mut self, f: &mut Frame<'_>, resize: Resize, area: Rect) {
        let block = block("Image");
        let inner_area = block.inner(area);

        let RawImage {
            mut buffer,
            width,
            height,
        } = self.get_resized(&self.path, self.output_width).unwrap();

        dither(
            &mut buffer,
            DitherMethod::Stucki,
            ColorPalette::Monochrome,
            width,
            height,
        );

        self.dithered_image = Some(RawImage {
            buffer: buffer.clone(),
            width,
            height,
        });

        let dithered: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, buffer).unwrap();
        let dynamic = DynamicImage::from(dithered);
        let dithered_protocol: &mut StatefulProtocol =
            &mut self.picker.new_resize_protocol(dynamic.clone());

        f.render_stateful_widget(
            StatefulImage::new().resize(resize),
            inner_area,
            dithered_protocol,
        );
    }

    // Save the dithered image
    pub fn save_dither(&mut self, image: Option<RawImage>) {
        let RawImage {
            buffer,
            width,
            height,
        } = image.unwrap();
        save_image(buffer.clone(), PathBuf::from("dithered.png"), width, height);
    }

    // Get file explorer showing only pictures and directories
    pub fn get_explorer() -> Result<FileExplorer, Box<dyn Error>> {
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

    // Update path with files with image extension
    pub fn update(&mut self, fe: &FileExplorer) {
        let image_extensions: [&str; 3] = ["jpg", "png", "JPG"];
        let img_path = &fe.current().path.display().to_string();
        let extension = Path::new(img_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("../assets/01.png");
        if image_extensions.contains(&extension) {
            self.path = img_path.clone();
            self.image_source = image::ImageReader::open(&self.path)
                .unwrap()
                .decode()
                .unwrap();
            self.image_scale_state = self.picker.new_resize_protocol(self.image_source.clone());
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
