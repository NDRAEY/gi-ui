use super::*;

#[test]
fn just_create() {
    let mut canvas = canvas::Canvas::new(1, 1);

    let res = canvas.set_pixel(0, 0, 0xff_123456);
    assert!(res.is_some());

    let color = canvas.get_pixel(0, 0);
    assert_eq!(color, Some(0xff_123456));
}
