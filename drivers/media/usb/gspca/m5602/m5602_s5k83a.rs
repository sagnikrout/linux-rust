//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/m5602/m5602_s5k83a.h
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
// Driver for the s5k83a sensor
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

pub const S5K83A_FLIP: c_uint = 0x01;
pub const S5K83A_HFLIP_TUNE: c_uint = 0x03;
pub const S5K83A_VFLIP_TUNE: c_uint = 0x05;
pub const S5K83A_BRIGHTNESS: c_uint = 0x0a;
pub const S5K83A_EXPOSURE: c_uint = 0x18;
pub const S5K83A_GAIN: c_uint = 0x1b;
pub const S5K83A_PAGE_MAP: c_uint = 0xec;
pub const S5K83A_DEFAULT_GAIN: c_uint = 0x71;
pub const S5K83A_DEFAULT_BRIGHTNESS: c_uint = 0x7e;
pub const S5K83A_DEFAULT_EXPOSURE: c_uint = 0x00;
pub const S5K83A_MAXIMUM_EXPOSURE: c_uint = 0x3c;
pub const S5K83A_FLIP_MASK: c_uint = 0x10;
pub const S5K83A_GPIO_LED_MASK: c_uint = 0x10;
pub const S5K83A_GPIO_ROTATION_MASK: c_uint = 0x40;
//
// Kernel module parameters
extern "C" {
    pub fn s5k83a_probe(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k83a_init(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k83a_init_controls(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k83a_start(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k83a_stop(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn s5k83a_disconnect(sd: *mut sd);
}
