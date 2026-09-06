//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lgdt3305.h
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
// Support for LG Electronics LGDT3304 and LGDT3305 - VSB/QAM
//
// Copyright (C) 2008, 2009, 2010 Michael Krufky <mkrufky@linuxtv.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgdt3305_mpeg_mode {
    LGDT3305_MPEG_PARALLEL = 0,
    LGDT3305_MPEG_SERIAL = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgdt3305_tp_clock_edge {
    LGDT3305_TPCLK_RISING_EDGE = 0,
    LGDT3305_TPCLK_FALLING_EDGE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgdt3305_tp_clock_mode {
    LGDT3305_TPCLK_GATED = 0,
    LGDT3305_TPCLK_FIXED = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgdt3305_tp_valid_polarity {
    LGDT3305_TP_VALID_LOW = 0,
    LGDT3305_TP_VALID_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgdt_demod_chip_type {
    LGDT3305 = 0,
    LGDT3304 = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgdt3305_config {
    pub i2c_addr: u8,
// user defined IF frequency in KHz
    pub qam_if_khz: u16,
    pub vsb_if_khz: u16,
// AGC Power reference - defaults are used if left unset
    pub /: *mut *mut u16 usref_8vsb; / default: 0x32c4,
    pub /: *mut *mut u16 usref_qam64; / default: 0x5400,
    pub /: *mut *mut u16 usref_qam256; / default: 0x2a80,
// disable i2c repeater - 0:repeater enabled 1:repeater disabled
    pub deny_i2c_rptr:1: c_uint,
// spectral inversion - 0:disabled 1:enabled
    pub spectral_inversion:1: c_uint,
// use RF AGC loop - 0:disabled 1:enabled
    pub rf_agc_loop:1: c_uint,
    pub mpeg_mode: lgdt3305_mpeg_mode,
    pub tpclk_edge: lgdt3305_tp_clock_edge,
    pub tpclk_mode: lgdt3305_tp_clock_mode,
    pub tpvalid_polarity: lgdt3305_tp_valid_polarity,
    pub demod_chip: lgdt_demod_chip_type,
}

