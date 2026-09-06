//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/abs_lowcore.c
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


// SPDX-License-Identifier: GPL-2.0

    unsigned long __bootdata_preserved(__abs_lowcore);
#[no_mangle]
pub unsafe extern "C" fn abs_lowcore_map(cpu: c_int, lc: *mut lowcore, alloc: bool) -> c_int {
    int abs_lowcore_map(int cpu, struct lowcore *lc, bool alloc)
    {
    let mut addr: c_ulong = __abs_lowcore + (cpu * sizeof(struct lowcore));
    let mut phys: c_ulong = __pa(lc);
    int rc, i;
    for (i = 0; i < LC_PAGES; i++) {
    rc = __vmem_map_4k_page(addr, phys, PAGE_KERNEL, alloc);
    if (rc) {
//
// Do not unmap allocated page tables in case the
// allocation was not requested. In such a case the
// request is expected coming from an atomic context,
// while the unmap attempt might sleep.
//
    if (alloc) {
    for (--i; i >= 0; i--) {
    addr -= PAGE_SIZE;
    vmem_unmap_4k_page(addr);
    }
    }
    return rc;
    }
    addr += PAGE_SIZE;
    phys += PAGE_SIZE;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn abs_lowcore_unmap(cpu: c_int) {
    void abs_lowcore_unmap(int cpu)
    {
    let mut addr: c_ulong = __abs_lowcore + (cpu * sizeof(struct lowcore));
    int i;
    for (i = 0; i < LC_PAGES; i++) {
    vmem_unmap_4k_page(addr);
    addr += PAGE_SIZE;
    }
    }
