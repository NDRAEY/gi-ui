use core::cell::RefCell;
use core::cmp::max;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;

use crate::canvas::Canvas;
use crate::components::touchable::Touchable;
use crate::draw::Draw;
use crate::parent::HasParent;
use crate::side::Side;
use crate::size::Size;
use crate::{alignment, Drawable};

use super::Direction;

type ContainerDrawable = Rc<RefCell<Box<(dyn Drawable + 'static)>>>;
type Drawables = Vec<ContainerDrawable>;

// TODO: Add alignment
#[derive(Default)]
pub struct LinearLayout<'a> {
    pub(crate) parent: Option<&'a dyn Drawable>,
    pub(crate) contained_widgets: Drawables,
    pub direction: Direction,
    pub horizontal_alignment: alignment::HorizontalAlignment,
    pub vertical_alignment: alignment::VerticalAlignment,
    pub margin: crate::side::Side,
}

impl Size for LinearLayout<'_> {
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
                Direction::Vertical => {
                    sy += h as isize;
                    sx = max(sx, w as isize);
                }
                Direction::Horizontal => {
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

impl Draw for LinearLayout<'_> {
    fn draw(&mut self, canvas: &mut Canvas, x: isize, y: isize) {
        let mut sx = x + self.margin.left;
        let mut sy = y + self.margin.top;

        for element in &mut self.contained_widgets {
            let mut element = element.borrow_mut();
            let (w, h) = element.size();

            element.draw(canvas, sx as _, sy as _);

            match self.direction {
                Direction::Vertical => {
                    sy += h as isize;
                }
                Direction::Horizontal => {
                    sx += w as isize;
                }
            }
        }
    }
}

impl Drawable for LinearLayout<'static> {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

impl LinearLayout<'_> {
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

    pub fn calculate_element_position(
        &self,
        element: &ContainerDrawable,
    ) -> Option<(usize, usize)> {
        let mut sx = self.margin.left as usize;
        let mut sy = self.margin.top as usize;

        for i in &self.contained_widgets {
            let (w, h) = i.borrow().size();

            if i.as_ptr() == element.as_ptr() {
                return Some((sx, sy));
            }

            match self.direction {
                Direction::Vertical => {
                    sy += h;
                }
                Direction::Horizontal => {
                    sx += w;
                }
            }
        }

        None
    }

    pub fn process_touches(&mut self, x: usize, y: usize) {
        for i in &self.contained_widgets {
            let position = self.calculate_element_position(i).unwrap();
            let size = i.borrow().size();

            if !crate::rect::is_point((x, y), (position.0, position.1, size.0, size.1)) {
                continue;
            }

            let mut elem = i.borrow_mut();
            let elem: Option<&mut Touchable> = elem.as_any_mut().downcast_mut::<Touchable>();

            if let Some(elem) = elem {
                elem.touch(x - position.0, y - position.1);
            }
        }
    }
}

impl HasParent<'_> for LinearLayout<'_> {
    fn parent(&self) -> Option<&dyn Drawable> {
        self.parent
    }
}