const VGA_ADDRESS: *mut u8 = 0xb8000 as *mut u8;

pub const WHITE_ON_BLUE: u8 = 0x1f;
pub const GREEN_ON_BLACK: u8 = 0x0a;
pub const CYAN_ON_BLACK: u8 = 0x0b;
pub const RED_ON_BLACK: u8 = 0x0c;
pub const YELLOW_ON_BLACK: u8 = 0x0e;

pub unsafe fn clear_screen() {
    core::ptr::write_bytes(VGA_ADDRESS, 0, 4000);
}

pub unsafe fn write_string(row: usize, col: usize, s: &str, color: u8) {
    let offset = (row * 80 + col) * 2;
    for (i, &b) in s.as_bytes().iter().enumerate() {
        *VGA_ADDRESS.add(offset + i * 2) = b;
        *VGA_ADDRESS.add(offset + i * 2 + 1) = color;
    }
}
