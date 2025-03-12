use std::fs::File;
use zeraus::canvas::Canvas;
use zeraus::components::circle::Circle;
use zeraus::draw::Draw;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT); // 100, 100
    canvas.fill(0xff_000000);

    let circle = Circle::new()
        .with_radius(25)
        .border(0xff_ffffff, 5)
        .foreground_color(0xff_0000aa);

    circle.draw(&mut canvas, 0, 0);

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
