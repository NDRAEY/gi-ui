use alloc::rc::Rc;
use core::cell::RefCell;
use core::cmp::max;
use gi_derive::{with_parent, Widget};

use alloc::{boxed::Box, vec::Vec};

use alloc::vec;

use crate::parent::HasParent;
use crate::{canvas::Canvas, draw::Draw, size::Size, Drawable};


type ContainerDrawable = Rc<RefCell<Box<(dyn Drawable + 'static)>>>;
type Drawables = Vec<ContainerDrawable>;

#[with_parent]
#[derive(Default, Widget)]
pub struct OverlayLayout<'a> {
    pub(crate) contained_widgets: Drawables,
}

impl Size for OverlayLayout<'_> {
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

impl Draw for OverlayLayout<'_> {
    fn draw(&mut self, canvas: &mut Canvas, x: isize, y: isize) {
        for element in &mut self.contained_widgets {
            element.borrow_mut().draw(canvas, x, y);
        }
    }
}

impl OverlayLayout<'_> {
    pub fn new() -> Self {
        Self {
            parent: None,
            contained_widgets: vec![],
        }
    }

    pub fn push(&mut self, element: impl Drawable + 'static) -> Rc<RefCell<Box<dyn Drawable>>> {
        let el: Rc<RefCell<Box<dyn Drawable>>> = Rc::new(RefCell::new(Box::new(element)));

        self.contained_widgets.push(el.clone());

        el
    }
}

impl HasParent<'_> for OverlayLayout<'_> {
    fn parent(&self) -> Option<&dyn Drawable> {
        self.parent
    }
}
