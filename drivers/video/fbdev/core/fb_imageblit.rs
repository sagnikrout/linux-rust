//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/core/fb_imageblit.h
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
// Generic bitmap / 8 bpp image bitstreamer for packed pixel framebuffers
//
// Rewritten by:
// Copyright (C)  2025 Zsolt Kajtar (soci@c64.rulez.org)
//
// Based on previous work of:
// Copyright (C)  June 1999 James Simmons
// Anton Vorontsov <avorontsov@ru.mvista.com>
// Pavel Pisa <pisa@cmp.felk.cvut.cz>
// Antonino A. Daplas <adaplas@gmail.com>
// and others
//
// NOTES:
//
// Handles native and foreign byte order on both endians, standard and
// reverse pixel order in a byte (<8 BPP), word length of 32/64 bits,
// bits per pixel from 1 to the word length. Handles line lengths at byte
// granularity while maintaining aligned accesses.
//
// Optimized routines for word aligned 1, 2, 4 pixel per word for high
// bpp modes and 4 pixel at a time operation for low bpp.
//
// The color image is expected to be one byte per pixel, and values should
// not exceed the bitdepth or the pseudo palette (if used).
//

// bitmap image iterator, one pixel at a time
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_bitmap_iter {
    pub data: *const u8,
    pub colors: [c_ulong; 2],
    pub i: int width,,
}

// pixels = iter->colors[(iter->data[byte] >> bit) & 1];
// color image iterator, one pixel at a time
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_color_iter {
    pub data: *const u8,
    pub palette: *const u32,
    pub reverse: fb_reverse,
    pub shift: c_int,
    pub i: int width,,
}

// pixels = color << iter->shift;
// pixels = fb_reverse_bits_long(*pixels);
// bitmap image iterator, 4 pixels at a time
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_bitmap4x_iter {
    pub data: *const u8,
    pub bgcolor: u32 fgxcolor,,
    pub i: int width,,
    pub expand: *const u32,
    pub bpp: c_int,
    pub top: bool,
}

// bits = iter->bpp * iter->i;

// bits = iter->bpp * BITS_PER_BYTE/2;
// pixels = (iter->fgxcolor & iter->expand[data]) ^ iter->bgcolor;
// pixels <<= BITS_PER_LONG - *bits;

// draw a line a group of pixels at a time
// draw a color image a pixel at a time

// draw a 2 color image four pixels at a time (for 1-8 bpp only)
// draw a bitmap image 1 pixel at a time (for >8 bpp)

// one pixel per word, 64/32 bpp blitting

// aligned 32/16 bpp blitting

// aligned 16/8 bpp blitting

