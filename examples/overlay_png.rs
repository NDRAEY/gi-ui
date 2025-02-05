use std::fs::File;

use zeraus::{canvas::Canvas, components::{layout::{linear::LinearLayout, overlay::OverlayLayout, Direction}, rectangle::Rectangle}, draw::Draw, size::Size};

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

    let buffer = canvas.buffer();

    {
        if std::fs::exists("./out.png").unwrap() {
            std::fs::remove_file("./out.png").unwrap();
        }

        let file = File::create("./out.png").unwrap();
        let writer = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, total_width as u32, total_height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(buffer).unwrap();
    }
}