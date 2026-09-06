use super::io::{inb, outb};

const PORT: u16 = 0x3f8;

pub unsafe fn init() {
    outb(PORT + 1, 0x00);
    outb(PORT + 3, 0x80);
    outb(PORT + 0, 0x03);
    outb(PORT + 1, 0x00);
    outb(PORT + 3, 0x03);
    outb(PORT + 2, 0xc7);
    outb(PORT + 4, 0x0b);
}

pub unsafe fn write_byte(b: u8) {
    let mut timeout: u32 = 100_000;
    while (inb(PORT + 5) & 0x20) == 0 {
        timeout -= 1;
        if timeout == 0 {
            return;
        }
        core::hint::spin_loop();
    }
    outb(PORT, b);
}

pub unsafe fn write_str(s: &str) {
    for b in s.bytes() {
        if b == b'\n' {
            write_byte(b'\r');
        }
        write_byte(b);
    }
}

pub unsafe fn write_bytes(bytes: &[u8]) {
    for &b in bytes {
        write_byte(b);
    }
}
