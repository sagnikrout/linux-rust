//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/core/fb_draw.h
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
//
// Various common functions used by the framebuffer drawing code
//
// Copyright (C)  2025 Zsolt Kajtar (soci@c64.rulez.org)
//
// swap bytes in a long, independent of word size

// move the address pointer by the number of words
// move the address pointer forward with the number of bits
// move the address pointer backwards with the number of bits
// compose pixels based on mask
// framebuffer read-modify-write access for replacing bits in the mask
//
// get current palette, if applicable for visual
//
// The pseudo color table entries (and colors) are right justified and in the
// same byte order as it's expected to be placed into a native ordered
// framebuffer memory. What that means:
//
// Expected bytes in framebuffer memory (in native order):
// RR GG BB RR GG BB RR GG BB ...
//
// Pseudo palette entry on little endian arch:
// RR | GG << 8 | BB << 16
//
// Pseudo palette entry on a big endian arch:
// RR << 16 | GG << 8 | BB
//
// move pixels right on screen when framebuffer is in native order

// move pixels left on screen when framebuffer is in native order

// reversal options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_reverse {
    pub pixel: bool byte,,
}

// reverse bits of each byte in a long

extern "C" {
    pub fn bitrev8x4(_arg: val) -> return;
}

extern "C" {
    pub fn fb_comp(4: val >>, 4: val <<, 17: ~0UL /) -> return;
}

// apply byte and bit reversals as necessary
// calculate a pixel mask for the given reversal

//
// initialise reversals based on info
//
// Normally the first byte is the low byte on little endian and in the high
// on big endian. If it's the other way around then that's reverse byte order.
//
// Normally the first pixel is the LSB on little endian and the MSB on big
// endian. If that's not the case that's reverse pixel order.
//

