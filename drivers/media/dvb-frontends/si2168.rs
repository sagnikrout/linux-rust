//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/si2168.h
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
// Silicon Labs Si2168 DVB-T/T2/C demodulator driver
//
// Copyright (C) 2014 Antti Palosaari <crope@iki.fi>
//

//
// struct si2168_config - configuration parameters for si2168
//
// @fe:
// frontend returned by driver
// @i2c_adapter:
// tuner I2C adapter returned by driver
// @ts_mode:
// Transport Stream mode. Can be:
// - %SI2168_TS_PARALLEL
// - %SI2168_TS_SERIAL
// - %SI2168_TS_TRISTATE
// - %SI2168_TS_CLK_MANUAL
// @ts_clock_inv:
// TS clock inverted
// @ts_clock_gapped:
// TS clock gapped
// @spectral_inversion:
// Inverted spectrum
//
// Note:
// The I2C address of this demod is 0x64.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2168_config {
    pub fe: *mut dvb_frontend,
    pub i2c_adapter: *mut i2c_adapter,
pub const SI2168_TS_PARALLEL: c_uint = 0x06;
pub const SI2168_TS_SERIAL: c_uint = 0x03;
pub const SI2168_TS_TRISTATE: c_uint = 0x00;
pub const SI2168_TS_CLK_MANUAL: c_uint = 0x20;
    pub ts_mode: u8,
// Flags
    pub ts_clock_inv:1: c_uint,
    pub ts_clock_gapped:1: c_uint,
    pub spectral_inversion:1: c_uint,
}
