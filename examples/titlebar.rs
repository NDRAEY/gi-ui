use gi_ui::canvas::Canvas;
use gi_ui::components::circle::Circle;
use gi_ui::components::layout::linear::LinearLayout;
use gi_ui::components::layout::overlay::OverlayLayout;
use gi_ui::components::layout::Direction;
use gi_ui::components::margin::Margin;
use gi_ui::components::rectangle::Rectangle;
use gi_ui::components::touchable::Touchable;
use gi_ui::draw::Draw;
use gi_ui::helpers::i_am_sure_mut;
use gi_ui::size::Size;
use gi_ui::Drawable;

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut bar = OverlayLayout::new();

    let rect = Rectangle::new()
        .with_size(800, HEIGHT)
        .foreground_color(0xff_0000ff);

    let close_button = Touchable::new(Margin::like_args(
        Circle::new()
            .with_radius(10)
            .set_foreground_color(0xff_ff0000),
        0,
        0,
        10,
        0,
    ))
    .with_touch_listener(|e, _, _| {
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
    together.set_direction(Direction::Horizontal);

    together.push(close_button);
    together.push(minimize_button);
    together.push(maximize_button);

    let together = Margin::like_args(together, 15, 15, 15, 15);
    let together =
        Touchable::new(together).with_touch_listener(
            |elem: &mut dyn Drawable, x, y| {
            let el = elem
                .as_any_mut()
                .downcast_mut::<Margin<LinearLayout>>()
                .unwrap();
            el.element_mut().process_touches(x, y);
        });

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

    gi_ui::helpers::export_to_png(&canvas);
}
