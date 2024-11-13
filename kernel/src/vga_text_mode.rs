use core::fmt::Write;
use kvolatile::KVolatile;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Character {
    charcode: u8,
    color: u8,
}

impl Character {
    fn new(charcode: u8, bg_colour: Colour, fg_colour: Colour) -> Self {
        Self {
            charcode,
            color: (bg_colour as u8) << 4 | (fg_colour as u8),
        }
    }
}

struct Buffer {
    chars: [[KVolatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    column_position: usize,
    buffer: &'static mut Buffer,
    default_bg_colour: Colour,
    default_fg_colour: Colour,
}

impl VgaWriter {
    pub fn new_line(&mut self) {
        // move the items up one row
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            let character = Character::new(b' ', self.default_bg_colour, self.default_fg_colour);
            self.buffer.chars[row][col].write(character);
        }
    }

    pub fn write_byte(&mut self, char: u8) {
        if char == b'\n' {
            self.new_line();
            return; // write a new line only
        }

        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
            // continue to write to screen here
        }
        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        self.buffer.chars[row][col].write(Character::new(char, Colour::Black, Colour::White));
        self.column_position += 1;
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }
}

impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'\n' => self.write_byte(byte), // handle \n
                0x20..=0x7e => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }

        Ok(())
    }
}

pub fn write() {
    let mut writer = VgaWriter {
        column_position: 0,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        default_bg_colour: Colour::Black,
        default_fg_colour: Colour::White,
    };
    writer.clear_screen();
    let _ = writer.write_str("Hello world!\ntest");
}
