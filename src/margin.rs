#[derive(Debug)]
pub struct MarginValue {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

pub trait Margin {
    fn margin(&self) -> MarginValue;
    fn set_margin(&mut self, margin: MarginValue);
}