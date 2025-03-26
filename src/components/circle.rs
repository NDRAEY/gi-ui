use crate::{canvas::Canvas, draw::Draw, size::Size, Drawable};

#[derive(Debug, Default, Clone, Copy)]
pub struct Circle {
    pub(crate) radius: usize,
    pub(crate) foreground_color: u32,
    pub(crate) border_color: u32,
    pub(crate) border_size: usize,
}

impl Draw for Circle {
    fn draw(&mut self, canvas: &mut Canvas, x: usize, y: usize) {
        self.draw_fill(
            canvas,
            x,
            y,
            self.radius + self.border_size,
            self.border_color
        );

        self.draw_fill(
            canvas,
            x + self.border_size,
            y + self.border_size,
            self.radius,
            self.foreground_color
        );
    }
}

impl Size for Circle {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unreachable!();
    }

    fn size(&self) -> (usize, usize) {
        (
            (self.radius * 2) + self.border_size,
            (self.radius * 2) + self.border_size,
        )
    }
}

impl Drawable for Circle {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

impl Circle {
    pub fn new() -> Self {
        Circle {
            radius: 0,
            foreground_color: 0x00_000000,
            border_color: 0x00_000000,
            border_size: 0,
        }
    }

    pub fn with_radius(mut self, radius: usize) -> Self {
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

    fn draw_fill(&self, canvas: &mut Canvas, x: usize, y: usize, radius: usize, color: u32) {
        let xc = x + radius;
        let yc = y + radius;
        let r = radius as isize;

        let mut d = 3 - (2 * r);
        let mut xi = 0;
        let mut yi = r;

        while xi <= yi {
            draw_scanline(
                canvas,
                xc as isize - yi,
                xc as isize + yi,
                yc as isize + xi,
                color,
            );
            draw_scanline(
                canvas,
                xc as isize - yi,
                xc as isize + yi,
                yc as isize - xi,
                color,
            );
            draw_scanline(
                canvas,
                xc as isize - xi,
                xc as isize + xi,
                yc as isize + yi,
                color,
            );
            draw_scanline(
                canvas,
                xc as isize - xi,
                xc as isize + xi,
                yc as isize - yi,
                color,
            );

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
        canvas.set_pixel(x as usize, y as usize, color);
    }
}
