use vga::{writers::{Graphics640x480x16, GraphicsWriter}, drawing::Point, colors::Color16};

pub fn init() {
    let mode = Graphics640x480x16::new();
    mode.set_mode();
}

pub fn to_usize(point: Point<isize>) -> Point<usize> {
    (point.0 as usize, point.1 as usize)
}

pub fn draw_rect(position: Point<isize>, size: Point<isize>, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    mode.draw_line((position.0, position.1), (position.0, position.1 + size.1), color);
    mode.draw_line((position.0, position.1), (position.0 + size.0, position.1), color);
    mode.draw_line((position.0 + size.0, position.1), (position.0 + size.0, position.1 + size.1), color);
    mode.draw_line((position.0, position.1 + size.1), (position.0 + size.0, position.1 + size.1), color);
}