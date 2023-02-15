use vga::writers::{Graphics640x480x16, Graphics320x240x256, Graphics320x200x256, GraphicsWriter};
use vga::drawing::Point;
use vga::colors::Color16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GraphicsMode {
    Graphics640x480x16,
    Graphics320x200x256,
    Graphics320x240x256,
}

pub fn init(graphics_mode: GraphicsMode) {
    match graphics_mode {
        GraphicsMode::Graphics640x480x16 => {
            let mode = Graphics640x480x16::new();
            mode.set_mode();
        },
        GraphicsMode::Graphics320x240x256 => {
            let mode = Graphics320x240x256::new();
            mode.set_mode();
        },
        GraphicsMode::Graphics320x200x256 => {
            let mode = Graphics320x200x256::new();
            mode.set_mode();
        },
        _ => {
            panic!("Graphics mode not supported");
        }
    }
}

pub fn to_usize(point: Point<isize>) -> Point<usize> {
    (point.0 as usize, point.1 as usize)
}

pub fn to_isize(point: Point<usize>) -> Point<isize> {
    (point.0 as isize, point.1 as isize)
}

pub fn draw_rect(position: Point<isize>, size: Point<isize>, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    mode.draw_line((position.0, position.1), (position.0, position.1 + size.1), color);
    mode.draw_line((position.0, position.1), (position.0 + size.0, position.1), color);
    mode.draw_line((position.0 + size.0, position.1), (position.0 + size.0, position.1 + size.1), color);
    mode.draw_line((position.0, position.1 + size.1), (position.0 + size.0, position.1 + size.1), color);
}