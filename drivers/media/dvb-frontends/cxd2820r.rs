//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2820r.h
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
// Sony CXD2820R demodulator driver
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

pub const CXD2820R_TS_SERIAL: c_uint = 0x08;
pub const CXD2820R_TS_SERIAL_MSB: c_uint = 0x28;
pub const CXD2820R_TS_PARALLEL: c_uint = 0x30;
pub const CXD2820R_TS_PARALLEL_MSB: c_uint = 0x70;
//
// I2C address: 0x6c, 0x6d
//
// struct cxd2820r_platform_data - Platform data for the cxd2820r driver
// @ts_mode: TS mode.
// @ts_clk_inv: TS clock inverted.
// @if_agc_polarity: IF AGC polarity.
// @spec_inv: Input spectrum inverted.
// @gpio_chip_base: GPIO.
// @get_dvb_frontend: Get DVB frontend.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2820r_platform_data {
    pub ts_mode: u8,
    pub ts_clk_inv: bool,
    pub if_agc_polarity: bool,
    pub spec_inv: bool,
    pub gpio_chip_base: *mut c_int,
    pub ): *mut *mut *mut dvb_frontend (get_dvb_frontend)(i2c_client,
// private: For legacy media attach wrapper. Do not set value.
    pub attach_in_use: bool,
}

//
// struct cxd2820r_config - configuration for cxd2020r demod
//
// @i2c_address: Demodulator I2C address. Driver determines DVB-C slave I2C
// address automatically from master address.
// Default: none, must set. Values: 0x6c, 0x6d.
// @ts_mode:	TS output mode. Default: none, must set. Values: FIXME?
// @ts_clock_inv: TS clock inverted. Default: 0. Values: 0, 1.
// @if_agc_polarity: Default: 0. Values: 0, 1
// @spec_inv:	Spectrum inversion. Default: 0. Values: 0, 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2820r_config {
// Demodulator I2C address.
// Driver determines DVB-C slave I2C address automatically from master
// address.
// Default: none, must set
// Values: 0x6c, 0x6d
//
    pub i2c_address: u8,
// TS output mode.
// Default: none, must set.
// Values:
//
    pub ts_mode: u8,
// TS clock inverted.
// Default: 0
// Values: 0, 1
//
    pub ts_clock_inv: bool,
// IF AGC polarity.
// Default: 0
// Values: 0, 1
//
    pub if_agc_polarity: bool,
// Spectrum inversion.
// Default: 0
// Values: 0, 1
//
    pub spec_inv: bool,
}

//
// cxd2820r_attach - Attach a cxd2820r demod
//
// @config: pointer to &struct cxd2820r_config with demod configuration.
// @i2c: i2c adapter to use.
// @gpio_chip_base: if zero, disables GPIO setting. Otherwise, if
// CONFIG_GPIOLIB is set dynamically allocate
// gpio base; if is not set, use its value to
// setup the GPIO pins.
//
// return: FE pointer on success, NULL on failure.
//

