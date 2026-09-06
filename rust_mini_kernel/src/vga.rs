const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_ADDRESS: *mut u8 = 0xb8000 as *mut u8;

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

pub struct VgaWriter;

impl VgaWriter {
    pub unsafe fn clear_screen() {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let offset = (row * BUFFER_WIDTH + col) * 2;
                *VGA_ADDRESS.add(offset) = b' ';
                *VGA_ADDRESS.add(offset + 1) = 0x07;
            }
        }
    }

    pub unsafe fn write_string(row: usize, col: usize, s: &str, fg: Color, bg: Color) {
        let color_code = (bg as u8) << 4 | (fg as u8);
        for (i, b) in s.bytes().enumerate() {
            if col + i >= BUFFER_WIDTH {
                break;
            }
            let offset = (row * BUFFER_WIDTH + col + i) * 2;
            *VGA_ADDRESS.add(offset) = b;
            *VGA_ADDRESS.add(offset + 1) = color_code;
        }
    }
}
