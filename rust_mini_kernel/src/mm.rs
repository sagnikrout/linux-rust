use super::serial::SerialPort;

pub const PAGE_SIZE: usize = 4096;
const TOTAL_PAGES: usize = 32768; // 128 MB managed physical pool
static mut ALLOC_BITMAP: [u64; TOTAL_PAGES / 64] = [0; TOTAL_PAGES / 64];

pub struct PageAllocator;

impl PageAllocator {
    pub unsafe fn init() {
        let reserved_pages = 1024;
        let mut i = 0;
        while i < reserved_pages {
            let idx = i / 64;
            let bit = i % 64;
            ALLOC_BITMAP[idx] |= 1 << bit;
            i += 1;
        }
        SerialPort::write_str("[MM] 128MB Physical Page Frame Allocator active (4096B pages)\n");
    }

    pub unsafe fn alloc_page() -> Option<*mut u8> {
        let mut idx = 0;
        while idx < (TOTAL_PAGES / 64) {
            let slot = ALLOC_BITMAP[idx];
            if slot != !0 {
                let mut bit = 0;
                while bit < 64 {
                    if (slot & (1 << bit)) == 0 {
                        ALLOC_BITMAP[idx] |= 1 << bit;
                        let page_idx = idx * 64 + bit;
                        let addr = (page_idx * PAGE_SIZE) as *mut u8;
                        core::ptr::write_bytes(addr, 0, PAGE_SIZE);
                        return Some(addr);
                    }
                    bit += 1;
                }
            }
            idx += 1;
        }
        None
    }

    pub unsafe fn free_page(ptr: *mut u8) {
        let addr = ptr as usize;
        let page_idx = addr / PAGE_SIZE;
        let idx = page_idx / 64;
        let bit = page_idx % 64;
        if idx < (TOTAL_PAGES / 64) {
            ALLOC_BITMAP[idx] &= !(1 << bit);
        }
    }
}
