use std::fs::File;
use zeraus::canvas::Canvas;
use zeraus::draw::Draw;
use zeraus::components::image::Image;
use zeraus::size::Size;

const WIDTH: usize = 1280;
const HEIGHT: usize = 1280;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let data = std::fs::read("./static/test_image.tga").unwrap();
    let image = Image::from_image_data(data.as_slice()).unwrap();

    image.draw(&mut canvas, 0, 0);

    let (w, h) = image.size();
    println!("{w} {h}");
    canvas.resize(w, h);

    let buffer = canvas.buffer();

    {
        if std::fs::exists("./out.png").unwrap() {
            std::fs::remove_file("./out.png").unwrap();
        }

        let file = File::create("./out.png").unwrap();
        let writer = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, w as u32, h as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(buffer).unwrap();
    }
}
