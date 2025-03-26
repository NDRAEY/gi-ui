use core::any::Any;

use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

use alloc::boxed::Box;

pub type TouchListener = dyn FnMut(&mut dyn Drawable, usize, usize);

pub struct Touchable {
    pub(crate) element: Box<dyn Drawable>,
    pub(crate) touch_listener: Box<TouchListener>,
}

impl Draw for Touchable {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: usize, y: usize) {
        self.element.draw(canvas, x, y);
    }
}

impl Size for Touchable {
    fn set_size(&mut self, x: usize, y: usize) {
        self.element.set_size(x, y);
    }

    fn size(&self) -> (usize, usize) {
        self.element.size()
    }
}

impl Drawable for Touchable {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Touchable {
    pub fn with(element: impl Drawable + 'static) -> Self {
        Self {
            element: Box::new(element),
            touch_listener: Box::new(|_, _, _| {}),
        }
    }

    pub fn with_listener(
        element: impl Drawable + 'static,
        listener: fn(&mut dyn Drawable, usize, usize),
    ) -> Self {
        Self {
            element: Box::new(element),
            touch_listener: Box::new(listener),
        }
    }

    pub fn register_callback(&mut self, f: Box<TouchListener>) {
        self.touch_listener = f;
    }

    pub fn touch(&mut self, x: usize, y: usize) {
        (self.touch_listener)(self.element.as_mut(), x, y);
    }
}
