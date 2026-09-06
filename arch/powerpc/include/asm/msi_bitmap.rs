//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/msi_bitmap.h
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
// Copyright 2008, Michael Ellerman, IBM Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_bitmap {
    pub of_node: *mut device_node,
    pub bitmap: *mut c_ulong,
    pub lock: spinlock_t,
    pub irq_count: c_uint,
    pub bitmap_from_slab: bool,
}

extern "C" {
    pub fn msi_bitmap_alloc_hwirqs(bmp: *mut msi_bitmap, num: c_int) -> c_int;
}
extern "C" {
    pub fn msi_bitmap_reserve_hwirq(bmp: *mut msi_bitmap, hwirq: c_uint);
}
extern "C" {
    pub fn msi_bitmap_reserve_dt_hwirqs(bmp: *mut msi_bitmap) -> c_int;
}
extern "C" {
    pub fn msi_bitmap_free(bmp: *mut msi_bitmap);
}
