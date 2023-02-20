use vga::writers::{GraphicsWriter, Graphics320x240x256};
use vga::drawing::Point;
use vga::colors::Color16;

use crate::shapes::{Shape, Line, Triangle, Circle, Rectangle};

pub fn init() {
    let mode = Graphics320x240x256::new();
    mode.set_mode();
    mode.clear_screen(0x0);
}

pub fn to_usize(point: Point<isize>) -> Point<usize> {
    (point.0 as usize, point.1 as usize)
}

pub fn to_isize(point: Point<usize>) -> Point<isize> {
    (point.0 as isize, point.1 as isize)
}

pub fn draw_pixel(position: Point<isize>, color: u8) {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    mode.set_pixel(position.0 as usize, position.1 as usize, color);
}

pub fn color_palette_load() {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    mode.draw_line((0, 0), (200, 200), 0xFA);
}

// shapes
pub fn draw_shape(shape: &Shape, color: u8) {
    match shape {
        Shape::Rectangle(rectangle) => draw_rectangle(rectangle, color),
        Shape::Circle(circle) => draw_circle(circle, color),
        Shape::Triangle(triangle) => draw_triangle(triangle, color),
        Shape::Line(line) => draw_line(line, color),
    }
}

pub fn draw_shape_filled(shape: &Shape, color: u8) {
    match shape {
        Shape::Rectangle(rectangle) => draw_rectangle_filled(rectangle, color),
        Shape::Circle(circle) => draw_circle_filled(circle, color),
        Shape::Triangle(triangle) => draw_triangle_filled(triangle, color),
        Shape::Line(line) => draw_line(line, color),
    }
}

fn draw_triangle_filled(triangle: &Triangle, color: u8) {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    // Draw lines
    for x in triangle.a.0..triangle.b.0 {
        for y in triangle.a.1..triangle.c.1 {
            draw_pixel((x, y), color);
        }
    }
}

fn draw_circle_filled(circle: &Circle, color: u8) {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    // Bresenham's circle algorithm
    let mut x = circle.radius as isize;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        // Draw lines
        for i in circle.center.0 - x..circle.center.0 + x {
            draw_pixel((i, circle.center.1 + y), color);
            draw_pixel((i, circle.center.1 - y), color);
        }

        for i in circle.center.0 - y..circle.center.0 + y {
            draw_pixel((i, circle.center.1 + x), color);
            draw_pixel((i, circle.center.1 - x), color);
        }

        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
}

fn draw_rectangle_filled(rectangle: &Rectangle, color: u8) {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    // Draw lines
    for x in rectangle.start.0..rectangle.end.0 {
        for y in rectangle.start.1..rectangle.end.1 {
            draw_pixel((x, y), color);
        }
    }
}

fn draw_line(line: &Line, color: u8) {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    mode.draw_line(line.start, line.end, color);
}

fn draw_triangle(triangle: &Triangle, color: u8) {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    // Split into lines
    let a_line = Line::new(triangle.a, triangle.b);
    let b_line = Line::new(triangle.b, triangle.c);
    let c_line = Line::new(triangle.c, triangle.a);

    // Draw lines using mode.draw_line
    mode.draw_line(a_line.start, a_line.end, color);
    mode.draw_line(b_line.start, b_line.end, color);
    mode.draw_line(c_line.start, c_line.end, color);
}

fn draw_circle(circle: &Circle, color: u8) {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    // Bresenham's circle algorithm
    let mut x = circle.radius as isize;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        draw_pixel((circle.center.0 + x, circle.center.1 + y), color);
        draw_pixel((circle.center.0 + y, circle.center.1 + x), color);
        draw_pixel((circle.center.0 - y, circle.center.1 + x), color);
        draw_pixel((circle.center.0 - x, circle.center.1 + y), color);
        draw_pixel((circle.center.0 - x, circle.center.1 - y), color);
        draw_pixel((circle.center.0 - y, circle.center.1 - x), color);
        draw_pixel((circle.center.0 + y, circle.center.1 - x), color);
        draw_pixel((circle.center.0 + x, circle.center.1 - y), color);

        if err <= 0 {
            y += 1;
            err += 2 * y + 1;
        }

        if err > 0 {
            x -= 1;
            err -= 2 * x + 1;
        }
    }
}

fn draw_rectangle(rectangle: &Rectangle, color: u8) {
    let mode = Graphics320x240x256::new();
    mode.set_mode();

    // Split into lines
    let top_line = Line::new(rectangle.start, (rectangle.end.0, rectangle.start.1));
    let bottom_line = Line::new((rectangle.start.0, rectangle.end.1), rectangle.end);
    let left_line = Line::new(rectangle.start, (rectangle.start.0, rectangle.end.1));
    let right_line = Line::new((rectangle.end.0, rectangle.start.1), rectangle.end);

    // Draw lines using mode.draw_line
    mode.draw_line(top_line.start, top_line.end, color);
    mode.draw_line(bottom_line.start, bottom_line.end, color);
    mode.draw_line(left_line.start, left_line.end, color);
    mode.draw_line(right_line.start, right_line.end, color);
}

pub fn write_string(start: Point<usize>, string: &str, color: u8) {
    let mode = Graphics320x240x256::new();
    let mut y_offset = 0;
    let mut x_offset = 0;

    for (_offset, character) in string.chars().enumerate() {
        if character == '\n' {
            x_offset = 0;
            y_offset += 1;
            continue;
        }

        x_offset += 1;
        mode.draw_character(start.0 + x_offset * 8, start.1 + (y_offset * 10), character, color);
    }
}

pub fn write_str_centered(bounds_start: Point<usize>, bounds_end: Point<usize>, string: &str, color: u8) {
    let lines = string.split('\n');

    for (offset, line) in lines.enumerate() {
        let string_width = line.len() * 8;
        let string_height = 16;

        let x = (bounds_start.0 + bounds_end.0) / 2 - string_width / 2;
        let y = (bounds_start.1 + bounds_end.1) / 2 - string_height / 2;

        write_string((x, y + (offset * 10)), line, color);
    }
}

pub fn write_str_centered_x(bounds_start: Point<usize>, bounds_end: Point<usize>, y: usize, string: &str, color: u8) {
    // split string into new lines
    let lines = string.split('\n');

    for (offset, line) in lines.enumerate() {
        let string_width = line.len() * 8;

        let x = (bounds_start.0 + bounds_end.0) / 2 - string_width / 2;

        write_string((x, y + (offset * 10)), line, color);
    }
}

pub fn write_str_centered_y(bounds_start: Point<usize>, bounds_end: Point<usize>, x: usize, string: &str, color: u8) {
    let string_height = 16;

    let y = (bounds_start.1 + bounds_end.1) / 2 - string_height / 2;

    write_string((x, y), string, color);
}