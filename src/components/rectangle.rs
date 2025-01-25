use crate::components::widget;
use crate::draw::Draw;
use crate::impl_position;
use crate::position::Position;

#[derive(Default, Clone)]
pub struct Rectangle {
    pub(crate) widget: widget::Widget,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) border_color: u32,
    pub(crate) foreground_color: u32,
    pub(crate) border_size: usize,
}

impl_position!(Rectangle);

impl Draw for Rectangle {
    fn draw(&self, canvas: &mut crate::canvas::Canvas) {
        let ax = (self.widget.x + self.width) - 1;
        let ay = (self.widget.y + self.height) - 1;

        for b in 0..self.border_size {
            // Draw top and bottom borders
            for i in (self.widget.x + b)..=(ax - b) {
                canvas.set_pixel(i, self.widget.y + b, self.border_color); // Top border
                canvas.set_pixel(i, ay - b, self.border_color); // Bottom border
            }

            // Draw left and right borders
            for j in (self.widget.y + b)..=(ay - b) {
                canvas.set_pixel(self.widget.x + b, j, self.border_color); // Left border
                canvas.set_pixel(ax - b, j, self.border_color); // Right border
            }
        }

        for cy in self.border_size..(self.height - self.border_size) {
            for cx in self.border_size..(self.width - self.border_size) {
                canvas.set_pixel(cx, cy, self.foreground_color);
            }
        }
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
