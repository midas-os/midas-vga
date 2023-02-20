// Include required files
use midas_vga::graphics::*;
use midas_vga::shapes::*;
use vga::colors::Color16;

// Main function
fn main() {
    // Initialize graphics mode
    init();

    // Create circle at (320, 240) with a radius of 100
    let circle = Shape::circle((320, 240), 100);

    // Draw circle
    draw_shape(&circle, Color16::White);
}