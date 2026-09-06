//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tda18250.h
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
// NXP TDA18250BHN silicon tuner driver
//
// Copyright (C) 2017 Olli Salonen <olli.salonen@iki.fi>
//

pub const TDA18250_XTAL_FREQ_16MHZ: c_int = 0;
pub const TDA18250_XTAL_FREQ_24MHZ: c_int = 1;
pub const TDA18250_XTAL_FREQ_25MHZ: c_int = 2;
pub const TDA18250_XTAL_FREQ_27MHZ: c_int = 3;
pub const TDA18250_XTAL_FREQ_30MHZ: c_int = 4;
pub const TDA18250_XTAL_FREQ_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18250_config {
    pub if_dvbt_6: u16,
    pub if_dvbt_7: u16,
    pub if_dvbt_8: u16,
    pub if_dvbc_6: u16,
    pub if_dvbc_8: u16,
    pub if_atsc: u16,
    pub xtal_freq: u8,
    pub loopthrough: bool,
//
// frontend
//
    pub fe: *mut dvb_frontend,

    pub mdev: *mut media_device,

}
