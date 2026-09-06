use super::io::{inb, outb};

const PORT: u16 = 0x3f8; // COM1 base

pub struct SerialPort;

impl SerialPort {
    pub unsafe fn init() {
        outb(PORT + 1, 0x00); // Disable interrupts
        outb(PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
        outb(PORT + 0, 0x03); // Set divisor to 3 (38400 baud)
        outb(PORT + 1, 0x00);
        outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(PORT + 2, 0xc7); // Enable FIFO, clear them, 14-byte threshold
        outb(PORT + 4, 0x0b); // IRQs enabled, RTS/DSR set
    }

    pub unsafe fn is_transmit_empty() -> bool {
        (inb(PORT + 5) & 0x20) != 0
    }

    pub unsafe fn write_byte(b: u8) {
        while !Self::is_transmit_empty() {}
        outb(PORT, b);
    }

    pub unsafe fn write_str(s: &str) {
        for b in s.bytes() {
            if b == b'\n' {
                Self::write_byte(b'\r');
            }
            Self::write_byte(b);
        }
    }

    pub unsafe fn write_bytes(bytes: &[u8]) {
        for &b in bytes {
            Self::write_byte(b);
        }
    }
}
