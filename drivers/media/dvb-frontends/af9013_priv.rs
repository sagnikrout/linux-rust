//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/af9013_priv.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct af9013_reg_mask_val {
    pub reg: u16,
    pub mask: u8,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct af9013_coeff {
    pub clock: u32,
    pub bandwidth_hz: u32,
    pub val: [u8; 24],
}

// pre-calculated coeff lookup table
// 28.800 MHz
// 20.480 MHz
// 28.000 MHz
// 25.000 MHz
//
// Afatech AF9013 demod init
//
// Panasonic ENV77H11D5 tuner init
// AF9013_TUNER_ENV77H11D5    0x81
//
// Microtune MT2060 tuner init
// AF9013_TUNER_MT2060        0x82
//
// Microtune MT2060 tuner init
// AF9013_TUNER_MT2060_2      0x93
//
// MaxLinear MXL5003 tuner init
// AF9013_TUNER_MXL5003D      0x03
//
// MaxLinear MXL5005S & MXL5007T tuner init
// AF9013_TUNER_MXL5005D      0x0d
// AF9013_TUNER_MXL5005R      0x1e
// AF9013_TUNER_MXL5007T      0xb1
//
// Quantek QT1010 tuner init
// AF9013_TUNER_QT1010        0x86
// AF9013_TUNER_QT1010A       0xa2
//
// Freescale MC44S803 tuner init
// AF9013_TUNER_MC44S803      0x85
//
// Unknown, probably for tin can tuner, tuner init
// AF9013_TUNER_UNKNOWN       0x8c
//
// NXP TDA18271 & TDA18218 tuner init
// AF9013_TUNER_TDA18271      0x9c
// AF9013_TUNER_TDA18218      0xb3
//
