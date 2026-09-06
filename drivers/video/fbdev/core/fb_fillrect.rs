//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/core/fb_fillrect.h
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
// Generic bit area filler and twister engine for packed pixel framebuffers
//
// Rewritten by:
// Copyright (C)  2025 Zsolt Kajtar (soci@c64.rulez.org)
//
// Based on earlier work of:
// Copyright (C)  2000 James Simmons (jsimmons@linux-fbdev.org)
// Michal Januszewski <spock@gentoo.org>
// Anton Vorontsov <avorontsov@ru.mvista.com>
// Pavel Pisa <pisa@cmp.felk.cvut.cz>
// Antonino A. Daplas <adaplas@gmail.com>
// Geert Uytterhoeven
// and others
//
// NOTES:
//
// Handles native and foreign byte order on both endians, standard and
// reverse pixel order in a byte (<8 BPP), word length of 32/64 bits,
// bits per pixel from 1 to the word length. Handles line lengths at byte
// granularity while maintaining aligned accesses.
//
// Optimized path for power of two bits per pixel modes.
//

// inverts bits at a given offset
// state for pattern generator and whether swapping is necessary
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_pattern {
    pub pixels: c_ulong,
    pub right: int left,,
    pub reverse: fb_reverse,
}

// used to get the pattern in native order
// used to get the pattern in reverse order
extern "C" {
    pub fn swab_long(_arg: pattern->pixels) -> return;
}
// next static pattern
// nothing to do
// next rotating pattern

// create the filling pattern from a given color

// overwrite bits according to a pattern in a line
// inverts bits according to a pattern in a line
// pattern doesn't change. 1, 2, 4, 8, 16, 32, 64 bpp
// rotate pattern to the correct position
extern "C" {
    pub fn fb_right(_arg: pattern, fb_left(pattern: shift) |, shift: bpp -) -> return;
}
// rotating pattern, for example 24 bpp
