//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/et8ek8/et8ek8_reg.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// et8ek8_reg.h
//
// Copyright (C) 2008 Nokia Corporation
//
// Contact: Sakari Ailus <sakari.ailus@iki.fi>
// Tuukka Toivonen <tuukkat76@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct et8ek8_mode {
// Physical sensor resolution and current image window
    pub sensor_width: u16,
    pub sensor_height: u16,
    pub sensor_window_origin_x: u16,
    pub sensor_window_origin_y: u16,
    pub sensor_window_width: u16,
    pub sensor_window_height: u16,
// Image data coming from sensor (after scaling)
    pub width: u16,
    pub height: u16,
    pub window_origin_x: u16,
    pub window_origin_y: u16,
    pub window_width: u16,
    pub window_height: u16,
    pub /: *mut *mut u32 pixel_clock; / in Hz,
    pub timeperframe: v4l2_fract,
    pub /: *mut *mut u32 max_exp; / Maximum exposure value,
    pub /: *mut *mut u32 bus_format; / MEDIA_BUS_FMT_,
    pub /: *mut *mut u32 sensitivity; / 16.16 fixed point,
}

pub const ET8EK8_REG_8BIT: c_int = 1;
pub const ET8EK8_REG_16BIT: c_int = 2;
pub const ET8EK8_REG_DELAY: c_int = 100;
pub const ET8EK8_REG_TERM: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct et8ek8_reg {
    pub type: u16,
    pub /: *mut *mut u16 reg; / 16-bit offset,
    pub /: *mut *mut u32 val; / 8/16/32-bit value,
}

// Possible struct smia_reglist types.
pub const ET8EK8_REGLIST_STANDBY: c_int = 0;
pub const ET8EK8_REGLIST_POWERON: c_int = 1;
pub const ET8EK8_REGLIST_RESUME: c_int = 2;
pub const ET8EK8_REGLIST_STREAMON: c_int = 3;
pub const ET8EK8_REGLIST_STREAMOFF: c_int = 4;
pub const ET8EK8_REGLIST_DISABLED: c_int = 5;
pub const ET8EK8_REGLIST_MODE: c_int = 10;
pub const ET8EK8_REGLIST_LSC_ENABLE: c_int = 100;
pub const ET8EK8_REGLIST_LSC_DISABLE: c_int = 101;
pub const ET8EK8_REGLIST_ANR_ENABLE: c_int = 102;
pub const ET8EK8_REGLIST_ANR_DISABLE: c_int = 103;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct et8ek8_reglist {
    pub type: u32,
    pub mode: et8ek8_mode,
    pub regs: [et8ek8_reg; ],
}

pub const ET8EK8_MAX_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct et8ek8_meta_reglist {
    pub version: [c_char; ET8EK8_MAX_LEN],
    pub ptr: *mut et8ek8_reglist,
    pub reglist: [}; ],
}
