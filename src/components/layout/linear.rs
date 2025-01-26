use std::cmp::max;

use crate::canvas::Canvas;
// use crate::components::widget::Widget;
use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;
// use crate::position::Position;
// use crate::impl_position;

use super::Direction;

type Drawables = Vec<Box<dyn Drawable>>;

#[derive(Default)]
pub struct LinearLayout {
    //pub(crate) widget: Widget,
    pub(crate) contained_widgets: Drawables,
    pub(crate) direction: Direction,
}

// impl_position!(LinearLayout);

impl Size for LinearLayout {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unreachable!();
    }

    fn size(&self) -> (usize, usize) {
        let mut sx = 0;
        let mut sy = 0;

        for element in &self.contained_widgets {
            let (w, h) = element.size();

            match self.direction {
                Direction::VERTICAL => {
                    sy += h;
                    sx = max(sx, w);
                }
                Direction::HORIZONTAL => {
                    sx += w;
                    sy = max(sy, h);
                }
            }
        }

        (sx, sy)
    }
}

impl Draw for LinearLayout {
    fn draw(&self, canvas: &mut Canvas, x: usize, y: usize) {
        let mut sx = x;
        let mut sy = y;

        for element in &self.contained_widgets {
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

    pub fn elements(&self) -> &Drawables {
        &self.contained_widgets
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn push(&mut self, element: impl Drawable + 'static) {
        self.contained_widgets.push(Box::new(element));
    }
}
