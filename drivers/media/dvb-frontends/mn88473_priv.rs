//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mn88473_priv.h
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
pub struct mn88473_dev {
    pub client: [*mut i2c_client; 3],
    pub regmap: [*mut regmap; 3],
    pub frontend: dvb_frontend,
    pub i2c_wr_max: u16,
    pub active: bool,
    pub clk: u32,
}
