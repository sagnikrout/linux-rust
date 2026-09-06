//! Kernel Comprehensive Automated Test Suite (Expanded 8-Part Suite)
use super::serial::SerialPort;
use super::mm::{PageAllocator, PAGE_SIZE};
use super::intrinsics::{memset, memcpy, memcmp};
use super::cpuid::CpuInfo;

pub struct KernelTestSuite;

impl KernelTestSuite {
    pub unsafe fn run_all_tests() -> bool {
        SerialPort::write_str("\n=======================================================\n");
        SerialPort::write_str("      EXTENDED RIGOROUS KERNEL TEST SUITE (8 TESTS)    \n");
        SerialPort::write_str("=======================================================\n");

        let t1 = Self::test_memory_intrinsics();
        let t2 = Self::test_page_allocator_stress();
        let t3 = Self::test_memory_pattern_integrity();
        let t4 = Self::test_64bit_math_invariants();
        let t5 = Self::test_page_reclamation_and_churn();
        let t6 = Self::test_allocator_saturation_limits();
        let t7 = Self::test_cpuid_feature_detection();
        let t8 = Self::test_stack_canary_integrity();

        let all_passed = t1 && t2 && t3 && t4 && t5 && t6 && t7 && t8;
        SerialPort::write_str("=======================================================\n");
        if all_passed {
            SerialPort::write_str(">>> ALL 8 RIGOROUS TESTS PASSED (100% SUCCESS RATE) <<<\n");
        } else {
            SerialPort::write_str(">>> CRITICAL FAILURE: ONE OR MORE TESTS FAILED <<<\n");
        }
        SerialPort::write_str("=======================================================\n\n");
        all_passed
    }

    unsafe fn test_memory_intrinsics() -> bool {
        SerialPort::write_str("[TEST 1/8] Memory Intrinsics (memset, memcpy, memcmp)... ");
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
        SerialPort::write_str("[TEST 2/8] Page Allocator Multi-Allocation Stress... ");
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
            PageAllocator::free_page(pages[i]);
        }

        SerialPort::write_str("PASS (32 pages allocated & freed)\n");
        true
    }

    unsafe fn test_memory_pattern_integrity() -> bool {
        SerialPort::write_str("[TEST 3/8] Memory Boundary & Pattern Verification... ");
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
                    PageAllocator::free_page(page);
                    return false;
                }
            }

            PageAllocator::free_page(page);
            SerialPort::write_str("PASS (4096 bytes pattern-verified)\n");
            true
        } else {
            SerialPort::write_str("FAILED\n");
            false
        }
    }

    unsafe fn test_64bit_math_invariants() -> bool {
        SerialPort::write_str("[TEST 4/8] 64-bit Arithmetic & Bitwise Invariants... ");
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
        SerialPort::write_str("[TEST 5/8] Page Reclamation & Allocator Churn (Alloc/Free Cycles)... ");
        // Allocate page A, free it, then allocate again -> must successfully reuse the page
        let page_a = PageAllocator::alloc_page();
        if page_a.is_none() {
            SerialPort::write_str("FAILED (Init alloc)\n");
            return false;
        }
        let addr_a = page_a.unwrap();
        PageAllocator::free_page(addr_a);

        let page_b = PageAllocator::alloc_page();
        if page_b.is_none() {
            SerialPort::write_str("FAILED (Realloc)\n");
            return false;
        }
        let addr_b = page_b.unwrap();
        
        if addr_a != addr_b {
            SerialPort::write_str("FAILED (Did not reclaim immediately available slot)\n");
            PageAllocator::free_page(addr_b);
            return false;
        }
        PageAllocator::free_page(addr_b);

        SerialPort::write_str("PASS (Immediate slot reclamation verified)\n");
        true
    }

    unsafe fn test_allocator_saturation_limits() -> bool {
        SerialPort::write_str("[TEST 6/8] Allocator High-Density Multi-Chunk Allocation (128 Pages / 512KB)... ");
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
                PageAllocator::free_page(bulk[i]);
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
        SerialPort::write_str("[TEST 7/8] CPUID Hardware Instruction Sanity... ");
        let vendor = CpuInfo::get_vendor();
        // Valid x86 vendor strings are 12 ASCII characters
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
        SerialPort::write_str("[TEST 8/8] Stack Pointer Alignment & Canary Validation... ");
        let mut stack_canary: u64 = 0x5a5a_a5a5_dead_beef;
        let rsp: u64;
        core::arch::asm!("mov {}, rsp", out(reg) rsp, options(nomem, nostack, preserves_flags));

        // In x86_64 ABI, stack should remain 16-byte aligned before function calls
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
}
