use alloc::rc::Rc;
use core::cell::RefCell;
use core::cmp::max;

use alloc::{boxed::Box, vec::Vec};

use alloc::vec;

use crate::{canvas::Canvas, draw::Draw, size::Size, Drawable};

type ContainerDrawable = Rc<RefCell<Box<(dyn Drawable + 'static)>>>;
type Drawables = Vec<ContainerDrawable>;

#[derive(Default)]
pub struct OverlayLayout {
    //pub(crate) widget: Widget,
    pub(crate) contained_widgets: Drawables,
}

impl Size for OverlayLayout {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unreachable!();
    }

    /// Returns the size of Layout (margins included)
    fn size(&self) -> (usize, usize) {
        let mut sx = 0usize;
        let mut sy = 0usize;

        for element in &self.contained_widgets {
            let (w, h) = element.borrow().size();

            sy = max(sy, h);
            sx = max(sx, w);
        }

        (sx, sy)
    }
}

impl Draw for OverlayLayout {
    fn draw(&mut self, canvas: &mut Canvas, x: usize, y: usize) {
        for element in &mut self.contained_widgets {
            element.borrow_mut().draw(canvas, x, y);
        }
    }
}

impl Drawable for OverlayLayout {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

impl OverlayLayout {
    pub fn new() -> Self {
        Self {
            contained_widgets: vec![],
        }
    }

    pub fn push(&mut self, element: impl Drawable + 'static) -> Rc<RefCell<Box<dyn Drawable>>> {
        let el: Rc<RefCell<Box<dyn Drawable>>> = Rc::new(RefCell::new(Box::new(element)));

        self.contained_widgets.push(el.clone());

        el
    }
}
