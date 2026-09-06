//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/pixel_format.h
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


// SPDX-License-Identifier: GPL-2.0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pixel_format {
    pub bits_per_pixel: c_uchar,
    pub indexed: bool,
    pub offset: c_uchar,
    pub length: c_uchar,
    pub blue: } alpha, red, green,,
}

//
// pixel_format_cmp - Compares two pixel-format descriptions
//
// @lhs: a pixel-format description
// @rhs: a pixel-format description
//
// Compares two pixel-format descriptions for their order. The semantics
// are equivalent to memcmp().
//
// Returns:
// 0 if both arguments describe the same pixel format, less-than-zero if lhs < rhs,
// or greater-than-zero if lhs > rhs.
//
// pixel_format_equal - Compares two pixel-format descriptions for equality
//
// @lhs: a pixel-format description
// @rhs: a pixel-format description
//
// Returns:
// True if both arguments describe the same pixel format, or false otherwise.
//
