use crate::position::Position;

#[derive(Clone)]
pub struct Widget {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Default for Widget {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Position for Widget {
    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
