//! Automatically rewritten from C to Rust
//! Source: tools/testing/vma/main.c
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

//
// Directly import the VMA implementation here. Our vma_internal.h wrapper
// provides userland-equivalent functionality for everything vma.c uses.
//

// Tests are included directly so they can test static functions in mm/vma.c.

    let mut __read_mostly: int sysctl_max_map_count = DEFAULT_MAX_MAP_COUNT;
// Helper functions which utilise static kernel functions.
    struct vm_area_struct *merge_existing(struct vma_merge_struct *vmg)
    {
    struct vm_area_struct *vma;
    vma = vma_merge_existing_range(vmg);
    if (vma)
    vma_assert_attached(vma);
    return vma;
    }
#[no_mangle]
pub unsafe extern "C" fn attach_vma(mm: *mut mm_struct, vma: *mut vm_area_struct) -> c_int {
    int attach_vma(struct mm_struct *mm, struct vm_area_struct *vma)
    {
    int res;
    res = vma_link(mm, vma);
    if (!res)
    vma_assert_attached(vma);
    return res;
    }
// Main test running which invokes tests/ *.c runners.
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut num_tests: c_int = 0, num_fail = 0;
    maple_tree_init();
    vma_state_init();
    run_merge_tests(&num_tests, &num_fail);
    run_mmap_tests(&num_tests, &num_fail);
    run_vma_tests(&num_tests, &num_fail);
    printf("%d tests run, %d passed, %d failed.\n",
    num_tests, num_tests - num_fail, num_fail);
    let mut num_fail: return = = 0 ? EXIT_SUCCESS : EXIT_FAILURE;
    }
