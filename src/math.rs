use vga::drawing::Point;

pub fn calculate_centered_rect(size: Point<isize>) -> Point<isize> {
    let x = (640 - size.0) / 2;
    let y = (480 - size.1) / 2;

    (x, y)
}

pub fn calculate_rect_end_point(start: Point<isize>, size: Point<isize>) -> Point<isize> {
    let x = start.0 + size.0;
    let y = start.1 + size.1;

    (x, y)
}

pub fn calculate_rect_size(start: Point<isize>, end: Point<isize>) -> Point<isize> {
    let x = end.0 - start.0;
    let y = end.1 - start.1;

    (x, y)
}

pub fn calculate_rect_center(start: Point<isize>, end: Point<isize>) -> Point<isize> {
    let x = (start.0 + end.0) / 2;
    let y = (start.1 + end.1) / 2;

    (x, y)
}