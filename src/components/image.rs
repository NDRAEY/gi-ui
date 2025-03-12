use tinytga::{ParseError, RawPixel, RawTga};

use crate::{canvas::Canvas, draw::Draw, size::Size, Drawable};
use tinytga::Bpp::Bits24;

use alloc::vec::Vec;

#[derive(Clone)]
pub struct Image {
    //pub(crate) widget: widget::Widget,
    pub(crate) image_width: usize,
    pub(crate) image_height: usize,
    pub(crate) image_bpp: usize,
    pub(crate) image_data: Vec<u32>,
}

impl Size for Image {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unreachable!();
    }

    fn size(&self) -> (usize, usize) {
        (self.image_width, self.image_height)
    }
}

impl Draw for Image {
    fn draw(&self, canvas: &mut Canvas, sx: usize, sy: usize) {
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                canvas.set_pixel(sx + x, sy + y, self.image_data[y * self.image_width + x]);
            }
        }
    }
}

impl Drawable for Image {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl Image {
    pub fn from_image_data(data: &[u8]) -> Option<Image> {
        let im: Result<RawTga, ParseError> = RawTga::from_slice(data);

        match im {
            Ok(image) => {
                let pxls: Vec<_> = image.pixels().map(|a| {
                    let mod_ = if image.image_data_bpp() == Bits24 { 0xff_000000 } else { 0 };

                    mod_ | a.color
                }).collect();

                return Some(Image {
                    image_width: image.size().width as usize,
                    image_height: image.size().height as usize,
                    image_bpp: image.image_data_bpp() as usize,
                    image_data: pxls,
                });
            }
            Err(_) => None,
        }
    }
}
