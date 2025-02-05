use crate::{
    components::{
        layout::{linear::LinearLayout, overlay::OverlayLayout, Direction},
        rectangle::Rectangle,
    },
    size::Size,
};

#[test]
fn simple_overlay_size() {
    let rect1 = Rectangle::new()
        .with_size(20, 20)
        .foreground_color(0xff_ff0000);

    let rect2 = Rectangle::new()
        .with_size(20, 20)
        .foreground_color(0xff_00ff00);

    let mut layout = LinearLayout::new();
    layout.set_direction(Direction::HORIZONTAL);
    layout.push(rect1);

    let mut layout2 = LinearLayout::new();
    layout2.set_direction(Direction::HORIZONTAL);
    layout2.push(rect2);

    let mut lay = OverlayLayout::new();
    lay.push(layout);
    lay.push(layout2);

    assert_eq!(lay.size(), (20, 20));
}

#[test]
fn overlay_but_second_layout_is_margined() {
    let rect1 = Rectangle::new()
        .with_size(20, 20)
        .foreground_color(0xff_ff0000);

    let rect2 = Rectangle::new()
        .with_size(20, 20)
        .foreground_color(0xff_00ff00);

    let mut layout = LinearLayout::new();
    layout.set_direction(Direction::HORIZONTAL);
    layout.push(rect1);

    let mut layout2 = LinearLayout::new();
    layout2.set_direction(Direction::HORIZONTAL);

    // Here we add margin a margin to the second layout
    layout2.set_margin(5, 5, 0, 0);
    layout2.push(rect2);

    let mut lay = OverlayLayout::new();
    lay.push(layout);
    lay.push(layout2);

    assert_eq!(lay.size(), (25, 25));
}