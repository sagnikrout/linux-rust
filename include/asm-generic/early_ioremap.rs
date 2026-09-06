//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/early_ioremap.h
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

//
// early_ioremap() and early_iounmap() are for temporary early boot-time
// mappings, before the real ioremap() is functional.
//
extern "C" {
    pub fn early_iounmap(addr: *mut void __iomem, size: c_ulong);
}
extern "C" {
    pub fn early_memunmap(addr: *mut c_void, size: c_ulong);
}

// Arch-specific initialization
extern "C" {
    pub fn early_ioremap_init();
}
// Generic initialization called by architecture code
extern "C" {
    pub fn early_ioremap_setup();
}
//
// Called as last step in paging_init() so library can act
// accordingly for subsequent map/unmap requests.
//
extern "C" {
    pub fn early_ioremap_reset();
}
//
// Early copy from unmapped memory to kernel mapped memory.
//

