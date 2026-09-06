//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/itd1000_priv.h
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
// Driver for the Integrant ITD1000 "Zero-IF Tuner IC for Direct Broadcast Satellite"
//
// Copyright (c) 2007 Patrick Boettcher <pb@linuxtv.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct itd1000_state {
    pub cfg: *mut itd1000_config,
    pub i2c: *mut i2c_adapter,
    pub /: *mut *mut u32 frequency; / contains the value resulting from the LO-setting,
// ugly workaround for flexcop's incapable i2c-controller
// FIXME, if possible
//
    pub shadow: [u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum itd1000_register {
    VCO_CHP1 = 0x65,
    VCO_CHP2,
    PLLCON1,
    PLLNH,
    PLLNL,
    PLLFH,
    PLLFM,
    PLLFL,
    RESERVED_0X6D,
    PLLLOCK,
    VCO_CHP2_I2C,
    VCO_CHP1_I2C,
    BW,
    RESERVED_0X73 = 0x73,
    RESERVED_0X74,
    RESERVED_0X75,
    GVBB,
    GVRF,
    GVBB_I2C,
    EXTGVBBRF,
    DIVAGCCK,
    BBTR,
    RFTR,
    BBGVMIN,
    RESERVED_0X7E,
    RESERVED_0X85 = 0x85,
    RESERVED_0X86,
    CON1,
    RESERVED_0X88,
    RESERVED_0X89,
    RFST0,
    RFST1,
    RFST2,
    RFST3,
    RFST4,
    RFST5,
    RFST6,
    RFST7,
    RFST8,
    RFST9,
    RESERVED_0X94,
    RESERVED_0X95,
    RESERVED_0X96,
    RESERVED_0X97,
    RESERVED_0X98,
    RESERVED_0X99,
    RESERVED_0X9A,
    RESERVED_0X9B,
}
