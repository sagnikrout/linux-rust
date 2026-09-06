//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/a8293.h
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
// Allegro A8293 SEC driver
//
// Copyright (C) 2011 Antti Palosaari <crope@iki.fi>
//

//
// I2C address
// 0x08, 0x09, 0x0a, 0x0b
//
// struct a8293_platform_data - Platform data for the a8293 driver
// @dvb_frontend: DVB frontend.
// @volt_slew_nanos_per_mv: Slew rate when increasing LNB voltage,
// in nanoseconds per millivolt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a8293_platform_data {
    pub dvb_frontend: *mut dvb_frontend,
    pub volt_slew_nanos_per_mv: c_int,
}
