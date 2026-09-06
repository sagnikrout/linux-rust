//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/fc2580.h
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
// FCI FC2580 silicon tuner driver
//
// Copyright (C) 2012 Antti Palosaari <crope@iki.fi>
//

//
// I2C address
// 0x56, ...
//
// struct fc2580_platform_data - Platform data for the fc2580 driver
// @clk: Clock frequency (0 = internal clock).
// @dvb_frontend: DVB frontend.
// @get_v4l2_subdev: Get V4L2 subdev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc2580_platform_data {
    pub clk: u32,
    pub dvb_frontend: *mut dvb_frontend,
    pub ): *mut *mut *mut v4l2_subdev (get_v4l2_subdev)(i2c_client,
}
