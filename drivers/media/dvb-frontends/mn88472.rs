//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mn88472.h
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
// Panasonic MN88472 DVB-T/T2/C demodulator driver
//
// Copyright (C) 2013 Antti Palosaari <crope@iki.fi>
//

// Define old names for backward compatibility

//
// struct mn88472_config - Platform data for the mn88472 driver
// @xtal: Clock frequency.
// @ts_mode: TS mode.
// @ts_clock: TS clock config.
// @i2c_wr_max: Max number of bytes driver writes to I2C at once.
// @fe: pointer to a frontend pointer
// @get_dvb_frontend: Get DVB frontend callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mn88472_config {
    pub xtal: c_uint,
pub const MN88472_TS_MODE_SERIAL: c_int = 0;
pub const MN88472_TS_MODE_PARALLEL: c_int = 1;
    pub ts_mode: c_int,
pub const MN88472_TS_CLK_FIXED: c_int = 0;
pub const MN88472_TS_CLK_VARIABLE: c_int = 1;
    pub ts_clock: c_int,
    pub i2c_wr_max: u16,
// Everything after that is returned by the driver.
//
// DVB frontend.
//
    pub fe: *mut dvb_frontend,
    pub ): *mut *mut *mut dvb_frontend (get_dvb_frontend)(i2c_client,
}
