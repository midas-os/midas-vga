use vga::writers::{Graphics640x480x16, GraphicsWriter};
use vga::drawing::Point;
use vga::colors::Color16;

use crate::shapes::{Shape, Line, Triangle, Circle, Rectangle};

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

// shapes
pub fn draw_shape(shape: &Shape, color: Color16) {
    match shape {
        Shape::Rectangle(rectangle) => draw_rectangle(rectangle, color),
        Shape::Circle(circle) => draw_circle(circle, color),
        Shape::Triangle(triangle) => draw_triangle(triangle, color),
        Shape::Line(line) => draw_line(line, color),
    }
}

pub fn draw_shape_filled(shape: &Shape, color: Color16) {
    match shape {
        Shape::Rectangle(rectangle) => draw_rectangle_filled(rectangle, color),
        Shape::Circle(circle) => draw_circle_filled(circle, color),
        Shape::Triangle(triangle) => draw_triangle_filled(triangle, color),
        Shape::Line(line) => draw_line(line, color),
    }
}

fn draw_triangle_filled(triangle: &Triangle, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    // Draw lines
    for x in triangle.a.0..triangle.b.0 {
        for y in triangle.a.1..triangle.c.1 {
            draw_pixel((x, y), color);
        }
    }
}

fn draw_circle_filled(circle: &Circle, color: Color16) {
    let mode = Graphics640x480x16::new();
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

fn draw_rectangle_filled(rectangle: &Rectangle, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    // Draw lines
    for x in rectangle.start.0..rectangle.end.0 {
        for y in rectangle.start.1..rectangle.end.1 {
            draw_pixel((x, y), color);
        }
    }
}

fn draw_line(line: &Line, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    // Draw line
    let mut x = line.start.0;
    let mut y = line.start.1;
    let mut dx = line.end.0 - line.start.0;
    let mut dy = line.end.1 - line.start.1;
    let mut sx = 1;
    let mut sy = 1;

    if dx < 0 {
        dx = -dx;
        sx = -1;
    }

    if dy < 0 {
        dy = -dy;
        sy = -1;
    }

    let mut err = dx - dy;

    loop {
        draw_pixel((x, y), color);

        if x == line.end.0 && y == line.end.1 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }

        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn draw_triangle(triangle: &Triangle, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    // Split into lines
    let a_line = Line::new(triangle.a, triangle.b);
    let b_line = Line::new(triangle.b, triangle.c);
    let c_line = Line::new(triangle.c, triangle.a);

    // Draw lines
    draw_line(&a_line, color);
    draw_line(&b_line, color);
    draw_line(&c_line, color);
}

fn draw_circle(circle: &Circle, color: Color16) {
    let mode = Graphics640x480x16::new();
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

fn draw_rectangle(rectangle: &Rectangle, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    // Split into lines
    let top_line = Line::new(rectangle.start, (rectangle.end.0, rectangle.start.1));
    let bottom_line = Line::new((rectangle.start.0, rectangle.end.1), rectangle.end);
    let left_line = Line::new(rectangle.start, (rectangle.start.0, rectangle.end.1));
    let right_line = Line::new((rectangle.end.0, rectangle.start.1), rectangle.end);

    // Draw lines
    draw_line(&top_line, color);
    draw_line(&bottom_line, color);
    draw_line(&left_line, color);
    draw_line(&right_line, color);
}

pub fn write_string(start: Point<usize>, string: &str, color: Color16) {
    let mode = Graphics640x480x16::new();
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