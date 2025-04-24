
use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

use alloc::boxed::Box;
use gi_derive::{with_parent, Widget};

pub type TouchListener = dyn FnMut(&mut dyn Drawable, usize, usize);

#[with_parent]
#[derive(Widget)]
pub struct Touchable {
    pub(crate) element: Box<dyn Drawable>,
    pub(crate) mousedown_listener: Box<TouchListener>,
    pub(crate) mouseup_listener: Box<TouchListener>,
    pub(crate) touch_listener: Box<TouchListener>,
}

impl Draw for Touchable<'_> {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: isize, y: isize) {
        self.element.draw(canvas, x, y);
    }
}

impl Size for Touchable<'_> {
    fn set_size(&mut self, x: usize, y: usize) {
        self.element.set_size(x, y);
    }

    fn size(&self) -> (usize, usize) {
        self.element.size()
    }
}

impl Touchable<'_> {
    pub fn new(element: impl Drawable + 'static) -> Self {
        Self {
            parent: None,
            element: Box::new(element),
            touch_listener: Box::new(|_, _, _| {}),
            mouseup_listener: Box::new(|_, _, _| {}),
            mousedown_listener: Box::new(|_, _, _| {}),
        }
    }

    pub fn with_touch_listener(self, listener: fn(&mut dyn Drawable, usize, usize)) -> Self {
        let mut touchable = self;
        touchable.touch_listener = Box::new(listener);
        touchable
    }

    pub fn with_mouseup_listener(self, listener: fn(&mut dyn Drawable, usize, usize)) -> Self {
        let mut touchable = self;
        touchable.mouseup_listener = Box::new(listener);
        touchable
    }

    pub fn with_mousedown_listener(self, listener: fn(&mut dyn Drawable, usize, usize)) -> Self {
        let mut touchable = self;
        touchable.mousedown_listener = Box::new(listener);
        touchable
    }

    pub fn touch(&mut self, x: usize, y: usize) {
        (self.touch_listener)(self.element.as_mut(), x, y);
    }

    pub fn mouse_up(&mut self, x: usize, y: usize) {
        (self.mouseup_listener)(self.element.as_mut(), x, y);
    }

    pub fn mouse_down(&mut self, x: usize, y: usize) {
        (self.mousedown_listener)(self.element.as_mut(), x, y);
    }
}
