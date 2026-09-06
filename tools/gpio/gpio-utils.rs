//! Automatically rewritten from C Header to Rust Module
//! Source: tools/gpio/gpio-utils.h
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
// GPIO tools - utility helpers library for the GPIO tools
//
// Copyright (C) 2015 Linus Walleij
//
// Portions copied from iio_utils and lssio:
// Copyright (c) 2010 Manuel Stahl <manuel.stahl@iis.fraunhofer.de>
// Copyright (c) 2008 Jonathan Cameron
//

extern "C" {
    pub fn gpiotools_set_values(fd: c_int, values: *mut gpio_v2_line_values) -> c_int;
}
extern "C" {
    pub fn gpiotools_get_values(fd: c_int, values: *mut gpio_v2_line_values) -> c_int;
}
extern "C" {
    pub fn gpiotools_release_line(fd: c_int) -> c_int;
}
extern "C" {
    pub fn gpiotools_get(device_name: *const c_char, line: c_uint) -> c_int;
}
// helper functions for gpio_v2_line_values bits
// b |= _BITULL(n);
// b ^= _BITULL(n);
// b &= ~_BITULL(n);
