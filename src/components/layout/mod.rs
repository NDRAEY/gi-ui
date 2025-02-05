pub mod linear;
pub mod overlay;

#[derive(Default)]
pub enum Direction {
    #[default]
    VERTICAL,
    HORIZONTAL,
}
