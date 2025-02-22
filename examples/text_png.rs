use std::fs::File;

use zeraus::{
    canvas::Canvas,
    components::{
        layout::{linear::LinearLayout, overlay::OverlayLayout, Direction},
        rectangle::Rectangle,
        text::Text,
    },
    draw::Draw,
    size::Size,
};

const WIDTH: usize = 4000;
const HEIGHT: usize = 200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let text = Text::new()
        .with_size(64)
        .with_font_file("./examples/Ubuntu-Regular.ttf")
        .unwrap()
        // .with_text("Aa");
    .with_text("Perkele vitun saatana jumalauta");

    let tsize = text.size();

    let rect = Rectangle::new()
        .with_size(tsize.0, tsize.1)
        .foreground_color(0xff_ffffff);

    println!("{:?}", tsize);

    let mut layout = OverlayLayout::new();
    layout.push(rect);
    layout.push(text);

    let (total_width, total_height) = layout.size();

    layout.draw(&mut canvas, 0, 0);

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
