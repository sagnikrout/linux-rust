//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/m5602/m5602_s5k4aa.h
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
// Driver for the s5k4aa sensor
//
// Copyright (C) 2008 Erik Andrén
// Copyright (C) 2007 Ilyes Gouta. Based on the m5603x Linux Driver Project.
// Copyright (C) 2005 m5603x Linux Driver Project <m5602@x3ng.com.br>
//
// Portions of code to USB interface and ALi driver software,
// Copyright (c) 2006 Willem Duinker
// v4l2 interface modeled after the V4L2 driver
// for SN9C10x PC Camera Controllers
//

//
pub const S5K4AA_PAGE_MAP: c_uint = 0xec;
pub const S5K4AA_PAGE_MAP_0: c_uint = 0x00;
pub const S5K4AA_PAGE_MAP_1: c_uint = 0x01;
pub const S5K4AA_PAGE_MAP_2: c_uint = 0x02;
// Sensor register definitions for page 0x02
pub const S5K4AA_READ_MODE: c_uint = 0x03;
pub const S5K4AA_ROWSTART_HI: c_uint = 0x04;
pub const S5K4AA_ROWSTART_LO: c_uint = 0x05;
pub const S5K4AA_COLSTART_HI: c_uint = 0x06;
pub const S5K4AA_COLSTART_LO: c_uint = 0x07;
pub const S5K4AA_WINDOW_HEIGHT_HI: c_uint = 0x08;
pub const S5K4AA_WINDOW_HEIGHT_LO: c_uint = 0x09;
pub const S5K4AA_WINDOW_WIDTH_HI: c_uint = 0x0a;
pub const S5K4AA_WINDOW_WIDTH_LO: c_uint = 0x0b;
pub const S5K4AA_GLOBAL_GAIN__: c_uint = 0x0f;
// sync lost, if too low, reduces frame rate if too high
pub const S5K4AA_H_BLANK_HI__: c_uint = 0x1d;
pub const S5K4AA_H_BLANK_LO__: c_uint = 0x1e;
pub const S5K4AA_EXPOSURE_HI: c_uint = 0x17;
pub const S5K4AA_EXPOSURE_LO: c_uint = 0x18;
pub const S5K4AA_BRIGHTNESS: c_uint = 0x1f /* (digital?) gain : 5 bits */;
pub const S5K4AA_GAIN: c_uint = 0x20 /* (analogue?) gain : 7 bits */;
pub const S5K4AA_NOISE_SUPP: c_uint = 0x37;
pub const S5K4AA_RM_ROW_SKIP_4X: c_uint = 0x08;
pub const S5K4AA_RM_ROW_SKIP_2X: c_uint = 0x04;
pub const S5K4AA_RM_COL_SKIP_4X: c_uint = 0x02;
pub const S5K4AA_RM_COL_SKIP_2X: c_uint = 0x01;
pub const S5K4AA_RM_H_FLIP: c_uint = 0x40;
pub const S5K4AA_RM_V_FLIP: c_uint = 0x80;
pub const S5K4AA_DEFAULT_GAIN: c_uint = 0x5f;
pub const S5K4AA_DEFAULT_BRIGHTNESS: c_uint = 0x10;
//
// Kernel module parameters
extern "C" {
    pub fn s5k4aa_probe(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k4aa_init(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k4aa_init_controls(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k4aa_start(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k4aa_disconnect(sd: *mut sd);
}
