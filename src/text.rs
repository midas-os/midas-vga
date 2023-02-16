use vga::{writers::{TextWriter, Text80x25, ScreenCharacter}, colors::{TextModeColor}};

static SIZE: (usize, usize) = (80, 25);

pub fn init() {
    let mode = Text80x25::new();
    mode.set_mode();
    mode.clear_screen();
}

pub fn write_char(character: ScreenCharacter, x: usize, y: usize) {
    let mode = Text80x25::new();
    mode.set_mode();

    mode.write_character(x, y, character);
}

pub fn write_string(string: &str, x: usize, y: usize, color: TextModeColor) {
    let mode = Text80x25::new();
    mode.set_mode();

    for (i, character) in string.chars().enumerate() {
        write_char(ScreenCharacter::new(character as u8, color), x + i, y);
    }
}

pub fn write_string_centered(string: &str, y: usize, color: TextModeColor) {
    let mode = Text80x25::new();
    mode.set_mode();

    let lines = string.lines().count();
    let y = y - (lines / 2);

    for (_i, _line) in string.lines().enumerate() {
        let x = (SIZE.0 / 2) - (string.len() / 2);
        write_string(string, x, y, color);
    }
}