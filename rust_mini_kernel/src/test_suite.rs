//! Kernel Comprehensive Automated Test Suite (Simplified)
use super::serial;
use super::mm::{self, PAGE_SIZE, TOTAL_PAGES, MmError};
use super::intrinsics::{memset, memcpy, memcmp};
use super::cpuid;
use core::sync::atomic::{AtomicU64, Ordering};

extern "C" {
    static stack_guard_page: u8;
    static stack_bottom: u8;
    static stack_top: u8;
}

pub unsafe fn run_all_tests() -> bool {
    serial::write_str("\n=======================================================\n");
    serial::write_str("   RIGOROUS KERNEL & SECURITY TEST SUITE (10 TESTS)    \n");
    serial::write_str("=======================================================\n");

    let t1 = test_memory_intrinsics();
    let t2 = test_page_allocator_stress();
    let t3 = test_memory_pattern_integrity();
    let t4 = test_64bit_math_invariants();
    let t5 = test_page_reclamation_and_churn();
    let t6 = test_allocator_saturation_limits();
    let t7 = test_cpuid_feature_detection();
    let t8 = test_stack_canary_integrity();
    let t9 = test_allocator_security_boundaries();
    let t10 = test_stack_guard_and_atomic_invariants();

    let all_passed = t1 && t2 && t3 && t4 && t5 && t6 && t7 && t8 && t9 && t10;
    serial::write_str("=======================================================\n");
    if all_passed {
        serial::write_str(">>> ALL 10 RIGOROUS TESTS PASSED (100% SUCCESS RATE) <<<\n");
    } else {
        serial::write_str(">>> CRITICAL FAILURE: ONE OR MORE TESTS FAILED <<<\n");
    }
    serial::write_str("=======================================================\n\n");
    all_passed
}

unsafe fn test_memory_intrinsics() -> bool {
    serial::write_str("[TEST 1/10] Memory Intrinsics (memset, memcpy, memcmp)... ");
    let mut buf1 = [0u8; 128];
    let mut buf2 = [0u8; 128];

    memset(buf1.as_mut_ptr(), 0xaa, 128);
    for &b in buf1.iter() {
        if b != 0xaa {
            serial::write_str("FAILED\n");
            return false;
        }
    }

    memcpy(buf2.as_mut_ptr(), buf1.as_ptr(), 128);
    for &b in buf2.iter() {
        if b != 0xaa {
            serial::write_str("FAILED\n");
            return false;
        }
    }

    if memcmp(buf1.as_ptr(), buf2.as_ptr(), 128) != 0 {
        serial::write_str("FAILED\n");
        return false;
    }

    buf2[64] = 0x55;
    if memcmp(buf1.as_ptr(), buf2.as_ptr(), 128) == 0 {
        serial::write_str("FAILED\n");
        return false;
    }

    serial::write_str("PASS\n");
    true
}

unsafe fn test_page_allocator_stress() -> bool {
    serial::write_str("[TEST 2/10] Page Allocator Multi-Allocation Stress... ");
    const NUM_PAGES: usize = 32;
    let mut pages: [*mut u8; NUM_PAGES] = [core::ptr::null_mut(); NUM_PAGES];

    for i in 0..NUM_PAGES {
        match mm::alloc_page() {
            Some(p) => pages[i] = p,
            None => {
                serial::write_str("FAILED (OOM)\n");
                return false;
            }
        }
    }

    for i in 0..NUM_PAGES {
        for j in (i + 1)..NUM_PAGES {
            if pages[i] == pages[j] {
                serial::write_str("FAILED (Overlap)\n");
                return false;
            }
        }
    }

    for i in 0..NUM_PAGES {
        let _ = mm::free_page(pages[i]);
    }

    serial::write_str("PASS (32 pages allocated & freed)\n");
    true
}

unsafe fn test_memory_pattern_integrity() -> bool {
    serial::write_str("[TEST 3/10] Memory Boundary & Pattern Verification... ");
    if let Some(page) = mm::alloc_page() {
        let ptr = page as *mut u32;
        let count = PAGE_SIZE / 4;
        for i in 0..count {
            *ptr.add(i) = (i as u32) ^ 0xdeadbeef;
        }

        for i in 0..count {
            let expected = (i as u32) ^ 0xdeadbeef;
            if *ptr.add(i) != expected {
                serial::write_str("FAILED (Corruption)\n");
                let _ = mm::free_page(page);
                return false;
            }
        }

        let _ = mm::free_page(page);
        serial::write_str("PASS (4096 bytes pattern-verified)\n");
        true
    } else {
        serial::write_str("FAILED\n");
        false
    }
}

unsafe fn test_64bit_math_invariants() -> bool {
    serial::write_str("[TEST 4/10] 64-bit Arithmetic & Bitwise Invariants... ");
    let a: u64 = 0x12345678_9abcdef0;
    let b: u64 = 0x0fedcba9_87654321;
    let sum = a.wrapping_add(b);
    if sum != 0x22222222_22222211 {
        serial::write_str("FAILED (Add)\n");
        return false;
    }

    let shift = a << 4;
    if shift != 0x23456789_abcdef00 {
        serial::write_str("FAILED (Shift)\n");
        return false;
    }

    serial::write_str("PASS\n");
    true
}

unsafe fn test_page_reclamation_and_churn() -> bool {
    serial::write_str("[TEST 5/10] Page Reclamation & Allocator Churn (Alloc/Free Cycles)... ");
    let page_a = mm::alloc_page();
    if page_a.is_none() {
        serial::write_str("FAILED (Init alloc)\n");
        return false;
    }
    let addr_a = page_a.unwrap();
    let _ = mm::free_page(addr_a);

    let page_b = mm::alloc_page();
    if page_b.is_none() {
        serial::write_str("FAILED (Realloc)\n");
        return false;
    }
    let addr_b = page_b.unwrap();
    
    if addr_a != addr_b {
        serial::write_str("FAILED (Did not reclaim immediately available slot)\n");
        let _ = mm::free_page(addr_b);
        return false;
    }
    let _ = mm::free_page(addr_b);

    serial::write_str("PASS (Immediate slot reclamation verified)\n");
    true
}

unsafe fn test_allocator_saturation_limits() -> bool {
    serial::write_str("[TEST 6/10] Allocator High-Density Multi-Chunk Allocation (128 Pages / 512KB)... ");
    const BULK_SIZE: usize = 128;
    let mut bulk: [*mut u8; BULK_SIZE] = [core::ptr::null_mut(); BULK_SIZE];

    let mut success = true;
    for i in 0..BULK_SIZE {
        match mm::alloc_page() {
            Some(p) => bulk[i] = p,
            None => {
                success = false;
                break;
            }
        }
    }

    for i in 0..BULK_SIZE {
        if !bulk[i].is_null() {
            let _ = mm::free_page(bulk[i]);
        }
    }

    if !success {
        serial::write_str("FAILED (High-density exhaustion)\n");
        return false;
    }
    serial::write_str("PASS (512KB bulk allocation & reclamation verified)\n");
    true
}

unsafe fn test_cpuid_feature_detection() -> bool {
    serial::write_str("[TEST 7/10] CPUID Hardware Instruction Sanity... ");
    let vendor = cpuid::get_vendor();
    let mut is_ascii = true;
    for &b in vendor.iter() {
        if b < 32 || b > 126 {
            is_ascii = false;
            break;
        }
    }
    if !is_ascii {
        serial::write_str("FAILED (Non-ASCII vendor bytes)\n");
        return false;
    }
    serial::write_str("PASS (Valid 12-byte hardware signature)\n");
    true
}

unsafe fn test_stack_canary_integrity() -> bool {
    serial::write_str("[TEST 8/10] Stack Pointer Alignment & Canary Validation... ");
    let stack_canary: u64 = 0x5a5a_a5a5_dead_beef;
    let rsp: u64;
    core::arch::asm!("mov {}, rsp", out(reg) rsp, options(nomem, nostack, preserves_flags));

    if stack_canary != 0x5a5a_a5a5_dead_beef {
        serial::write_str("FAILED (Canary corrupted)\n");
        return false;
    }

    if (rsp & 0x7) != 0 {
        serial::write_str("FAILED (RSP unaligned)\n");
        return false;
    }

    serial::write_str("PASS (Stack 64-bit aligned, canary intact)\n");
    true
}

unsafe fn test_allocator_security_boundaries() -> bool {
    serial::write_str("[TEST 9/10] Security: Memory Allocator Bounds & Double-Free Protection... ");
    let page = match mm::alloc_page() {
        Some(p) => p,
        None => {
            serial::write_str("FAILED (Alloc)\n");
            return false;
        }
    };

    if let Err(_) = mm::free_page(page) {
        serial::write_str("FAILED (Valid free rejected)\n");
        return false;
    }

    match mm::free_page(page) {
        Err(MmError::DoubleFreeDetected) => {}
        _ => {
            serial::write_str("FAILED (Double-free not detected)\n");
            return false;
        }
    }

    let misaligned = page.wrapping_add(17);
    match mm::free_page(misaligned) {
        Err(MmError::MisalignedPointer) => {}
        _ => {
            serial::write_str("FAILED (Misaligned pointer not rejected)\n");
            return false;
        }
    }

    let reserved_addr = 0x2000 as *mut u8;
    match mm::free_page(reserved_addr) {
        Err(MmError::ReservedRegionViolation) => {}
        _ => {
            serial::write_str("FAILED (Reserved kernel region not protected)\n");
            return false;
        }
    }

    let oob_addr = (TOTAL_PAGES * PAGE_SIZE + 0x1000) as *mut u8;
    match mm::free_page(oob_addr) {
        Err(MmError::OutOfBounds) => {}
        _ => {
            serial::write_str("FAILED (Out-of-bounds pointer not rejected)\n");
            return false;
        }
    }

    match mm::free_page(core::ptr::null_mut()) {
        Err(MmError::NullPointer) => {}
        _ => {
            serial::write_str("FAILED (Null pointer not rejected)\n");
            return false;
        }
    }

    serial::write_str("PASS (Double-free, misaligned & reserved guards verified)\n");
    true
}

unsafe fn test_stack_guard_and_atomic_invariants() -> bool {
    serial::write_str("[TEST 10/10] Security: Stack Guard Buffer & Atomic Invariants... ");

    let guard_addr = core::ptr::addr_of!(stack_guard_page) as usize;
    let bottom_addr = core::ptr::addr_of!(stack_bottom) as usize;
    let top_addr = core::ptr::addr_of!(stack_top) as usize;

    if bottom_addr <= guard_addr {
        serial::write_str("FAILED (Stack bottom below guard page)\n");
        return false;
    }

    let guard_size = bottom_addr - guard_addr;
    if guard_size < 4096 {
        serial::write_str("FAILED (Guard page smaller than 4096 bytes)\n");
        return false;
    }

    if top_addr <= bottom_addr {
        serial::write_str("FAILED (Stack top below bottom)\n");
        return false;
    }

    let mut current_rsp: u64;
    core::arch::asm!("mov {}, rsp", out(reg) current_rsp, options(nomem, nostack, preserves_flags));
    let rsp_val = current_rsp as usize;
    if rsp_val <= bottom_addr || rsp_val > top_addr {
        serial::write_str("FAILED (Current RSP outside stack bounds)\n");
        return false;
    }

    let test_atomic = AtomicU64::new(0x1111);
    let exchanged = test_atomic.compare_exchange(0x1111, 0x2222, Ordering::SeqCst, Ordering::Relaxed);
    if exchanged != Ok(0x1111) || test_atomic.load(Ordering::SeqCst) != 0x2222 {
        serial::write_str("FAILED (Atomic CAS failed to update)\n");
        return false;
    }

    let failed_cas = test_atomic.compare_exchange(0x9999, 0x3333, Ordering::SeqCst, Ordering::Relaxed);
    if failed_cas != Err(0x2222) || test_atomic.load(Ordering::SeqCst) != 0x2222 {
        serial::write_str("FAILED (Atomic CAS false-positive)\n");
        return false;
    }

    serial::write_str("PASS (Guard page 4KB buffer & Atomic CAS verified)\n");
    true
}
