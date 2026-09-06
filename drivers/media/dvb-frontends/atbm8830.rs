//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/atbm8830.h
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
// Support for AltoBeam GB20600 (a.k.a DMB-TH) demodulator
// ATBM8830, ATBM8831
//
// Copyright (C) 2009 David T.L. Wong <davidtlwong@gmail.com>
//

pub const ATBM8830_PROD_8830: c_int = 0;
pub const ATBM8830_PROD_8831: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atbm8830_config {
// product type
    pub prod: u8,
// the demodulator's i2c address
    pub demod_address: u8,
// parallel or serial transport stream
    pub serial_ts: u8,
// transport stream clock output only when receiving valid stream
    pub ts_clk_gated: u8,
// Decoder sample TS data at rising edge of clock
    pub ts_sampling_edge: u8,
// Oscillator clock frequency
    pub /: *mut *mut u32 osc_clk_freq; / in kHz,
// IF frequency
    pub /: *mut *mut u32 if_freq; / in kHz,
// Swap I/Q for zero IF
    pub zif_swap_iq: u8,
// Tuner AGC settings
    pub agc_min: u8,
    pub agc_max: u8,
    pub agc_hold_loop: u8,
}

