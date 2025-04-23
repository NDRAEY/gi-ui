#[derive(Debug, Clone, Copy)]
pub enum SizePolicy {
    Fixed(usize),
    FillParent
}

pub trait Size {
    fn set_size(&mut self, x: usize, y: usize);
    fn size(&self) -> (usize, usize);
}

// pub trait FullSize {
//     fn full_size(&self) -> (usize, usize);
// }

#[macro_export]
macro_rules! impl_size {
    ($type:ty) => {
        impl Size for $type {
            fn set_size(&mut self, width: usize, height: usize) {
                self.widget.set_size(width, height);
            }

            fn size(&self) -> (usize, usize) {
                self.widget.size()
            }
        }
    };
}
