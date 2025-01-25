use std::fs::File;
use zeraus::canvas::Canvas;
use zeraus::components::rectangle::Rectangle;
use zeraus::draw::Draw;
use zeraus::position::Position;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut rect = Rectangle::new()
        .with_size(WIDTH, HEIGHT)
        .borders(0xff_ff0000, 2)
        .foreground_color(0xff_ff00ff);

    rect.set_position(0, 0);
    rect.draw(&mut canvas);

    let buffer = canvas.buffer();

    {
        if std::fs::exists("./out.png").unwrap() {
            std::fs::remove_file("./out.png").unwrap();
        }

        let file = File::create("./out.png").unwrap();
        let writer = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, WIDTH as u32, HEIGHT as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(buffer).unwrap();
    }
}
