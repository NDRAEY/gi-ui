use core::cell::{RefCell, RefMut};

use gi_derive::widget;
use alloc::rc::Rc;

#[cfg(feature = "std")]
use std::io::Read;

use alloc::string::String;
use alloc::string::ToString;

use fontdue::{
    self,
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    FontSettings,
};

use crate::{draw::Draw, size::Size, Drawable};

#[widget]
#[derive(Default, Clone)]
pub struct Text {
    pub(crate) color: u32,
    pub(crate) text: String,
    pub(crate) size: usize,
    pub(crate) font: Option<fontdue::Font>,
    // I would like to add `layout: Option<fontdue::layout::Layout>`, but it's not cloneable,
    // So I use Rc to make it shared.
    layout: Rc<RefCell<Option<fontdue::layout::Layout>>>,
}

impl Draw for Text {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: isize, y: isize) {
        let mut layout_ref = self.prepare_layout();
        let layout = layout_ref.as_mut().unwrap();

        let positions = layout.glyphs();

        // We iterate over characters.
        for (nr, i) in self.text.chars().enumerate() {
            // And rasterize each character.
            let (_, character) = self.font.as_ref().unwrap().rasterize(i, self.size as f32);

            let position = positions[nr];

            // Then it's an usual loop to draw a chacter on a `canvas`.
            for py in 0..position.height {
                for px in 0..position.width {
                    // Get an intensity from bitmap
                    let intensity = character[py * position.width + px] as f32 / 255.0;
                    // Separate color from it
                    let color = self.color & 0xff_ff_ff;
                    // And alpha channel (0..=255), then we multiply with alpha (0.00..=1.00).
                    let alpha = ((self.color >> 24) & 0xff) as f32 * intensity;
                    // Then we glue it back.
                    let result = ((alpha as u32) << 24) | color;

                    // And paint that pixel to canvas.
                    canvas.blit(
                        x + px as isize + position.x as isize,
                        y + py as isize + position.y as isize,
                        result,
                    );
                }
            }
        }
    }
}

impl Size for Text {
    fn set_size(&mut self, _: usize, _: usize) {
        unreachable!();
    }

    fn size(&self) -> (usize, usize) {
        let mut layout_ref = self.prepare_layout();
        let layout = layout_ref.as_mut().unwrap();

        let last_character = layout.glyphs().iter().last().unwrap();
        let width = last_character.x as usize + last_character.width;

        (width, layout.height() as usize)
    }
}

impl Text {
    pub fn new() -> Self {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            ..Default::default()
        });

        Self {
            parent: None,
            color: 0xff_000000,
            text: "".to_string(),
            size: 18,
            font: None,
            layout: Rc::new(RefCell::new(Some(layout))),
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

    pub fn with_font_data(self, font_data: &[u8]) -> Option<Self> {
        let mut text_obj = self;

        let font = fontdue::Font::from_bytes(font_data, FontSettings::default());

        if font.is_err() {
            return None;
        }

        text_obj.font = Some(font.unwrap());

        Some(text_obj)
    }

    #[cfg(feature = "std")]
    pub fn with_font_file(self, font_path: &str) -> Option<Self> {
        let mut file = std::fs::File::open(font_path).unwrap();
        let length = file.metadata().unwrap().len();
        let mut data = vec![0; length as usize];

        file.read_exact(data.as_mut_slice()).unwrap();

        self.with_font_data(data.as_slice())
    }

    fn prepare_layout(&self) -> RefMut<Option<Layout>> {
        let fonts = &[self.font.as_ref().unwrap()];

        let mut almost_layout = self.layout.borrow_mut();
        let layout = almost_layout.as_mut().unwrap();

        layout.clear();
        layout.append(fonts, &TextStyle::new(&self.text, self.size as f32, 0));

        almost_layout
    }
}
