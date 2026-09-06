use core::arch::asm;
use super::serial::SerialPort;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

impl IdtEntry {
    pub const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            zero: 0,
        }
    }

    pub fn set_handler(&mut self, handler: usize) {
        self.offset_low = handler as u16;
        self.selector = 0x08;
        self.ist = 0;
        self.type_attr = 0x8e;
        self.offset_mid = (handler >> 16) as u16;
        self.offset_high = (handler >> 32) as u32;
        self.zero = 0;
    }
}

#[repr(C, packed)]
pub struct IdtPointer {
    limit: u16,
    base: u64,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];

extern "C" {
    fn default_exception_handler();
}

pub unsafe fn init_idt() {
    let handler_addr = default_exception_handler as *const () as usize;
    let mut i = 0;
    while i < 256 {
        IDT[i].set_handler(handler_addr);
        i += 1;
    }

    let ptr = IdtPointer {
        limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
        base: core::ptr::addr_of!(IDT) as u64,
    };

    asm!("lidt [{}]", in(reg) &ptr, options(readonly, nostack, preserves_flags));
    SerialPort::write_str("[IDT] 256-entry Interrupt Descriptor Table active\n");
}
