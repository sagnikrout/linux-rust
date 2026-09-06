//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/page.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2009 Chen Liqin <liqin.chen@sunplusct.com>
// Copyright (C) 2012 Regents of the University of California
// Copyright (C) 2017 SiFive
// Copyright (C) 2017 XiaojingZhu <zhuxiaoj@ict.ac.cn>
//

//
// PAGE_OFFSET -- the first address of the first page of memory.
// When not using MMU this corresponds to the first free page in
// physical memory (aligned on a page boundary).
//

extern "C" {
    pub fn clear_page(page: *mut c_void);
}

//
// Use struct definitions to apply C type checking
//
// Page Global Directory entry
// Page Table entry

//
// We override this value as its generic definition uses __pa too early in
// the boot process (before kernel_map.va_pa_offset is set).
//
pub const MIN_MEMBLOCK_ADDR: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_mapping {
    pub virt_addr: c_ulong,
    pub virt_offset: c_ulong,
    pub phys_addr: uintptr_t,
    pub size: uintptr_t,
// Offset between linear mapping virtual address and kernel load address
    pub va_pa_offset: c_ulong,
// Offset between kernel mapping virtual address and kernel load address
    pub page_offset: c_ulong,
    pub va_kernel_pa_offset: c_ulong,
}

extern "C" {
    pub fn linear_mapping_va_to_pa(x: c_ulong) -> phys_addr_t;
}

extern "C" {
    pub fn __virt_to_phys(x: c_ulong) -> phys_addr_t;
}
extern "C" {
    pub fn __phys_addr_symbol(x: c_ulong) -> phys_addr_t;
}

extern "C" {
    pub fn __va(PAGE_SHIFT: pfn <<) -> return;
}

