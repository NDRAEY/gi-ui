use core::cell::RefCell;
use core::cmp::max;

use alloc::rc::Rc;
use alloc::vec::Vec;
use gi_derive::widget;

use crate::canvas::Canvas;
use crate::components::touchable::Touchable;
use crate::draw::Draw;
use crate::side::Side;
use crate::size::Size;
use crate::{alignment, Drawable};

use super::Direction;

type ContainerDrawable = Rc<RefCell<(dyn Drawable + 'static)>>;
type Drawables = Vec<ContainerDrawable>;

#[widget]
#[derive(Default)]
pub struct LinearLayoutCompanion {
    pub(crate) contained_widgets: Drawables,
    pub direction: Direction,
    pub horizontal_alignment: alignment::HorizontalAlignment,
    pub vertical_alignment: alignment::VerticalAlignment,
    pub margin: crate::side::Side,
}

impl Size for LinearLayoutCompanion {
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

impl Draw for LinearLayoutCompanion {
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

impl LinearLayoutCompanion {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn push(
        &mut self,
        element: Rc<RefCell<dyn Drawable>>,
        parent: &Rc<RefCell<LinearLayoutCompanion>>, // Pass parent as Rc
    ) -> Rc<RefCell<dyn Drawable>> {
        // Downgrade the parent Rc to a Weak pointer
        let weak_parent = Rc::downgrade(parent);

        element.borrow_mut().set_parent(weak_parent);

        self.contained_widgets.push(element.clone());

        element
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

            if core::ptr::addr_eq(i.as_ptr(), element.as_ptr()) {
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

pub struct LinearLayout {
    pub(crate) companion: Rc<RefCell<LinearLayoutCompanion>>,
}

impl Default for LinearLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LinearLayout {
    pub fn new() -> Self {
        Self {
            companion: Rc::new(RefCell::new(LinearLayoutCompanion::new())),
        }
    }

    #[inline]
    pub fn set_direction(&mut self, direction: Direction) {
        self.companion.borrow_mut().set_direction(direction);
    }

    #[inline]
    pub fn push(&mut self, element: (impl Drawable + 'static)) -> Rc<RefCell<dyn Drawable>> {
        self.push_bare(Rc::new(RefCell::new(element)))
    }

    #[inline]
    pub fn push_bare(&mut self, element: Rc<RefCell<dyn Drawable>>) -> Rc<RefCell<dyn Drawable>> {
        let binding = &self.companion;

        self.companion.borrow_mut().push(element, binding)
    }

    #[inline]
    pub fn set_margin(&mut self, left: isize, top: isize, right: isize, bottom: isize) {
        self.companion.borrow_mut().set_margin(left, top, right, bottom);
    }

    #[inline]
    pub fn process_touches(&mut self, x: usize, y: usize) {
        self.companion.borrow_mut().process_touches(x, y);
    }

    #[inline]
    pub fn companion(&self) -> &Rc<RefCell<LinearLayoutCompanion>> {
        &self.companion
    }
}

impl Size for LinearLayout {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unreachable!();
    }

    fn size(&self) -> (usize, usize) {
        self.companion.borrow().size()
    }
}

impl Draw for LinearLayout {
    fn draw(&mut self, canvas: &mut Canvas, x: isize, y: isize) {
        self.companion.borrow_mut().draw(canvas, x, y);
    }
}

impl Drawable for LinearLayout {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
    
    fn parent(&self) -> Option<alloc::rc::Weak<core::cell::RefCell<dyn Drawable>>> {
        self.companion.borrow().parent()
    }
    
    fn set_parent(&mut self, parent: alloc::rc::Weak<core::cell::RefCell<dyn Drawable>>) {
        self.companion.borrow_mut().set_parent(parent)
    }
}
