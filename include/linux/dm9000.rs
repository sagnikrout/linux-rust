//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dm9000.h
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
// include/linux/dm9000.h
//
// Copyright (c) 2004 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// Header file for dm9000 platform data
//

// IO control flags

// platform data for platform device structure's platform_data field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm9000_plat_data {
    pub flags: c_uint,
    pub dev_addr: [c_uchar; ETH_ALEN],
// allow replacement IO routines
    pub len): *mut *mut *mut *mut void (inblk)(void __iomem reg, void data, int,
    pub len): *mut *mut *mut *mut void (outblk)(void __iomem reg, void data, int,
    pub len): *mut *mut *mut void (dumpblk)(void __iomem reg, int,
}
