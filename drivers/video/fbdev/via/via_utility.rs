//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/via_utility.h
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
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//
// These functions are used to get information about device's state
extern "C" {
    pub fn viafb_get_device_support_state(support_state: *mut u32);
}
extern "C" {
    pub fn viafb_get_device_connect_state(connect_state: *mut u32);
}
extern "C" {
    pub fn viafb_lcd_get_support_expand_state(xres: u32, yres: u32) -> bool;
}
// These function are used to access gamma table
extern "C" {
    pub fn viafb_set_gamma_table(bpp: c_int, gamma_table: *mut c_uint);
}
extern "C" {
    pub fn viafb_get_gamma_table(gamma_table: *mut c_uint);
}
extern "C" {
    pub fn viafb_get_gamma_support_state(bpp: c_int, support_state: *mut c_uint);
}
