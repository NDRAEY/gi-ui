use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

type TouchListener = fn(&mut dyn Drawable, usize, usize);

pub struct Touchable {
    pub(crate) element: Box<dyn Drawable>,
    pub(crate) touch_listener: TouchListener,
}

impl Draw for Touchable {
    fn draw(&self, canvas: &mut crate::canvas::Canvas, x: usize, y: usize) {
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
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

impl Touchable {
    pub fn with(element: Box<dyn Drawable>) -> Self {
        Self {
            element, touch_listener: |_, _, _| {}
        }
    }

    pub fn register_callback(&mut self, f: TouchListener) {
        self.touch_listener = f;
    }

    pub fn touch(&mut self, x: usize, y: usize) {
        (self.touch_listener)(self.element.as_mut(), x, y);
    }
}
