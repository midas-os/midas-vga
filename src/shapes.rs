use vga::drawing::Point;

pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
    Line(Line),
}

// Shapes
pub struct Rectangle {
    pub start: Point<isize>,
    pub end: Point<isize>,
}

pub struct Circle {
    pub center: Point<isize>,
    pub radius: usize,
}

pub struct Triangle {
    pub a: Point<isize>,
    pub b: Point<isize>,
    pub c: Point<isize>,
}

pub struct Line {
    pub start: Point<isize>,
    pub end: Point<isize>,
}

// Implement Shapes with the Shape enum
impl Rectangle {
    pub fn new(start: Point<isize>, end: Point<isize>) -> Rectangle {
        Rectangle { start, end }
    }
}

impl Circle {
    pub fn new(center: Point<isize>, radius: usize) -> Circle {
        Circle { center, radius }
    }
}

impl Triangle {
    pub fn new(a: Point<isize>, b: Point<isize>, c: Point<isize>) -> Triangle {
        Triangle { a, b, c }
    }
}

impl Line {
    pub fn new(start: Point<isize>, end: Point<isize>) -> Line {
        Line { start, end }
    }
}

// Implement Shape Trait for Shapes
impl Shape {
    pub fn rectangle(start: Point<isize>, end: Point<isize>) -> Shape {
        Shape::Rectangle(Rectangle::new(start, end))
    }

    pub fn circle(center: Point<isize>, radius: usize) -> Shape {
        Shape::Circle(Circle::new(center, radius))
    }

    pub fn triangle(a: Point<isize>, b: Point<isize>, c: Point<isize>) -> Shape {
        Shape::Triangle(Triangle::new(a, b, c))
    }

    pub fn line(start: Point<isize>, end: Point<isize>) -> Shape {
        Shape::Line(Line::new(start, end))
    }
}