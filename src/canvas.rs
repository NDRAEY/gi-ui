//! Canvas is a framebuffer for rendering widgets on it.
//! Pixel format is RGBA.
pub const BITS_PER_PIXEL: usize = 32;

use alloc::vec;
use alloc::vec::Vec;

use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

pub struct Canvas {
    framebuffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer_size = width * height * (BITS_PER_PIXEL >> 3);

        Canvas {
            framebuffer: vec![0; buffer_size],
            width,
            height,
        }
    }

    pub fn buffer(&self) -> &[u8] {
        self.framebuffer.as_ref()
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.framebuffer.as_mut()
    }

    pub fn buffer_size(&self) -> usize {
        self.width * self.height * (BITS_PER_PIXEL >> 3)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn pixel_position(&self, x: usize, y: usize) -> usize {
        (x + (y * self.width)) * (BITS_PER_PIXEL >> 3)
    }

    #[inline]
    pub fn into_rgba(color: u32) -> (u8, u8, u8, u8) {
        let b = (color & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let r = ((color >> 16) & 0xff) as u8;
        let a = ((color >> 24) & 0xff) as u8;

        (r, g, b, a)
    }

    /// Sets an individual pixel on the canvas
    /// Color format: `0xAARRGGBB`
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> Option<()> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let position = self.pixel_position(x, y);
        let (r, g, b, a) = Self::into_rgba(color);

        self.framebuffer[position] = r;
        self.framebuffer[position + 1] = g;
        self.framebuffer[position + 2] = b;
        self.framebuffer[position + 3] = a;

        Some(())
    }

    pub fn blit(&mut self, x: usize, y: usize, color: u32) -> Option<()> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let position = self.pixel_position(x, y);
        let (r, g, b, a) = Self::into_rgba(color);

        let r: u32 = r as u32;
        let g: u32 = g as u32;
        let b: u32 = b as u32;
        let a: u32 = a as u32;

        let background = self.get_pixel(x, y).unwrap_or(0);
        let (br, bg, bb, _) = Self::into_rgba(background);

        let br: u32 = br as u32;
        let bg: u32 = bg as u32;
        let bb: u32 = bb as u32;

        let inv_alpha = 255 - a;

        self.framebuffer[position] = ((a * b + inv_alpha * br) >> 8) as u8;
        self.framebuffer[position + 1] = ((a * g + inv_alpha * bg) >> 8) as u8;
        self.framebuffer[position + 2] = ((a * r + inv_alpha * bb) >> 8) as u8;
        self.framebuffer[position + 3] = (a) as u8;

        Some(())
    }

    /// Gets an individual pixel fromt the canvas
    /// Color format: `0xAARRGGBB`
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let position = self.pixel_position(x, y);

        let color = ((self.framebuffer[position] as u32) << 16)
            | ((self.framebuffer[position + 1] as u32) << 8)
            | ((self.framebuffer[position + 2] as u32))
            | ((self.framebuffer[position + 3] as u32) << 24);

        Some(color)
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let mut canvas = Canvas::new(width, height);

        for y in 0..height {
            for x in 0..width {
                canvas.set_pixel(x, y, self.get_pixel(x, y).unwrap_or(0));
            }
        }

        *self = canvas;
    }

    pub fn fill(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }
}

impl Size for Canvas {
    fn set_size(&mut self, x: usize, y: usize) {
        self.resize(x, y);
    }

    fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}

impl Draw for Canvas {
    fn draw(&mut self, canvas: &mut Canvas, sx: usize, sy: usize) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                canvas.blit(sx + x, sy + y, self.get_pixel(x, y).unwrap_or(0));
            }
        }
    }
}

impl Drawable for Canvas {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}
