//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mn88473.h
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
// Panasonic MN88473 DVB-T/T2/C demodulator driver
//
// Copyright (C) 2014 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mn88473_config {
//
// Max num of bytes given I2C adapter could write at once.
// Default: unlimited
//
    pub i2c_wr_max: u16,
//
// Xtal frequency Hz.
// Default: 25000000
//
    pub xtal: u32,
// Everything after that is returned by the driver.
//
// DVB frontend.
//
    pub fe: *mut dvb_frontend,
}
