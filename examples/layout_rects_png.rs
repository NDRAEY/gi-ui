use gi_ui::canvas::Canvas;
use gi_ui::components::layout::linear::LinearLayout;
use gi_ui::components::layout::Direction;
use gi_ui::components::rectangle::Rectangle;
use gi_ui::draw::Draw;
use gi_ui::size::Size;

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

    let rect3 = Rectangle::new()
        .with_size(20, 20)
        .foreground_color(0xff_0000ff);

    let rect4 = Rectangle::new()
        .with_size(20, 20)
        .foreground_color(0xff_ffffff);

    let mut layout = LinearLayout::new();

    layout.set_direction(Direction::Horizontal);
    layout.push(rect1);
    layout.push(rect2);

    let mut layout2 = LinearLayout::new();

    layout2.set_direction(Direction::Horizontal);
    layout2.push(rect3);
    layout2.push(rect4);

    let mut layout_common = LinearLayout::new();
    
    let layout = layout_common.push(layout);
    let layout2 = layout_common.push(layout2);

    println!("{} {}", layout.borrow().size().0, layout.borrow().size().1);
    println!(
        "{} {}",
        layout2.borrow().size().0,
        layout2.borrow().size().1
    );

    let (total_width, total_height) = layout_common.size();

    layout_common.draw(&mut canvas, 0, 0);

    canvas.resize(total_width, total_height);

    gi_ui::helpers::export_to_png(&canvas);
}
