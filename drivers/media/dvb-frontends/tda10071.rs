//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/tda10071.h
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
// NXP TDA10071 + Conexant CX24118A DVB-S/S2 demodulator + tuner driver
//
// Copyright (C) 2011 Antti Palosaari <crope@iki.fi>
//

//
// I2C address
// 0x05, 0x55,
//
// struct tda10071_platform_data - Platform data for the tda10071 driver
// @clk: Clock frequency.
// @i2c_wr_max: Max bytes I2C adapter can write at once.
// @ts_mode: TS mode.
// @spec_inv: Input spectrum inversion.
// @pll_multiplier: PLL multiplier.
// @tuner_i2c_addr: CX24118A tuner I2C address (0x14, 0x54, ...).
// @get_dvb_frontend: Get DVB frontend.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda10071_platform_data {
    pub clk: u32,
    pub i2c_wr_max: u16,
pub const TDA10071_TS_SERIAL: c_int = 0;
pub const TDA10071_TS_PARALLEL: c_int = 1;
    pub ts_mode: u8,
    pub spec_inv: bool,
    pub pll_multiplier: u8,
    pub tuner_i2c_addr: u8,
    pub ): *mut *mut *mut dvb_frontend (get_dvb_frontend)(i2c_client,
}
