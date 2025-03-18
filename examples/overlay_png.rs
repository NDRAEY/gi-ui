use zeraus::{
    canvas::Canvas,
    components::{
        layout::{linear::LinearLayout, overlay::OverlayLayout, Direction},
        rectangle::Rectangle,
    },
    draw::Draw,
    size::Size,
};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

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
    layout2.set_margin(5, 5, 0, 0);
    layout2.push(rect2);

    let mut lay = OverlayLayout::new();
    lay.push(layout);
    lay.push(layout2);

    let (total_width, total_height) = lay.size();

    lay.draw(&mut canvas, 0, 0);

    canvas.resize(total_width, total_height);

    zeraus::helpers::export_to_png(&canvas);
}
