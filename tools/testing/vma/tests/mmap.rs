//! Automatically rewritten from C to Rust
//! Source: tools/testing/vma/tests/mmap.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-or-later
#[no_mangle]
unsafe extern "C" fn test_mmap_region_basic() -> bool {
    static bool test_mmap_region_basic(void)
    {
    const vma_flags_t vma_flags = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT,
    VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = {};
    unsigned long addr;
    struct vm_area_struct *vma;
    VMA_ITERATOR(vmi, &mm, 0);
    current.mm = &mm;
// Map at 0x300000, length 0x3000.
    addr = __mmap_region(core::ptr::null_mut(), 0x300000, 0x3000, vma_flags, 0x300, core::ptr::null_mut());
    ASSERT_EQ(addr, 0x300000);
// Map at 0x250000, length 0x3000.
    addr = __mmap_region(core::ptr::null_mut(), 0x250000, 0x3000, vma_flags, 0x250, core::ptr::null_mut());
    ASSERT_EQ(addr, 0x250000);
// Map at 0x303000, merging to 0x300000 of length 0x6000.
    addr = __mmap_region(core::ptr::null_mut(), 0x303000, 0x3000, vma_flags, 0x303, core::ptr::null_mut());
    ASSERT_EQ(addr, 0x303000);
// Map at 0x24d000, merging to 0x250000 of length 0x6000.
    addr = __mmap_region(core::ptr::null_mut(), 0x24d000, 0x3000, vma_flags, 0x24d, core::ptr::null_mut());
    ASSERT_EQ(addr, 0x24d000);
    ASSERT_EQ(mm.map_count, 2);
    for_each_vma(vmi, vma) {
    if (vma.vm_start == 0x300000) {
    ASSERT_EQ(vma.vm_end, 0x306000);
    ASSERT_EQ(vma.vm_pgoff, 0x300);
    } else if (vma.vm_start == 0x24d000) {
    ASSERT_EQ(vma.vm_end, 0x253000);
    ASSERT_EQ(vma.vm_pgoff, 0x24d);
    } else {
    ASSERT_FALSE(true);
    }
    }
    cleanup_mm(&mm, &vmi);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn run_mmap_tests(num_tests: *mut c_int, num_fail: *mut c_int) {
    static void run_mmap_tests(int *num_tests, int *num_fail)
    {
    TEST(mmap_region_basic);
    }
