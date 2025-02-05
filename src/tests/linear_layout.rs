use crate::{
    components::{
        layout::{linear::LinearLayout, Direction},
        rectangle::Rectangle,
    },
    size::Size,
};

#[test]
fn four_rectangles_vh() {
    let rect1 = Rectangle::new().with_size(20, 20);

    let rect2 = Rectangle::new().with_size(20, 20);

    let rect3 = Rectangle::new().with_size(20, 20);

    let rect4 = Rectangle::new().with_size(20, 20);

    let mut layout = LinearLayout::new();

    layout.set_direction(Direction::HORIZONTAL);
    layout.push(rect1);
    layout.push(rect2);

    let mut layout2 = LinearLayout::new();
    layout2.set_direction(Direction::HORIZONTAL);
    layout2.push(rect3);
    layout2.push(rect4);

    assert_eq!(layout.size(), (40, 20));
    assert_eq!(layout2.size(), (40, 20));

    let mut layout_common = LinearLayout::new();
    layout_common.push(layout);
    layout_common.push(layout2);

    assert_eq!(layout_common.size(), (40, 40));
}
