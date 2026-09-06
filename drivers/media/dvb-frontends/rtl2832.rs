//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/rtl2832.h
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
// Realtek RTL2832 DVB-T demodulator driver
//
// Copyright (C) 2012 Thomas Mair <thomas.mair86@gmail.com>
// Copyright (C) 2012-2014 Antti Palosaari <crope@iki.fi>
//

//
// struct rtl2832_platform_data - Platform data for the rtl2832 driver
// @clk: Clock frequency (4000000, 16000000, 25000000, 28800000).
// @tuner: Used tuner model.
// @get_dvb_frontend: Get DVB frontend.
// @get_i2c_adapter: Get I2C adapter.
// @slave_ts_ctrl: Control slave TS interface.
// @pid_filter: Set PID to PID filter.
// @pid_filter_ctrl: Control PID filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl2832_platform_data {
    pub clk: u32,
//
// XXX: This list must be kept sync with dvb_usb_rtl28xxu USB IF driver.
//
pub const RTL2832_TUNER_FC2580: c_uint = 0x21;
pub const RTL2832_TUNER_TUA9001: c_uint = 0x24;
pub const RTL2832_TUNER_FC0012: c_uint = 0x26;
pub const RTL2832_TUNER_E4000: c_uint = 0x27;
pub const RTL2832_TUNER_FC0013: c_uint = 0x29;
pub const RTL2832_TUNER_R820T: c_uint = 0x2a;
pub const RTL2832_TUNER_R828D: c_uint = 0x2b;
pub const RTL2832_TUNER_SI2157: c_uint = 0x2c;
    pub tuner: u8,
    pub ): *mut *mut *mut dvb_frontend (get_dvb_frontend)(i2c_client,
    pub ): *mut *mut *mut i2c_adapter (get_i2c_adapter)(i2c_client,
    pub bool): *mut *mut *mut int (slave_ts_ctrl)(struct i2c_client ,,
    pub int): *mut *mut *mut int (pid_filter)(struct dvb_frontend , u8, u16,,
    pub int): *mut *mut *mut int (pid_filter_ctrl)(struct dvb_frontend ,,
// private: Register access for SDR module use only
    pub regmap: *mut regmap,
}
