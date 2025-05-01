use gi_ui::canvas::Canvas;
use gi_ui::components::circle::Circle;
use gi_ui::components::layout::linear::LinearLayout;
use gi_ui::components::layout::overlay::OverlayLayout;
use gi_ui::components::layout::Direction;
use gi_ui::components::margin::Margin;
use gi_ui::components::rectangle::Rectangle;
use gi_ui::components::text8x8::Text;
use gi_ui::components::touchable::Touchable;
use gi_ui::draw::Draw;
use gi_ui::helpers::i_am_sure_mut;
use gi_ui::size::{Size, SizePolicy};
use gi_ui::{linear_layout, Drawable};

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut bar = OverlayLayout::new();

    let rect = Rectangle::new()
        .with_size(800, HEIGHT)
        .foreground_color(0xff_0000ff);

    let close_button = Touchable::new(
        Margin::new(
            Circle::new()
                .with_radius(SizePolicy::Fixed(10))
                .with_foreground_color(0xff_ff0000),
        )
        .right(10),
    )
    .with_touch_listener(|e, _, _| {
        println!("Clicked on close");

        let elem: &mut Margin<Circle> = i_am_sure_mut(e);

        elem.element_mut().set_foreground_color(0xff_ffffff);
    });

    let minimize_button = Margin::new(
        Circle::new()
            .with_radius(SizePolicy::Fixed(10))
            .with_foreground_color(0xff_ffff00),
    )
    .right(10);

    let maximize_button = Margin::new(
        Circle::new()
            .with_radius(SizePolicy::Fixed(10))
            .with_foreground_color(0xff_00ff00),
    )
    .right(10);

    let titlebar = Margin::new(Text::new().with_text("Hello world!").with_color(0xff_ffffff).with_size(16)).right(10);

    let mut together = linear_layout![close_button, minimize_button, maximize_button];
    together.set_direction(Direction::Horizontal);

    let together = Touchable::new(together).with_touch_listener(|elem: &mut dyn Drawable, x, y| {
        let el: &mut LinearLayout = i_am_sure_mut(elem);
        el.process_touches(x, y);
    });

    let mut fin = linear_layout![together, titlebar];
    fin.set_direction(Direction::Horizontal);

    let fin = Margin::new(fin).all(15);
    let fin = Touchable::new(fin).with_touch_listener(|elem: &mut dyn Drawable, x, y| {
        let el: &mut Margin<LinearLayout> = i_am_sure_mut(elem);
        el.element_mut().process_touches(x, y);
    });

    bar.push(rect);
    let clickable_layout = bar.push(fin);

    let (total_width, total_height) = bar.size();

    {
        let mut clickable_layout = clickable_layout.borrow_mut();
        let clickable_layout: &mut Touchable = i_am_sure_mut(clickable_layout.as_mut());

        clickable_layout.touch(20, 20);
    }

    bar.draw(&mut canvas, 0, 0);

    println!("{} {}", total_width, total_height);

    canvas.resize(total_width, total_height);

    gi_ui::helpers::export_to_png(&canvas);
}
