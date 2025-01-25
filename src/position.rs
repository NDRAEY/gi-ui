pub trait Position {
    fn set_position(&mut self, x: usize, y: usize);
    fn position(&self) -> (usize, usize);
}

// #[macro_export]
// macro_rules! impl_position {
//     ($type:ty) => {
//         impl Position for $type {
//             fn set_position(&mut self, x: usize, y: usize) {
//                 self.widget.set_position(x, y);
//             }

//             fn position(&self) -> (usize, usize) {
//                 self.widget.position()
//             }
//         }
//     };
// }
