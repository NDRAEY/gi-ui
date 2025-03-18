use crate::canvas::Canvas;

pub trait Draw {
    fn draw(&mut self, canvas: &mut Canvas, x: usize, y: usize);
}
