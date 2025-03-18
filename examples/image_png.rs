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

    zeraus::helpers::export_to_png(&canvas);
}
