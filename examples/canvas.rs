use zeraus::{canvas::Canvas, components::{layout::linear::LinearLayout, rectangle::Rectangle}, draw::Draw};

const WIDTH: usize = 1000;
const HEIGHT: usize = 500;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut canvas2 = Canvas::new(40, 30);

    for y in 0..10 {
        for x in 0..10 {
            canvas2.set_pixel(10 + x, 10 + y, 0xff_0f0fff);
        }
    }

    let rect = Rectangle::new()
        .with_size(WIDTH, 75)
        .foreground_color(0xff_00ffff);

    let mut lay = LinearLayout::new();
    lay.push(rect);
    lay.push(canvas2);

    lay.draw(&mut canvas, 0, 0);

    zeraus::helpers::export_to_png(&canvas);
}