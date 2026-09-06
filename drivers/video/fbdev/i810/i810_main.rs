//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/i810/i810_main.h
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


// -*- linux-c -*-
// linux/drivers/video/i810fb_main.h -- Intel 810 frame buffer device
// main header file
//
// Copyright (C) 2001 Antonino Daplas<adaplas@pol.net>
// All Rights Reserved
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// Video Timings
extern "C" {
    pub fn round_off_xres(xres: *mut u32);
}
extern "C" {
    pub fn round_off_yres(xres: *mut u32, yres: *mut u32);
}
extern "C" {
    pub fn i810fb_fill_var_timings(var: *mut fb_var_screeninfo);
}
// Accelerated Functions
extern "C" {
    pub fn i810fb_imageblit(p: *mut fb_info, image: *const fb_image);
}
extern "C" {
    pub fn i810fb_sync(p: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn i810fb_init_ringbuffer(info: *mut fb_info);
}
extern "C" {
    pub fn i810fb_load_front(offset: u32, info: *mut fb_info);
}

// I2C
extern "C" {
    pub fn i810_create_i2c_busses(par: *mut i810fb_par);
}
extern "C" {
    pub fn i810_delete_i2c_busses(par: *mut i810fb_par);
}

// Conditionals

extern "C" {
    pub fn volatile(_arg: "wbinvd":::"memory") -> asm;
}

