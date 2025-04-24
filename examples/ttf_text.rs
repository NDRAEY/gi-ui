use gi_ui::{
    canvas::Canvas,
    components::{layout::overlay::OverlayLayout, rectangle::Rectangle, ttf_text::Text},
    draw::Draw,
    size::Size,
};

const WIDTH: usize = 4000;
const HEIGHT: usize = 200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let text = Text::new()
        .with_size(64)
        .with_font_file("./examples/Ubuntu-Regular.ttf")
        .unwrap()
        // .with_text("Aa");
        .with_text("It's Gi UI!!");

    let tsize = text.size();

    let rect = Rectangle::new()
        .with_size(tsize.0, tsize.1)
        .foreground_color(0xff_ffffff);

    println!("{:?}", tsize);

    let mut layout = OverlayLayout::new();
    layout.push(rect);
    layout.push(text);

    let (total_width, total_height) = layout.size();

    layout.draw(&mut canvas, 0, 0);

    canvas.resize(total_width, total_height);

    gi_ui::helpers::export_to_png(&canvas);
}
