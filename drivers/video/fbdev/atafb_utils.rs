//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/atafb_utils.h
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
// =================================================================
// Utility Assembler Functions
// =================================================================
// ======================================================================
// Those of a delicate disposition might like to skip the next couple of
// pages.
//
// These functions are drop in replacements for memmove and
// memset(_, 0, _). However their five instances add at least a kilobyte
// to the object file. You have been warned.
//
// Not a great fan of assembler for the sake of it, but I think
// that these routines are at least 10 times faster than their C
// equivalents for large blits, and that's important to the lowest level of
// a graphics driver. Question is whether some scheme with the blitter
// would be faster. I suspect not for simple text system - not much
// asynchrony.
//
// Code is very simple, just gruesome expansion. Basic strategy is to
// increase data moved/cleared at each step to 16 bytes to reduce
// instruction per data move overhead. movem might be faster still
// For more than 15 bytes, we try to align the write direction on a
// longword boundary to get maximum speed. This is even more gruesome.
// Unaligned read/write used requires 68020+ - think this is a problem?
//
// Sorry!
//
// ++roman: I've optimized Robert's original versions in some minor
// aspects, e.g. moveq instead of movel, let gcc choose the registers,
// use movem in some places...
// For other modes than 1 plane, lots of more such assembler functions
// were needed (e.g. the ones using movep or expanding color values).
//
// ++andreas: more optimizations:
// ++andreas: Simple and fast version of memmove, assumes size is

//
// This expands a up to 8 bit color into two longs
// for movel operations.
//

//
// set an 8bit value to a color
//

//
// set an 8bit value according to foreground/background color
//

// dst++ = m[0];

// dst++ = m[1];

// dst++ = m[2];
// dst++ = m[3];

// d++ = v;

// d++ = v;

// d++ = v;

