use crate::{canvas::Canvas, Drawable};

pub trait Draw {
    fn draw(&mut self, canvas: &mut Canvas, x: isize, y: isize);
}
