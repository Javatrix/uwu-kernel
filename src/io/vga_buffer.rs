use core::isize;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

fn vga_buffer() -> *mut u8 {
    0xb8000 as *mut u8
}

pub fn print(chars: &[u8]) {
    for (i, &byte) in chars.iter().enumerate() {
        print_screen_char(
            ScreenChar {
                ascii_char: byte,
                color_code: ColorCode::new(Color::White, Color::Cyan),
            },
            i,
        );
    }
}

fn print_screen_char(char: ScreenChar, pos: usize) {
    unsafe {
        *vga_buffer().offset(pos as isize * 2) = char.ascii_char;
        *vga_buffer().offset(pos as isize * 2 + 1) = char.color_code.0;
    }
}
