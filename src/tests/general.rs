use crate::canvas;

#[test]
fn just_create() {
    let mut canvas = canvas::Canvas::new(1, 1);

    let res = canvas.set_pixel(0, 0, 0xff_123456);
    assert!(res.is_some());

    let color = canvas.get_pixel(0, 0);
    assert_eq!(color, Some(0xff_123456));
}

#[test]
fn resize_nopanic() {
    let mut canvas = canvas::Canvas::new(1, 1);

    canvas.set_pixel(0, 0, 0xff_123456);

    canvas.resize(100, 100);
    canvas.resize(40, 40);
    canvas.resize(200, 200);
    canvas.resize(1, 1);

    let pixel = canvas.get_pixel(0, 0);

    assert_eq!(pixel, Some(0xff_123456));
}
