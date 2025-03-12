use core::cell::RefCell;
use core::cmp::max;

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::rc::Rc;

use crate::canvas::Canvas;
use crate::draw::Draw;
use crate::side::Side;
use crate::size::Size;
use crate::{alignment, Drawable};

use super::Direction;

type Drawables = Vec<Rc<RefCell<Box<(dyn Drawable + 'static)>>>>;

// TODO: Add alignment
#[derive(Default)]
pub struct LinearLayout {
    pub(crate) contained_widgets: Drawables,
    pub direction: Direction,
    pub horizontal_alignment: alignment::HorizontalAlignment,
    pub vertical_alignment: alignment::VerticalAlignment,
    pub margin: crate::side::Side,
}

impl Size for LinearLayout {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unreachable!();
    }

    /// Returns the size of Layout (margins included)
    fn size(&self) -> (usize, usize) {
        let mut sx = 0isize;
        let mut sy = 0isize;

        for element in &self.contained_widgets {
            let (w, h) = element.borrow().size();

            match self.direction {
                Direction::VERTICAL => {
                    sy += h as isize;
                    sx = max(sx, w as isize);
                }
                Direction::HORIZONTAL => {
                    sx += w as isize;
                    sy = max(sy, h as isize);
                }
            }
        }

        sx += self.margin.left + self.margin.right;
        sy += self.margin.top + self.margin.bottom;

        (sx as usize, sy as usize)
    }
}

impl Draw for LinearLayout {
    fn draw(&self, canvas: &mut Canvas, x: usize, y: usize) {
        let mut sx = (x as isize + self.margin.left) as usize;
        let mut sy = (y as isize + self.margin.top) as usize;

        for element in &self.contained_widgets {
            let element = element.borrow();
            let (w, h) = element.size();

            element.draw(canvas, sx, sy);

            match self.direction {
                Direction::VERTICAL => {
                    sy += h;
                }
                Direction::HORIZONTAL => {
                    sx += w;
                }
            }
        }
    }
}

impl Drawable for LinearLayout {}

impl LinearLayout {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn push(&mut self, element: (impl Drawable + 'static)) -> Rc<RefCell<Box<dyn Drawable>>> {
        let el: Rc<RefCell<Box<dyn Drawable>>> = Rc::new(RefCell::new(Box::new(element)));

        self.contained_widgets.push(el.clone());

        el
    }

    pub fn set_margin(&mut self, left: isize, top: isize, right: isize, bottom: isize) {
        self.margin = Side {
            left,
            top,
            right,
            bottom,
        };
    }
}
