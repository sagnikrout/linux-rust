//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tda18212.h
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
// NXP TDA18212HN silicon tuner driver
//
// Copyright (C) 2011 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18212_config {
    pub if_dvbt_6: u16,
    pub if_dvbt_7: u16,
    pub if_dvbt_8: u16,
    pub if_dvbt2_5: u16,
    pub if_dvbt2_6: u16,
    pub if_dvbt2_7: u16,
    pub if_dvbt2_8: u16,
    pub if_dvbc: u16,
    pub if_atsc_vsb: u16,
    pub if_atsc_qam: u16,
//
// pointer to DVB frontend
//
    pub fe: *mut dvb_frontend,
}
