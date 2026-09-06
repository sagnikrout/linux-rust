//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/rtl2830.h
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
// Realtek RTL2830 DVB-T demodulator driver
//
// Copyright (C) 2011 Antti Palosaari <crope@iki.fi>
//

//
// struct rtl2830_platform_data - Platform data for the rtl2830 driver
// @clk: Clock frequency (4000000, 16000000, 25000000, 28800000).
// @spec_inv: Spectrum inversion.
// @vtop: AGC take-over point.
// @krf: AGC ratio.
// @agc_targ_val: AGC.
// @get_dvb_frontend: Get DVB frontend.
// @get_i2c_adapter: Get I2C adapter.
// @pid_filter: Set PID to PID filter.
// @pid_filter_ctrl: Control PID filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl2830_platform_data {
    pub clk: u32,
    pub spec_inv: bool,
    pub vtop: u8,
    pub krf: u8,
    pub agc_targ_val: u8,
    pub ): *mut *mut *mut dvb_frontend (get_dvb_frontend)(i2c_client,
    pub ): *mut *mut *mut i2c_adapter (get_i2c_adapter)(i2c_client,
    pub int): *mut *mut *mut int (pid_filter)(struct dvb_frontend , u8, u16,,
    pub int): *mut *mut *mut int (pid_filter_ctrl)(struct dvb_frontend ,,
}
