//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lgdt3306a.h
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
// Support for LGDT3306A - 8VSB/QAM-B
//
// Copyright (C) 2013,2014 Fred Richter <frichter@hauppauge.com>
// based on lgdt3305.[ch] by Michael Krufky
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgdt3306a_mpeg_mode {
    LGDT3306A_MPEG_PARALLEL = 0,
    LGDT3306A_MPEG_SERIAL = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgdt3306a_tp_clock_edge {
    LGDT3306A_TPCLK_RISING_EDGE = 0,
    LGDT3306A_TPCLK_FALLING_EDGE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgdt3306a_tp_valid_polarity {
    LGDT3306A_TP_VALID_LOW = 0,
    LGDT3306A_TP_VALID_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgdt3306a_config {
    pub i2c_addr: u8,
// user defined IF frequency in KHz
    pub qam_if_khz: u16,
    pub vsb_if_khz: u16,
// disable i2c repeater - 0:repeater enabled 1:repeater disabled
    pub deny_i2c_rptr:1: c_uint,
// spectral inversion - 0:disabled 1:enabled
    pub spectral_inversion:1: c_uint,
    pub mpeg_mode: lgdt3306a_mpeg_mode,
    pub tpclk_edge: lgdt3306a_tp_clock_edge,
    pub tpvalid_polarity: lgdt3306a_tp_valid_polarity,
// demod clock freq in MHz; 24 or 25 supported
    pub xtalMHz: c_int,
// returned by driver if using i2c bus multiplexing
    pub fe: *mut dvb_frontend,
    pub i2c_adapter: *mut i2c_adapter,
}

