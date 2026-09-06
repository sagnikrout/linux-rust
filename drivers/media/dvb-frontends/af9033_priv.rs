//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/af9033_priv.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_val {
    pub reg: u32,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_val_mask {
    pub reg: u32,
    pub val: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coeff {
    pub clock: u32,
    pub bandwidth_hz: u32,
    pub val: [u8; 36],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_adc {
    pub clock: u32,
    pub adc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct val_snr {
    pub val: u32,
    pub snr: u8,
}

// Xtal clock vs. ADC clock lookup table
// Pre-calculated coeff lookup table
// 12.000 MHz
//
// Afatech AF9033 demod init
//
// Infineon TUA 9001 tuner init
// AF9033_TUNER_TUA9001    = 0x27
//
// Fitipower FC0011 tuner init
// AF9033_TUNER_FC0011    = 0x28
//
// Fitipower FC0012 tuner init
// AF9033_TUNER_FC0012    = 0x2e
//
// MaxLinear MxL5007T tuner init
// AF9033_TUNER_MXL5007T    = 0xa0
//
// NXP TDA18218HN tuner init
// AF9033_TUNER_TDA18218    = 0xa1
//
// FCI FC2580 tuner init
// AF9033_TUNER_FC2580      = 0x32
//
// IT9133 AX demod init
//
// ITE Tech IT9133 AX Omega tuner init
// AF9033_TUNER_IT9135_38   = 0x38
//
// ITE Tech IT9133 AX Omega LNA config 1 tuner init
// AF9033_TUNER_IT9135_51   = 0x51
//
// ITE Tech IT9133 AX Omega LNA config 2 tuner init
// AF9033_TUNER_IT9135_52   = 0x52
//
// ITE Tech IT9133 BX demod init
//
// ITE Tech IT9133 BX Omega tuner init
// AF9033_TUNER_IT9135_60   = 0x60
//
// ITE Tech IT9133 BX Omega LNA config 1 tuner init
// AF9033_TUNER_IT9135_61   = 0x61
//
// ITE Tech IT9133 BX Omega LNA config 2 tuner init
// AF9033_TUNER_IT9135_62   = 0x62
//
// NorDig power reference table
