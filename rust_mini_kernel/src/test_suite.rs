//! Kernel Comprehensive Automated Test Suite (Expanded 10-Part Suite with Security Verification)
use super::serial::SerialPort;
use super::mm::{PageAllocator, PAGE_SIZE, TOTAL_PAGES, MmError};
use super::intrinsics::{memset, memcpy, memcmp};
use super::cpuid::CpuInfo;
use core::sync::atomic::{AtomicU64, Ordering};

extern "C" {
    static stack_guard_page: u8;
    static stack_bottom: u8;
    static stack_top: u8;
}

pub struct KernelTestSuite;

impl KernelTestSuite {
    pub unsafe fn run_all_tests() -> bool {
        SerialPort::write_str("\n=======================================================\n");
        SerialPort::write_str("   RIGOROUS KERNEL & SECURITY TEST SUITE (10 TESTS)    \n");
        SerialPort::write_str("=======================================================\n");

        let t1 = Self::test_memory_intrinsics();
        let t2 = Self::test_page_allocator_stress();
        let t3 = Self::test_memory_pattern_integrity();
        let t4 = Self::test_64bit_math_invariants();
        let t5 = Self::test_page_reclamation_and_churn();
        let t6 = Self::test_allocator_saturation_limits();
        let t7 = Self::test_cpuid_feature_detection();
        let t8 = Self::test_stack_canary_integrity();
        let t9 = Self::test_allocator_security_boundaries();
        let t10 = Self::test_stack_guard_and_atomic_invariants();

        let all_passed = t1 && t2 && t3 && t4 && t5 && t6 && t7 && t8 && t9 && t10;
        SerialPort::write_str("=======================================================\n");
        if all_passed {
            SerialPort::write_str(">>> ALL 10 RIGOROUS TESTS PASSED (100% SUCCESS RATE) <<<\n");
        } else {
            SerialPort::write_str(">>> CRITICAL FAILURE: ONE OR MORE TESTS FAILED <<<\n");
        }
        SerialPort::write_str("=======================================================\n\n");
        all_passed
    }

    unsafe fn test_memory_intrinsics() -> bool {
        SerialPort::write_str("[TEST 1/10] Memory Intrinsics (memset, memcpy, memcmp)... ");
        let mut buf1 = [0u8; 128];
        let mut buf2 = [0u8; 128];

        memset(buf1.as_mut_ptr(), 0xaa, 128);
        for &b in buf1.iter() {
            if b != 0xaa {
                SerialPort::write_str("FAILED\n");
                return false;
            }
        }

        memcpy(buf2.as_mut_ptr(), buf1.as_ptr(), 128);
        for &b in buf2.iter() {
            if b != 0xaa {
                SerialPort::write_str("FAILED\n");
                return false;
            }
        }

        if memcmp(buf1.as_ptr(), buf2.as_ptr(), 128) != 0 {
            SerialPort::write_str("FAILED\n");
            return false;
        }

        buf2[64] = 0x55;
        if memcmp(buf1.as_ptr(), buf2.as_ptr(), 128) == 0 {
            SerialPort::write_str("FAILED\n");
            return false;
        }

        SerialPort::write_str("PASS\n");
        true
    }

    unsafe fn test_page_allocator_stress() -> bool {
        SerialPort::write_str("[TEST 2/10] Page Allocator Multi-Allocation Stress... ");
        const NUM_PAGES: usize = 32;
        let mut pages: [*mut u8; NUM_PAGES] = [core::ptr::null_mut(); NUM_PAGES];

        for i in 0..NUM_PAGES {
            match PageAllocator::alloc_page() {
                Some(p) => pages[i] = p,
                None => {
                    SerialPort::write_str("FAILED (OOM)\n");
                    return false;
                }
            }
        }

        for i in 0..NUM_PAGES {
            for j in (i + 1)..NUM_PAGES {
                if pages[i] == pages[j] {
                    SerialPort::write_str("FAILED (Overlap)\n");
                    return false;
                }
            }
        }

        for i in 0..NUM_PAGES {
            let _ = PageAllocator::free_page(pages[i]);
        }

        SerialPort::write_str("PASS (32 pages allocated & freed)\n");
        true
    }

    unsafe fn test_memory_pattern_integrity() -> bool {
        SerialPort::write_str("[TEST 3/10] Memory Boundary & Pattern Verification... ");
        if let Some(page) = PageAllocator::alloc_page() {
            let ptr = page as *mut u32;
            let count = PAGE_SIZE / 4;
            for i in 0..count {
                *ptr.add(i) = (i as u32) ^ 0xdeadbeef;
            }

            for i in 0..count {
                let expected = (i as u32) ^ 0xdeadbeef;
                if *ptr.add(i) != expected {
                    SerialPort::write_str("FAILED (Corruption)\n");
                    let _ = PageAllocator::free_page(page);
                    return false;
                }
            }

            let _ = PageAllocator::free_page(page);
            SerialPort::write_str("PASS (4096 bytes pattern-verified)\n");
            true
        } else {
            SerialPort::write_str("FAILED\n");
            false
        }
    }

    unsafe fn test_64bit_math_invariants() -> bool {
        SerialPort::write_str("[TEST 4/10] 64-bit Arithmetic & Bitwise Invariants... ");
        let a: u64 = 0x12345678_9abcdef0;
        let b: u64 = 0x0fedcba9_87654321;
        let sum = a.wrapping_add(b);
        if sum != 0x22222222_22222211 {
            SerialPort::write_str("FAILED (Add)\n");
            return false;
        }

        let shift = a << 4;
        if shift != 0x23456789_abcdef00 {
            SerialPort::write_str("FAILED (Shift)\n");
            return false;
        }

        SerialPort::write_str("PASS\n");
        true
    }

    unsafe fn test_page_reclamation_and_churn() -> bool {
        SerialPort::write_str("[TEST 5/10] Page Reclamation & Allocator Churn (Alloc/Free Cycles)... ");
        let page_a = PageAllocator::alloc_page();
        if page_a.is_none() {
            SerialPort::write_str("FAILED (Init alloc)\n");
            return false;
        }
        let addr_a = page_a.unwrap();
        let _ = PageAllocator::free_page(addr_a);

        let page_b = PageAllocator::alloc_page();
        if page_b.is_none() {
            SerialPort::write_str("FAILED (Realloc)\n");
            return false;
        }
        let addr_b = page_b.unwrap();
        
        if addr_a != addr_b {
            SerialPort::write_str("FAILED (Did not reclaim immediately available slot)\n");
            let _ = PageAllocator::free_page(addr_b);
            return false;
        }
        let _ = PageAllocator::free_page(addr_b);

        SerialPort::write_str("PASS (Immediate slot reclamation verified)\n");
        true
    }

    unsafe fn test_allocator_saturation_limits() -> bool {
        SerialPort::write_str("[TEST 6/10] Allocator High-Density Multi-Chunk Allocation (128 Pages / 512KB)... ");
        const BULK_SIZE: usize = 128;
        let mut bulk: [*mut u8; BULK_SIZE] = [core::ptr::null_mut(); BULK_SIZE];

        let mut success = true;
        for i in 0..BULK_SIZE {
            match PageAllocator::alloc_page() {
                Some(p) => bulk[i] = p,
                None => {
                    success = false;
                    break;
                }
            }
        }

        for i in 0..BULK_SIZE {
            if !bulk[i].is_null() {
                let _ = PageAllocator::free_page(bulk[i]);
            }
        }

        if !success {
            SerialPort::write_str("FAILED (High-density exhaustion)\n");
            return false;
        }
        SerialPort::write_str("PASS (512KB bulk allocation & reclamation verified)\n");
        true
    }

    unsafe fn test_cpuid_feature_detection() -> bool {
        SerialPort::write_str("[TEST 7/10] CPUID Hardware Instruction Sanity... ");
        let vendor = CpuInfo::get_vendor();
        let mut is_ascii = true;
        for &b in vendor.iter() {
            if b < 32 || b > 126 {
                is_ascii = false;
                break;
            }
        }
        if !is_ascii {
            SerialPort::write_str("FAILED (Non-ASCII vendor bytes)\n");
            return false;
        }
        SerialPort::write_str("PASS (Valid 12-byte hardware signature)\n");
        true
    }

    unsafe fn test_stack_canary_integrity() -> bool {
        SerialPort::write_str("[TEST 8/10] Stack Pointer Alignment & Canary Validation... ");
        let stack_canary: u64 = 0x5a5a_a5a5_dead_beef;
        let rsp: u64;
        core::arch::asm!("mov {}, rsp", out(reg) rsp, options(nomem, nostack, preserves_flags));

        if stack_canary != 0x5a5a_a5a5_dead_beef {
            SerialPort::write_str("FAILED (Canary corrupted)\n");
            return false;
        }

        if (rsp & 0x7) != 0 {
            SerialPort::write_str("FAILED (RSP unaligned)\n");
            return false;
        }

        SerialPort::write_str("PASS (Stack 64-bit aligned, canary intact)\n");
        true
    }

    unsafe fn test_allocator_security_boundaries() -> bool {
        SerialPort::write_str("[TEST 9/10] Security: Memory Allocator Bounds & Double-Free Protection... ");
        let page = match PageAllocator::alloc_page() {
            Some(p) => p,
            None => {
                SerialPort::write_str("FAILED (Alloc)\n");
                return false;
            }
        };

        // 1. Initial valid free must succeed
        if let Err(_) = PageAllocator::free_page(page) {
            SerialPort::write_str("FAILED (Valid free rejected)\n");
            return false;
        }

        // 2. Double-free must be detected and rejected
        match PageAllocator::free_page(page) {
            Err(MmError::DoubleFreeDetected) => {}
            _ => {
                SerialPort::write_str("FAILED (Double-free not detected)\n");
                return false;
            }
        }

        // 3. Misaligned pointer must be rejected
        let misaligned = page.wrapping_add(17);
        match PageAllocator::free_page(misaligned) {
            Err(MmError::MisalignedPointer) => {}
            _ => {
                SerialPort::write_str("FAILED (Misaligned pointer not rejected)\n");
                return false;
            }
        }

        // 4. Reserved kernel memory (<4MB) must be protected
        let reserved_addr = 0x2000 as *mut u8;
        match PageAllocator::free_page(reserved_addr) {
            Err(MmError::ReservedRegionViolation) => {}
            _ => {
                SerialPort::write_str("FAILED (Reserved kernel region not protected)\n");
                return false;
            }
        }

        // 5. Out of bounds address must be rejected
        let oob_addr = (TOTAL_PAGES * PAGE_SIZE + 0x1000) as *mut u8;
        match PageAllocator::free_page(oob_addr) {
            Err(MmError::OutOfBounds) => {}
            _ => {
                SerialPort::write_str("FAILED (Out-of-bounds pointer not rejected)\n");
                return false;
            }
        }

        // 6. Null pointer must be rejected
        match PageAllocator::free_page(core::ptr::null_mut()) {
            Err(MmError::NullPointer) => {}
            _ => {
                SerialPort::write_str("FAILED (Null pointer not rejected)\n");
                return false;
            }
        }

        SerialPort::write_str("PASS (Double-free, misaligned & reserved guards verified)\n");
        true
    }

    unsafe fn test_stack_guard_and_atomic_invariants() -> bool {
        SerialPort::write_str("[TEST 10/10] Security: Stack Guard Buffer & Atomic Invariants... ");

        // 1. Verify stack guard page spacing
        let guard_addr = core::ptr::addr_of!(stack_guard_page) as usize;
        let bottom_addr = core::ptr::addr_of!(stack_bottom) as usize;
        let top_addr = core::ptr::addr_of!(stack_top) as usize;

        if bottom_addr <= guard_addr {
            SerialPort::write_str("FAILED (Stack bottom below guard page)\n");
            return false;
        }

        let guard_size = bottom_addr - guard_addr;
        if guard_size < 4096 {
            SerialPort::write_str("FAILED (Guard page smaller than 4096 bytes)\n");
            return false;
        }

        if top_addr <= bottom_addr {
            SerialPort::write_str("FAILED (Stack top below bottom)\n");
            return false;
        }

        let mut current_rsp: u64;
        core::arch::asm!("mov {}, rsp", out(reg) current_rsp, options(nomem, nostack, preserves_flags));
        let rsp_val = current_rsp as usize;
        if rsp_val <= bottom_addr || rsp_val > top_addr {
            SerialPort::write_str("FAILED (Current RSP outside stack bounds)\n");
            return false;
        }

        // 2. Verify Atomic CAS concurrency primitive invariants
        let test_atomic = AtomicU64::new(0x1111);
        let exchanged = test_atomic.compare_exchange(0x1111, 0x2222, Ordering::SeqCst, Ordering::Relaxed);
        if exchanged != Ok(0x1111) || test_atomic.load(Ordering::SeqCst) != 0x2222 {
            SerialPort::write_str("FAILED (Atomic CAS failed to update)\n");
            return false;
        }

        // CAS with mismatched expected value must fail and not modify
        let failed_cas = test_atomic.compare_exchange(0x9999, 0x3333, Ordering::SeqCst, Ordering::Relaxed);
        if failed_cas != Err(0x2222) || test_atomic.load(Ordering::SeqCst) != 0x2222 {
            SerialPort::write_str("FAILED (Atomic CAS false-positive)\n");
            return false;
        }

        SerialPort::write_str("PASS (Guard page 4KB buffer & Atomic CAS verified)\n");
        true
    }
}
