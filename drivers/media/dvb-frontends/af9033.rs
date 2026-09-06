//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/af9033.h
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
// Afatech AF9033 demodulator driver
//
// Copyright (C) 2009 Antti Palosaari <crope@iki.fi>
// Copyright (C) 2012 Antti Palosaari <crope@iki.fi>
//
// I2C address: 0x1c, 0x1d, 0x1e, 0x1f
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct af9033_config {
//
// clock Hz
// 12000000, 22000000, 24000000, 34000000, 32000000, 28000000, 26000000,
// 30000000, 36000000, 20480000, 16384000
//
    pub clock: u32,
//
// ADC multiplier
//
pub const AF9033_ADC_MULTIPLIER_1X: c_int = 0;
pub const AF9033_ADC_MULTIPLIER_2X: c_int = 1;
    pub adc_multiplier: u8,
//
// tuner
//
pub const AF9033_TUNER_TUA9001: c_uint = 0x27 /* Infineon TUA 9001 */;
pub const AF9033_TUNER_FC0011: c_uint = 0x28 /* Fitipower FC0011 */;
pub const AF9033_TUNER_FC0012: c_uint = 0x2e /* Fitipower FC0012 */;
pub const AF9033_TUNER_MXL5007T: c_uint = 0xa0 /* MaxLinear MxL5007T */;
pub const AF9033_TUNER_TDA18218: c_uint = 0xa1 /* NXP TDA 18218HN */;
pub const AF9033_TUNER_FC2580: c_uint = 0x32 /* FCI FC2580 */;
// 50-5f Omega
pub const AF9033_TUNER_IT9135_38: c_uint = 0x38 /* Omega */;
pub const AF9033_TUNER_IT9135_51: c_uint = 0x51 /* Omega LNA config 1 */;
pub const AF9033_TUNER_IT9135_52: c_uint = 0x52 /* Omega LNA config 2 */;
// 60-6f Omega v2
pub const AF9033_TUNER_IT9135_60: c_uint = 0x60 /* Omega v2 */;
pub const AF9033_TUNER_IT9135_61: c_uint = 0x61 /* Omega v2 LNA config 1 */;
pub const AF9033_TUNER_IT9135_62: c_uint = 0x62 /* Omega v2 LNA config 2 */;
    pub tuner: u8,
//
// TS settings
//
pub const AF9033_TS_MODE_USB: c_int = 0;
pub const AF9033_TS_MODE_PARALLEL: c_int = 1;
pub const AF9033_TS_MODE_SERIAL: c_int = 2;
    pub ts_mode:2: u8,
//
// input spectrum inversion
//
    pub spec_inv: bool,
//
    pub dyn0_clk: bool,
//
// PID filter ops
//
    pub ops: *mut af9033_ops,
//
// frontend
// returned by that driver
//
    pub fe: *mut dvb_frontend,
//
// regmap for IT913x integrated tuner driver
// returned by that driver
//
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct af9033_ops {
    pub onoff): *mut *mut *mut int (pid_filter_ctrl)(struct dvb_frontend fe, int,
    pub onoff): c_int,
}
