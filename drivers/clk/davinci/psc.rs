//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/davinci/psc.h
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
// Clock driver for TI Davinci PSC controllers
//
// Copyright (C) 2018 David Lechner <david@lechnology.com>
//

// PSC quirk flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_lpsc_clkdev_info {
    pub con_id: *const c_char,
    pub dev_id: *const c_char,
}

//
// davinci_lpsc_clk_info - LPSC module-specific clock information
// @name: the clock name
// @parent: the parent clock name
// @cdevs: optional array of clkdev lookup table info
// @md: the local module domain (LPSC id)
// @pd: the power domain id
// @flags: bitmask of LPSC_* flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_lpsc_clk_info {
    pub name: *const c_char,
    pub parent: *const c_char,
    pub cdevs: *const davinci_lpsc_clkdev_info,
    pub md: u32,
    pub pd: u32,
    pub flags: c_ulong,
}

// Device-specific data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_psc_init_data {
    pub parent_clks: *mut clk_bulk_data,
    pub num_parent_clks: c_int,
    pub base): *mut *mut *mut int (psc_init)(struct device dev, void __iomem,
}
