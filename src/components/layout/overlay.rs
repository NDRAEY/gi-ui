use std::cmp::max;

use crate::{canvas::Canvas, draw::Draw, size::Size, Drawable};

type Drawables = Vec<Box<dyn Drawable>>;

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
            let (w, h) = element.size();

            sy = max(sy, h);
            sx = max(sx, w);
        }

        (sx, sy)
    }
}

impl Draw for OverlayLayout {
    fn draw(&self, canvas: &mut Canvas, x: usize, y: usize) {
        for element in &self.contained_widgets {
            element.draw(canvas, x, y);
        }
    }
}

impl Drawable for OverlayLayout {}

impl OverlayLayout {
    pub fn new() -> Self {
        Self {
            contained_widgets: vec![],
        }
    }

    pub fn push(&mut self, element: impl Drawable + 'static) {
        self.contained_widgets.push(Box::new(element));
    }
}
