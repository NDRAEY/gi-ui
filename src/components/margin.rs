use gi_derive::widget;

use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

#[derive(Default, Debug)]
pub struct MarginValue {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

#[widget]
#[derive(Debug)]
pub struct Margin<T: Drawable> {
    pub(crate) element: T,
    pub(crate) margin: MarginValue,
}

impl<T: Drawable> Draw for Margin<T> {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: isize, y: isize) {
        self.element.draw(
            canvas,
            x + self.margin.left as isize,
            y + self.margin.top as isize,
        );
    }
}

impl<T: Drawable> Size for Margin<T> {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unimplemented!()
    }

    fn size(&self) -> (usize, usize) {
        let (w, h) = self.element.size();

        (
            w + self.margin.left + self.margin.right,
            h + self.margin.top + self.margin.bottom,
        )
    }
}

impl<T: 'static + Drawable> Margin<T> {
    pub fn new(element: T) -> Self {
        Self {
            parent: None,
            element,
            margin: MarginValue::default(),
        }
    }

    pub fn with(element: T, margin: MarginValue) -> Self {
        Self {
            parent: None,
            element,
            margin,
        }
    }

    pub fn left(self, left: usize) -> Self {
        let mut margin = self;
        margin.margin.left = left;
        margin
    }

    pub fn right(self, right: usize) -> Self {
        let mut margin = self;
        margin.margin.right = right;
        margin
    }

    pub fn bottom(self, bottom: usize) -> Self {
        let mut margin = self;
        margin.margin.bottom = bottom;
        margin
    }

    pub fn top(self, top: usize) -> Self {
        let mut margin = self;
        margin.margin.top = top;
        margin
    }

    pub fn all(self, value: usize) -> Self {
        let mut margin = self;
        margin.margin = MarginValue {
            left: value,
            top: value,
            right: value,
            bottom: value,
        };
        margin
    }

    pub fn element_position(&self) -> (usize, usize) {
        (self.margin.left, self.margin.top)
    }

    pub fn element(&self) -> &T {
        &self.element
    }

    pub fn element_mut(&mut self) -> &mut T {
        &mut self.element
    }
}
