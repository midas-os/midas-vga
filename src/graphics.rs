use vga::writers::{Graphics640x480x16, GraphicsWriter};
use vga::drawing::Point;
use vga::colors::Color16;

pub fn init() {
    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);
}

pub fn to_usize(point: Point<isize>) -> Point<usize> {
    (point.0 as usize, point.1 as usize)
}

pub fn to_isize(point: Point<usize>) -> Point<isize> {
    (point.0 as isize, point.1 as isize)
}

pub fn draw_pixel(position: Point<isize>, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    mode.set_pixel(position.0 as usize, position.1 as usize, color);
}

pub fn draw_rect(position: Point<isize>, size: Point<isize>, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    mode.draw_line((position.0, position.1), (position.0, position.1 + size.1), color);
    mode.draw_line((position.0, position.1), (position.0 + size.0, position.1), color);
    mode.draw_line((position.0 + size.0, position.1), (position.0 + size.0, position.1 + size.1), color);
    mode.draw_line((position.0, position.1 + size.1), (position.0 + size.0, position.1 + size.1), color);
}

pub fn write_string(start: Point<usize>, string: &str, color: Color16) {
    let mode = Graphics640x480x16::new();
    let mut y_offset = 0;
    let mut x_offset = 0;

    for (offset, character) in string.chars().enumerate() {
        if character == '\n' {
            x_offset = 0;
            y_offset += 1;
            continue;
        }

        x_offset += 1;
        mode.draw_character(start.0 + x_offset * 8, start.1 + (y_offset * 10), character, color);
    }
}

pub fn draw_centered_rect(size: Point<isize>, color: Color16) {
    let x = (640 - size.0) / 2;
    let y = (480 - size.1) / 2;

    draw_rect((x, y), size, color);
}

pub fn calculate_centered_rect(size: Point<isize>) -> Point<isize> {
    let x = (640 - size.0) / 2;
    let y = (480 - size.1) / 2;

    (x, y)
}

pub fn write_str_centered(bounds_start: Point<usize>, bounds_end: Point<usize>, string: &str, color: Color16) {
    let lines = string.split('\n');

    for (offset, line) in lines.enumerate() {
        let string_width = line.len() * 8;
        let string_height = 16;

        let x = (bounds_start.0 + bounds_end.0) / 2 - string_width / 2;
        let y = (bounds_start.1 + bounds_end.1) / 2 - string_height / 2;

        write_string((x, y + (offset * 10)), line, color);
    }
}

pub fn write_str_centered_x(bounds_start: Point<usize>, bounds_end: Point<usize>, y: usize, string: &str, color: Color16) {
    // split string into new lines
    let lines = string.split('\n');

    for (offset, line) in lines.enumerate() {
        let string_width = line.len() * 8;

        let x = (bounds_start.0 + bounds_end.0) / 2 - string_width / 2;

        write_string((x, y + (offset * 10)), line, color);
    }
}

pub fn write_str_centered_y(bounds_start: Point<usize>, bounds_end: Point<usize>, x: usize, string: &str, color: Color16) {
    let string_height = 16;

    let y = (bounds_start.1 + bounds_end.1) / 2 - string_height / 2;

    write_string((x, y), string, color);
}