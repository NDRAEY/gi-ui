use std::fs::File;
use zeraus::alignment::{HorizontalAlignment, VerticalAlignment};
use zeraus::canvas::Canvas;
use zeraus::components::circle::Circle;
use zeraus::components::layout::linear::LinearLayout;
use zeraus::components::layout::overlay::OverlayLayout;
use zeraus::components::layout::Direction;
use zeraus::components::margin::{Margin, MarginValue};
use zeraus::components::rectangle::Rectangle;
use zeraus::components::touchable::Touchable;
use zeraus::draw::Draw;
use zeraus::size::Size;

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut bar = OverlayLayout::new();

    let rect = Rectangle::new()
        .with_size(800, HEIGHT)
        .foreground_color(0xff_0000ff);

    let close_button = Margin::like_args(
        Box::new(Circle::new().with_radius(25).set_foreground_color(0xff_ff0000)),
        10,
        0,
        10,
        0,
    );
    let minimize_button = Margin::like_args(
        Box::new(Circle::new().with_radius(25).set_foreground_color(0xff_ffff00)),
        10,
        0,
        10,
        0,
    );
    let mut maximize_button = Touchable::with(Box::new(Margin::like_args(
        Box::new(Circle::new().with_radius(25).set_foreground_color(0xff_00ff00)),
        10,
        0,
        10,
        0,
    )));

    maximize_button.register_callback(|elem, x, y| {
        // let el: &mut Margin = elem.as_any_mut().downcast_mut::<Margin>().unwrap();
        // let el: &mut Circle = el.element_mut().as_any_mut().downcast_mut::<Circle>().unwrap();

        // *el = el.set_foreground_color(0xff_fff00f);
        
        println!("Click at {}, {}", x, y);
    });

    

    let mut together = LinearLayout::new();
    together.set_direction(Direction::HORIZONTAL);

    together.push(close_button);
    together.push(minimize_button);
    let btn = together.push(maximize_button);

    bar.push(rect);
    bar.push(together);

    let (total_width, total_height) = bar.size();

    {
        let mut btn = btn.borrow_mut();
        let btn = btn.as_any_mut();
        let btn = btn.downcast_mut::<Touchable>();
        let btn = btn.unwrap();

        btn.touch(10, 15);
    }
    // let l = elp.borrow();
    // let t = l.as_ref().as_any().downcast_ref::<Margin>();
    // if let Some(x) = t {
    //     println!("{:?}", x.element_position());
    // }

    bar.draw(&mut canvas, 0, 0);

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
