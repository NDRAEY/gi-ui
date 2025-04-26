use gi_derive::widget;

use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

#[widget]
#[derive(Default, Clone)]
pub struct Rectangle {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) border_color: u32,
    pub(crate) foreground_color: u32,
    pub(crate) border_size: usize,
}

impl Draw for Rectangle {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: isize, y: isize) {
        let ax = (x + self.width as isize) - 1;
        let ay = (y + self.height as isize) - 1;

        for b in 0..self.border_size as isize {
            // Draw top and bottom borders
            for i in (x + b)..=(ax - b) {
                canvas.set_pixel(i, y + b, self.border_color); // Top border
                canvas.set_pixel(i, ay - b, self.border_color); // Bottom border
            }

            // Draw left and right borders
            for j in (y + b)..=(ay - b) {
                canvas.set_pixel(x + b, j, self.border_color); // Left border
                canvas.set_pixel(ax - b, j, self.border_color); // Right border
            }
        }

        for cy in self.border_size..(self.height - self.border_size) {
            for cx in self.border_size..(self.width - self.border_size) {
                canvas.set_pixel(x + cx as isize, y + cy as isize, self.foreground_color);
            }
        }
    }
}

impl Size for Rectangle {
    fn set_size(&mut self, x: usize, y: usize) {
        self.width = x;
        self.height = y;
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_size(self, width: usize, height: usize) -> Self {
        let mut rect = self;

        rect.width = width;
        rect.height = height;

        rect
    }

    pub fn foreground_color(self, color: u32) -> Self {
        let mut rect = self;

        rect.foreground_color = color;

        rect
    }

    pub fn borders(self, color: u32, size: usize) -> Self {
        let mut rect = self;

        rect.border_color = color;
        rect.border_size = size;

        rect
    }
}
