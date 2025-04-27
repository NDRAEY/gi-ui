use crate::{draw::Draw, size::Size, Drawable};
use font8x8::UnicodeFonts;
use alloc::string::String;
use alloc::string::ToString;
use gi_derive::widget;

#[widget]
#[derive(Default, Clone)]
pub struct Text {
    pub(crate) color: u32,
    pub(crate) text: String,
    pub(crate) size: usize,
    pub(crate) kerning_fix: isize
}

impl Text {
    pub fn new() -> Self {
        Self {
            color: 0xff_000000,
            size: 8,
            ..Self::default()
        }
    }

    pub fn with_size(self, size: usize) -> Self {
        let mut text = self;
        text.size = size;
        text
    }

    pub fn with_color(self, color: u32) -> Self {
        let mut text = self;
        text.color = color;
        text
    }

    pub fn with_text<S: ToString>(self, string: S) -> Self {
        let mut text = self;
        text.text = string.to_string();
        text
    }

    pub fn with_kerning(self, kerning: isize) -> Self {
        let mut text = self;
        text.kerning_fix = kerning;
        text
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    pub fn set_text<S: ToString>(&mut self, string: S) {
        self.text = string.to_string();
    }

    pub fn set_kerning(&mut self, kerning: isize) {
        self.kerning_fix = kerning;
    }
}

impl Draw for Text {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: isize, y: isize) {
        let mut rx: isize = 0;

        for ch in self.text.chars() {
            if let Some(c) = font8x8::BASIC_FONTS.get(ch) {
                for cy in 0..self.size {
                    // Scale cy back to the 0..8 range to select the correct font row
                    let font_y = (cy * 8) / self.size;
                    let line = c[font_y];

                    for cx in 0..self.size {
                        // Scale cx back to the 0..8 range to select the correct font column
                        let font_x = (cx * 8) / self.size;
                        let bit = line & (1 << font_x);

                        if bit != 0 {
                            canvas.set_pixel(x + rx + cx as isize, y + cy as isize, self.color);
                        }
                    }
                }
            }
            rx += (self.size as isize) + self.kerning_fix;
        }
    }
}

impl Size for Text {
    fn set_size(&mut self, _width: usize, _height: usize) {
        unreachable!()
    }

    fn size(&self) -> (usize, usize) {
        (self.text.len() * (self.size as isize + self.kerning_fix) as usize, self.size)
    }
}
