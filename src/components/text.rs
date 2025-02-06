use std::{cmp::max, io::Read};

use fontdue::{self, FontSettings};

use crate::{draw::Draw, size::Size, Drawable};

#[derive(Default, Clone)]
pub struct Text {
    //pub(crate) widget: widget::Widget,
    pub(crate) color: u32,
    pub(crate) text: String,
    pub(crate) size: usize,
    pub(crate) font: Option<fontdue::Font>,
}

impl Draw for Text {
    fn draw(&self, canvas: &mut crate::canvas::Canvas, x: usize, y: usize) {
        let (mut sx, mut sy) = (0usize, 0usize);

        for i in self.text.chars() {
            let (metrics, character) = self.font.as_ref().unwrap().rasterize(i, self.size as f32);

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    canvas.set_pixel(x, y, self.color * (character[y * metrics.width + x] as u32));
                }
            }

            sx += metrics.width;
        }
    }
}

impl Size for Text {
    fn set_size(&mut self, x: usize, y: usize) {
        unreachable!();
    }

    fn size(&self) -> (usize, usize) {
        let (mut width, mut height) = (0usize, 0usize);

        for i in self.text.chars() {
            let metrics = self.font.as_ref().unwrap().metrics(i, self.size as f32);

            // TODO: Advanced size calculation
            width += metrics.width;
            height = max(height, metrics.height);
        }

        (width, height)
    }
}

impl Drawable for Text {}

impl Text {
    pub fn new() -> Self {
        Self {
            color: 0,
            text: "".to_string(),
            size: 18,
            font: None,
        }
    }

    pub fn with_size(self, size: usize) -> Self {
        let mut text = self;
        text.size = size;
        text
    }

    pub fn with_text(self, text: &str) -> Self {
        let mut text_obj = self;
        text_obj.text = text.to_string();
        text_obj
    }

    pub fn with_font_file(self, font_path: &str) -> Option<Self> {
        let mut text_obj = self;

        let mut file = std::fs::File::open(font_path).unwrap();
        let length = file.metadata().unwrap().len();
        let mut data = vec![0; length as usize];

        file.read(data.as_mut_slice()).unwrap();

        let font = fontdue::Font::from_bytes(data.as_slice(), FontSettings::default());

        if let Err(e) = font {
            return None;
        }

        text_obj.font = Some(font.unwrap());

        Some(text_obj)
    }
}
