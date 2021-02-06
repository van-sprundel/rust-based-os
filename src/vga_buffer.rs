use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) } // 0xb8000 is the memory-mapped io for writing to screen
});
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // guarantees C struct
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// char sizes
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    // Character buffer
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT] // Generic so you cant write through a normal write
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            // see https://en.wikipedia.org/wiki/ASCII#Printable_characters
            match byte {
                // printable byte (32-126)
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // non-printable byte
                0x08 => self.remove_byte(), // BackSpace
                _ => self.write_byte(0xfe) // ■
            }
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // new line on \n byte
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1; // advance position
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read(); // read all characters
                self.buffer.chars[row - 1][col].write(character); // move area up by one
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1); // clear bottom row
        self.column_position = 0; // sets position to bottom
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank)
        }
    }
    pub fn remove_byte(&mut self) {
        if self.column_position > 0 {
            let row = BUFFER_HEIGHT - 1;
            let col = self.column_position;

            self.buffer.chars[row][col-1].write(ScreenChar {
                ascii_character: 0,
                color_code: ColorCode::new(Color::Black,Color::Black),
            });
            self.column_position -= 1; // advance position
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Macros
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    // we use a closure because we need to capture the value when it is locked, so it can't be interrupted
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output")
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output")
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some string that fits on one line";
    interrupts::without_interrupts(|| {
        // we need to lock the writer during the entire test, normally you'd only lock it while writing, but a deadlock prevention
        // keeps writing dots between the characters.
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() { // enumerate counts the characters in variable i
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read(); // - 2 because it should go to a new line
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
