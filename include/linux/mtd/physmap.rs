//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/physmap.h
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
// For boards with physically mapped flash and using
// drivers/mtd/maps/physmap.c mapping driver.
//
// Copyright (C) 2003 MontaVista Software Inc.
// Author: Jun Sun, jsun@mvista.com or jsun@junsun.net
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct physmap_flash_data {
    pub width: c_uint,
    pub ): *mut *mut int (init)(struct platform_device,
    pub ): *mut *mut void (exit)(struct platform_device,
    pub int): *mut *mut *mut void (set_vpp)(struct platform_device ,,
    pub nr_parts: c_uint,
    pub pfow_base: c_uint,
    pub probe_type: *mut c_char,
    pub parts: *mut mtd_partition,
    pub part_probe_types: *const *const c_char,
}
