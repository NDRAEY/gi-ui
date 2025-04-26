use gi_derive::widget;

use crate::{
    canvas::Canvas,
    draw::Draw,
    size::{Size, SizePolicy},
    Drawable,
};

#[widget]
#[derive(Clone)]
pub struct Circle {
    pub(crate) radius: SizePolicy,
    pub(crate) foreground_color: u32,
    pub(crate) border_color: u32,
    pub(crate) border_size: usize,
}

impl Draw for Circle {
    fn draw(&mut self, canvas: &mut Canvas, x: isize, y: isize) {
        let radius = self.calculate_radius(Some(canvas));

        self.draw_fill(canvas, x, y, radius + self.border_size, self.border_color);

        self.draw_fill(
            canvas,
            x + self.border_size as isize,
            y + self.border_size as isize,
            radius,
            self.foreground_color,
        );
    }
}

impl Size for Circle {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unreachable!();
    }

    fn size(&self) -> (usize, usize) {
        let radius = self.calculate_radius(None);

        (
            (radius * 2) + self.border_size,
            (radius * 2) + self.border_size,
        )
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}

impl Circle {
    pub fn new() -> Self {
        Self {
            parent: None,
            radius: SizePolicy::FillParent,
            foreground_color: 0x00_000000,
            border_color: 0x00_000000,
            border_size: 0,
        }
    }

    pub fn calculate_radius(&self, canvas: Option<&Canvas>) -> usize {
        match self.radius {
            SizePolicy::Fixed(sz) => sz,
            SizePolicy::FillParent => self
                .parent
                .as_ref()
                .map(|a| (a.as_ref().borrow().size().0 / 2) - self.border_size)
                .unwrap_or(
                    canvas
                        .map(|a| (core::cmp::min(a.width(), a.height()) / 2) - self.border_size)
                        .unwrap_or(0),
                ),
        }
    }

    pub fn with_radius(mut self, radius: SizePolicy) -> Self {
        self.radius = radius;
        self
    }

    pub fn set_foreground_color(mut self, color: u32) -> Self {
        self.foreground_color = color;
        self
    }

    pub fn set_border(mut self, color: u32, size: usize) -> Self {
        self.border_color = color;
        self.border_size = size;
        self
    }

    pub fn foreground_color(&self) -> u32 {
        self.foreground_color
    }

    pub fn border(&self) -> (u32, usize) {
        (self.border_color, self.border_size)
    }

    fn draw_fill(&self, canvas: &mut Canvas, x: isize, y: isize, radius: usize, color: u32) {
        let xc = x + radius as isize;
        let yc = y + radius as isize;
        let r = radius as isize;

        let mut d = 3 - (2 * r);
        let mut xi = 0;
        let mut yi = r;

        while xi <= yi {
            draw_scanline(canvas, xc - yi, xc + yi, yc + xi, color);
            draw_scanline(canvas, xc - yi, xc + yi, yc - xi, color);
            draw_scanline(canvas, xc - xi, xc + xi, yc + yi, color);
            draw_scanline(canvas, xc - xi, xc + xi, yc - yi, color);

            if d < 0 {
                d += (4 * xi) + 6;
            } else {
                d += (4 * (xi - yi)) + 10;
                yi -= 1;
            }

            xi += 1;
        }
    }
}

fn draw_scanline(canvas: &mut Canvas, x_start: isize, x_end: isize, y: isize, color: u32) {
    if y < 0 {
        return;
    }

    for x in x_start.max(0)..=x_end {
        canvas.set_pixel(x, y, color);
    }
}
