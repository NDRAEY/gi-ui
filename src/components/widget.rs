use crate::position::Position;
use crate::size::Size;

#[derive(Default, Clone)]
pub struct Widget {
    // pub(crate) x: usize,
    // pub(crate) y: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

// impl Position for Widget {
//     fn set_position(&mut self, x: usize, y: usize) {
//         self.x = x;
//         self.y = y;
//     }

//     fn position(&self) -> (usize, usize) {
//         (self.x, self.y)
//     }
// }

impl Size for Widget {
    fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
