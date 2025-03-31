use crate::canvas::Canvas;
use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

pub struct ScrollView<T: Drawable> {
    pub(crate) element: T,
    pub(crate) inner_canvas: Canvas,
    pub(crate) x: isize,
    pub(crate) y: isize,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl<T: Drawable> Draw for ScrollView<T> {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: isize, y: isize) {
        self.element.draw(&mut self.inner_canvas, self.x, self.y);
        self.inner_canvas.draw(canvas, x, y);
    }
}

impl<T: Drawable> Size for ScrollView<T> {
    fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;

        self.inner_canvas.resize(width, height);
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl<T: 'static + Drawable> Drawable for ScrollView<T> {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

impl<T: 'static + Drawable> ScrollView<T> {
    pub fn with(element: T) -> Self {
        let (w, h) = element.size();

        Self {
            element,
            width: w,
            height: h,
            x: 0,
            y: 0,
            inner_canvas: Canvas::new(w, h),
        }
    }

    pub fn set_position(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }

    pub fn element(&self) -> &T {
        &self.element
    }

    pub fn element_mut(&mut self) -> &mut T {
        &mut self.element
    }
}
