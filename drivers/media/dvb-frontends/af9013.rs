//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/af9013.h
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
// Afatech AF9013 demodulator driver
//
// Copyright (C) 2007 Antti Palosaari <crope@iki.fi>
// Copyright (C) 2011 Antti Palosaari <crope@iki.fi>
//
// Thanks to Afatech who kindly provided information.
//

//
// I2C address: 0x1c, 0x1d
//
// struct af9013_platform_data - Platform data for the af9013 driver
// @clk: Clock frequency.
// @tuner: Used tuner model.
// @if_frequency: IF frequency.
// @ts_mode: TS mode.
// @ts_output_pin: TS output pin.
// @spec_inv: Input spectrum inverted.
// @api_version: Firmware API version.
// @gpio: GPIOs.
// @get_dvb_frontend: Get DVB frontend callback.
// @get_i2c_adapter: Get I2C adapter.
// @pid_filter_ctrl: Control PID filter.
// @pid_filter: Set PID to PID filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct af9013_platform_data {
//
// 20480000, 25000000, 28000000, 28800000
//
    pub clk: u32,

    pub tuner: u8,
    pub if_frequency: u32,
pub const AF9013_TS_MODE_USB: c_int = 0;
pub const AF9013_TS_MODE_PARALLEL: c_int = 1;
pub const AF9013_TS_MODE_SERIAL: c_int = 2;
    pub ts_mode: u8,
    pub ts_output_pin: u8,
    pub spec_inv: bool,
    pub api_version: [u8; 4],    pub gpio: [u8; 4],
    pub ): *mut *mut *mut dvb_frontend (get_dvb_frontend)(i2c_client,
    pub ): *mut *mut *mut i2c_adapter (get_i2c_adapter)(i2c_client,
    pub int): *mut *mut *mut int (pid_filter_ctrl)(struct dvb_frontend ,,
    pub int): *mut *mut *mut int (pid_filter)(struct dvb_frontend , u8, u16,,
}

//
// AF9013/5 GPIOs (mostly guessed)
// demod#1-gpio#0 - set demod#2 i2c-addr for dual devices
// demod#1-gpio#1 - xtal setting (?)
// demod#1-gpio#3 - tuner#1
// demod#2-gpio#0 - tuner#2
// demod#2-gpio#1 - xtal setting (?)
//
