use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

use alloc::boxed::Box;

type Element = Box<dyn Drawable>;

#[derive(Default, Debug)]
pub struct MarginValue {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

pub struct Margin {
    pub(crate) element: Element,
    pub(crate) margin: MarginValue,
}

impl Draw for Margin {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: usize, y: usize) {
        self.element
            .draw(canvas, x + self.margin.left, y + self.margin.top);
    }
}

impl Size for Margin {
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

impl Drawable for Margin {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

impl Margin {
    pub fn with(element: Element, margin: MarginValue) -> Self {
        Self { element, margin }
    }

    pub fn left_and_right(element: Element, left: usize, right: usize) -> Self {
        Self {
            element,
            margin: MarginValue {
                left,
                top: 0,
                right,
                bottom: 0,
            },
        }
    }

    pub fn top_and_bottom(element: Element, top: usize, bottom: usize) -> Self {
        Self {
            element,
            margin: MarginValue {
                left: 0,
                top,
                right: 0,
                bottom,
            },
        }
    }

    pub fn like_args(element: Element, left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self {
            element,
            margin: MarginValue {
                left,
                top,
                right,
                bottom,
            }
        }
    }

    pub fn element_position(&self) -> (usize, usize) {
        (self.margin.left, self.margin.top)
    }

    pub fn element(&self) -> &Element {
        &self.element
    }

    pub fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}
