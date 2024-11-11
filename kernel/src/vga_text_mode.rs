use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct FullColourCode(u8);

#[allow(dead_code)]
impl FullColourCode {
    fn new(foreground: Colour, background: Colour) -> FullColourCode {
        FullColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
struct Character {
    char: u8,
    colour: FullColourCode,
}

struct VgaTextBuffer {
    chars: [[Character; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    column_position: usize,
    colour_code: FullColourCode,
    buffer: &'static mut VgaTextBuffer,
}

impl VgaWriter {
    pub fn new_line(&mut self) {
        // move the items up one row
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe {
                    // TODO: VOLATILE READ AND WRITE TO FILL
                    // or we could just write a volatile cell
                    let character = read_volatile(self.buffer.chars[row][col]);
                    self.buffer.chars[row - 1][col].write(character);
                };
            }
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
        let colour = self.colour_code;

        unsafe {
            write_volatile(
                &mut self.buffer.chars[row][col] as *mut Character,
                Character { char, colour },
            )
        }
        self.column_position += 1;
    }
}

impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            if 0x20 <= byte && byte <= 0x7e {
                self.write_byte(byte);
            } else {
                self.write_byte(0xfe);
            }
        }

        Ok(())
    }
}

pub fn write() {
    let mut writer = VgaWriter {
        column_position: 0,
        colour_code: FullColourCode::new(Colour::White, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VgaTextBuffer) },
    };
    let _ = writer.write_str("Hello world!");
}
