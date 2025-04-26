use gi_ui::{
    canvas::Canvas,
    components::{
        circle::Circle, layout::linear::LinearLayout, rectangle::Rectangle, scrollview::ScrollView,
    },
    draw::Draw,
    size::{Size, SizePolicy},
};

const WIDTH: usize = 150;
const HEIGHT: usize = 150;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT); // 100, 100
    canvas.fill(0xff_444444);

    let rect1 = Rectangle::new()
        .with_size(50, 50)
        .foreground_color(0xff_aa00aa);

    let rect2 = Rectangle::new()
        .with_size(50, 50)
        .foreground_color(0xff_00aaaa);

    let circle1 = Circle::new()
        .with_radius(SizePolicy::Fixed(20))
        .with_foreground_color(0xff_a0a00a);

    let circle2 = Circle::new()
        .with_radius(SizePolicy::Fixed(25))
        .with_foreground_color(0xff_ab23fc);

    let mut layout1 = LinearLayout::new();
    layout1.set_direction(gi_ui::components::layout::Direction::Horizontal);
    layout1.push(rect1);
    layout1.push(rect2);

    let mut layout2 = LinearLayout::new();
    layout2.set_direction(gi_ui::components::layout::Direction::Horizontal);
    layout2.push(circle1);
    layout2.push(circle2);

    let mut final_layout = LinearLayout::new();
    final_layout.push(layout1);
    final_layout.push(layout2);

    let mut limited_layout = ScrollView::with(final_layout);

    let size = limited_layout.size();

    limited_layout.set_size(size.0, size.1 - 25);
    limited_layout.set_position(0, -12);

    limited_layout.draw(&mut canvas, 0, 0);

    // canvas.resize(size.0, size.1);

    gi_ui::helpers::export_to_png(&canvas);
}
