pub fn is_point(point: (usize, usize), rect: (usize, usize, usize, usize)) -> bool {
    let (x, y) = point;
    let (rect_x, rect_y, rect_width, rect_height) = rect;

    x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height
}
