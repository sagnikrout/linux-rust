//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mn88443x.h
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
// Socionext MN88443x series demodulator driver for ISDB-S/ISDB-T.
//
// Copyright (c) 2018 Socionext Inc.
//

// ISDB-T IF frequency
pub const DIRECT_IF_57MHZ: c_int = 57000000;
pub const DIRECT_IF_44MHZ: c_int = 44000000;
pub const LOW_IF_4MHZ: c_int = 4000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mn88443x_config {
    pub mclk: *mut clk,
    pub if_freq: u32,
    pub reset_gpio: *mut gpio_desc,
// Everything after that is returned by the driver.
    pub fe: *mut dvb_frontend,
}
