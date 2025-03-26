use zeraus::canvas::Canvas;
use zeraus::components::circle::Circle;
use zeraus::components::layout::linear::LinearLayout;
use zeraus::components::layout::overlay::OverlayLayout;
use zeraus::components::layout::Direction;
use zeraus::components::margin::Margin;
use zeraus::components::rectangle::Rectangle;
use zeraus::components::touchable::Touchable;
use zeraus::draw::Draw;
use zeraus::helpers::i_am_sure_mut;
use zeraus::size::Size;
use zeraus::Drawable;

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut bar = OverlayLayout::new();

    let rect = Rectangle::new()
        .with_size(800, HEIGHT)
        .foreground_color(0xff_0000ff);

    let close_button = Touchable::with_listener(Box::new(Margin::like_args(
        Circle::new()
            .with_radius(10)
            .set_foreground_color(0xff_ff0000),
        0,
        0,
        10,
        0,
    )), |e, _, _| {
        println!("Clicked on close");

        let elem: &mut Margin<Circle> = i_am_sure_mut(e);

        *elem.element_mut() = elem.element_mut().set_foreground_color(0xff_ffffff);
    });

    let minimize_button = Margin::like_args(
            Circle::new()
                .with_radius(10)
                .set_foreground_color(0xff_ffff00),
        0,
        0,
        10,
        0,
    );
    let maximize_button = Margin::like_args(
            Circle::new()
                .with_radius(10)
                .set_foreground_color(0xff_00ff00),
        0,
        0,
        10,
        0,
    );

    let mut together = LinearLayout::new();
    together.set_direction(Direction::HORIZONTAL);

    together.push(close_button);
    together.push(minimize_button);
    together.push(maximize_button);

    let together = Margin::like_args(together, 15, 15, 15, 15);
    let mut together = Touchable::with(Box::new(together));

    together.register_callback(Box::new(
        |elem: &mut dyn Drawable, x, y| {
            let el = elem.as_any_mut().downcast_mut::<Margin<LinearLayout>>().unwrap();
            el.element_mut().process_touches(x, y);
        },
    ));

    bar.push(rect);
    let clickable_layout = bar.push(together);

    let (total_width, total_height) = bar.size();

    {
        let mut clickable_layout = clickable_layout.borrow_mut();
        let clickable_layout: &mut Touchable = clickable_layout
            .as_any_mut()
            .downcast_mut::<Touchable>()
            .unwrap();

        clickable_layout.touch(20, 20);
    }

    bar.draw(&mut canvas, 0, 0);

    println!("{} {}", total_width, total_height);

    canvas.resize(total_width, total_height);

    zeraus::helpers::export_to_png(&canvas);
}
