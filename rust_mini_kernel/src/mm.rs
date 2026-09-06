use super::serial::SerialPort;
use core::sync::atomic::{AtomicU64, Ordering};

pub const PAGE_SIZE: usize = 4096;
pub const TOTAL_PAGES: usize = 32768; // 128 MB managed physical pool
pub const RESERVED_PAGES: usize = 1024; // First 4MB reserved for kernel & page tables

const SLOTS: usize = TOTAL_PAGES / 64;

// Safe, thread-safe lock-free atomic bitmap
static ALLOC_BITMAP: [AtomicU64; SLOTS] = {
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; SLOTS]
};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum MmError {
    NullPointer,
    MisalignedPointer,
    ReservedRegionViolation,
    OutOfBounds,
    DoubleFreeDetected,
    OutOfMemory,
}

pub struct PageAllocator;

impl PageAllocator {
    pub unsafe fn init() {
        let mut i = 0;
        while i < RESERVED_PAGES {
            let idx = i / 64;
            let bit = i % 64;
            ALLOC_BITMAP[idx].fetch_or(1 << bit, Ordering::SeqCst);
            i += 1;
        }
        SerialPort::write_str("[MM-HARDENED] Lock-Free Atomic Allocator Active with Bounds Checking\n");
    }

    pub unsafe fn alloc_page() -> Option<*mut u8> {
        let mut idx = 0;
        while idx < SLOTS {
            let mut current = ALLOC_BITMAP[idx].load(Ordering::Relaxed);
            while current != !0 {
                let bit = (!current).trailing_zeros() as usize;
                if bit >= 64 {
                    break;
                }
                let mask = 1 << bit;
                // Atomic CAS to ensure lock-free concurrency safety across cores
                match ALLOC_BITMAP[idx].compare_exchange_weak(
                    current,
                    current | mask,
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        let page_idx = idx * 64 + bit;
                        let addr = (page_idx * PAGE_SIZE) as *mut u8;
                        core::ptr::write_bytes(addr, 0, PAGE_SIZE);
                        return Some(addr);
                    }
                    Err(actual) => {
                        current = actual;
                    }
                }
            }
            idx += 1;
        }
        None
    }

    pub unsafe fn free_page(ptr: *mut u8) -> Result<(), MmError> {
        if ptr.is_null() {
            return Err(MmError::NullPointer);
        }

        let addr = ptr as usize;
        // SEC-02: Enforce strict 4096-byte page alignment
        if (addr % PAGE_SIZE) != 0 {
            return Err(MmError::MisalignedPointer);
        }

        let page_idx = addr / PAGE_SIZE;
        // SEC-02: Protect reserved kernel memory (<4MB)
        if page_idx < RESERVED_PAGES {
            return Err(MmError::ReservedRegionViolation);
        }

        if page_idx >= TOTAL_PAGES {
            return Err(MmError::OutOfBounds);
        }

        let idx = page_idx / 64;
        let bit = page_idx % 64;
        let mask = 1 << bit;

        // SEC-02 & SEC-06: Atomic check-and-clear to prevent double-frees and race conditions
        let previous = ALLOC_BITMAP[idx].fetch_and(!mask, Ordering::SeqCst);
        if (previous & mask) == 0 {
            // Bit was ALREADY 0 -> Double free attempt detected!
            return Err(MmError::DoubleFreeDetected);
        }

        Ok(())
    }
}
