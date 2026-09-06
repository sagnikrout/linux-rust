//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/drxd.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// drxd.h: DRXD DVB-T demodulator driver
//
// Copyright (C) 2005-2007 Micronas
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxd_config {
    pub index: u8,
    pub pll_address: u8,
    pub pll_type: u8,
pub const DRXD_PLL_NONE: c_int = 0;
pub const DRXD_PLL_DTT7520X: c_int = 1;
pub const DRXD_PLL_MT3X0823: c_int = 2;
    pub clock: u32,
    pub insert_rs_byte: u8,
    pub demod_address: u8,
    pub demoda_address: u8,
    pub demod_revision: u8,
// If the tuner is not behind an i2c gate, be sure to flip this bit
    pub disable_i2c_gate_ctrl: u8,
    pub IF: u32,
    pub flag): *mut *mut *mut s16(osc_deviation) (void priv, s16 dev, int,
}

