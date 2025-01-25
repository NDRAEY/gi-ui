pub trait Position {
    fn set_position(&mut self, x: usize, y: usize);
    fn get_position(&self) -> (usize, usize);
}

#[macro_export]
macro_rules! impl_position {
    ($type:ty) => {
        impl Position for $type {
            fn set_position(&mut self, x: usize, y: usize) {
                self.widget.set_position(x, y);
            }

            fn get_position(&self) -> (usize, usize) {
                self.widget.get_position()
            }
        }
    };
}
