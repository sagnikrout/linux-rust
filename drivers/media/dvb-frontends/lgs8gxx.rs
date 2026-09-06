//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lgs8gxx.h
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
// Support for Legend Silicon GB20600 (a.k.a DMB-TH) demodulator
// LGS8913, LGS8GL5, LGS8G75
// experimental support LGS8G42, LGS8G52
//
// Copyright (C) 2007-2009 David T.L. Wong <davidtlwong@gmail.com>
// Copyright (C) 2008 Sirius International (Hong Kong) Limited
// Timothy Lee <timothy.lee@siriushk.com> (for initial work on LGS8GL5)
//

pub const LGS8GXX_PROD_LGS8913: c_int = 0;
pub const LGS8GXX_PROD_LGS8GL5: c_int = 1;
pub const LGS8GXX_PROD_LGS8G42: c_int = 3;
pub const LGS8GXX_PROD_LGS8G52: c_int = 4;
pub const LGS8GXX_PROD_LGS8G54: c_int = 5;
pub const LGS8GXX_PROD_LGS8G75: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgs8gxx_config {
// product type
    pub prod: u8,
// the demodulator's i2c address
    pub demod_address: u8,
// parallel or serial transport stream
    pub serial_ts: u8,
// transport stream polarity
    pub ts_clk_pol: u8,
// transport stream clock gated by ts_valid
    pub ts_clk_gated: u8,
// A/D Clock frequency
    pub /: *mut *mut u32 if_clk_freq; / in kHz,
// IF frequency
    pub /: *mut *mut u32 if_freq; / in kHz,
// Use External ADC
    pub ext_adc: u8,
// External ADC output two's complement
    pub adc_signed: u8,
// Sample IF data at falling edge of IF_CLK
    pub if_neg_edge: u8,
// IF use Negative center frequency
    pub if_neg_center: u8,
// 8G75 internal ADC input range selection
// 0: 0.8Vpp, 1: 1.0Vpp, 2: 1.6Vpp, 3: 2.0Vpp
    pub adc_vpp: u8,
// slave address and configuration of the tuner
    pub tuner_address: u8,
}

