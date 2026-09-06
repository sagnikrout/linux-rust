//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/drm_format_internal.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT

//
// Each pixel-format conversion helper takes a raw pixel in a
// specific input format and returns a raw pixel in a specific
// output format. All pixels are in little-endian byte order.
//
// Function names are
//
// drm_pixel_<input>_to_<output>_<algorithm>()
//
// where <input> and <output> refer to pixel formats. The
// <algorithm> is optional and hints to the method used for the
// conversion. Helpers with no algorithm given apply pixel-bit
// shifting.
//
// The argument type is u32. We expect this to be wide enough to
// hold all conversion input from 32-bit RGB to any output format.
// The Linux kernel should avoid format conversion for anything
// but XRGB8888 input data. Converting from other format can still
// be acceptable in some cases.
//
// The return type is u32. It is wide enough to hold all conversion
// output from XRGB8888. For output formats wider than 32 bit, a
// return type of u64 would be acceptable.
//
// Conversions from XRGB8888
//
// ITU-R BT.601: Y = 0.299 R + 0.587 G + 0.114 B
extern "C" {
    pub fn swab16(_arg: drm_pixel_xrgb8888_to_rgb565(pix)) -> return;
}
//
// Conversion from ARGB8888
//
