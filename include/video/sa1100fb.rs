//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/sa1100fb.h
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


//
// StrongARM 1100 LCD Controller Frame Buffer Device
//
// Copyright (C) 1999 Eric A. Thomas
// Based on acornfb.c Copyright (C) Russell King.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

pub const RGB_4: c_int = 0;
pub const RGB_8: c_int = 1;
pub const RGB_16: c_int = 2;
pub const NR_RGB: c_int = 3;
// These are the bitfields for each display depth that we support.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa1100fb_rgb {
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
}

// This structure describes the machine which we are running on.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa1100fb_mach_info {
    pub pixclock: u_long,
    pub xres: u_short,
    pub yres: u_short,
    pub bpp: u_char,
    pub hsync_len: u_char,
    pub left_margin: u_char,
    pub right_margin: u_char,
    pub vsync_len: u_char,
    pub upper_margin: u_char,
    pub lower_margin: u_char,
    pub sync: u_char,
    pub lccr0: u_int,
    pub lccr3: u_int,
// Overrides for the default RGB maps
    pub rgb: [*const sa1100fb_rgb; NR_RGB],
    pub (*backlight_power)(int): *mut c_void,
    pub (*lcd_power)(int): *mut c_void,
    pub (*set_visual)(u32): *mut c_void,
}
