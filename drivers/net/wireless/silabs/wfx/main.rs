//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/main.h
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
// Device probe and register.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright 2004-2006 Jean-Baptiste Note <jbnote@gmail.com>, et al.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_platform_data {
// Keyset and ".sec" extension will be appended to this string
    pub file_fw: *const c_char,
    pub file_pds: *const c_char,
    pub gpio_wakeup: *mut gpio_desc,
// if true HIF D_out is sampled on the rising edge of the clock (intended to be used in
// 50Mhz SDIO)
//
    pub use_rising_clk: bool,
}

extern "C" {
    pub fn wfx_probe(wdev: *mut wfx_dev) -> c_int;
}
extern "C" {
    pub fn wfx_release(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_api_older_than(wdev: *mut wfx_dev, major: c_int, minor: c_int) -> bool;
}
extern "C" {
    pub fn wfx_send_pds(wdev: *mut wfx_dev, buf: *mut u8, len: usize) -> c_int;
}
