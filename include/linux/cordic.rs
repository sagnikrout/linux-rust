//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cordic.h
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
// Copyright (c) 2011 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const CORDIC_ANGLE_GEN: c_int = 39797;
pub const CORDIC_PRECISION_SHIFT: c_int = 16;

//
// struct cordic_iq - i/q coordinate.
//
// @i: real part of coordinate (in phase).
// @q: imaginary part of coordinate (quadrature).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cordic_iq {
    pub i: i32,
    pub q: i32,
}

//
// cordic_calc_iq() - calculates the i/q coordinate for given angle.
//
// @theta: angle in degrees for which i/q coordinate is to be calculated.
// @coord: function output parameter holding the i/q coordinate.
//
// The function calculates the i/q coordinate for a given angle using the
// CORDIC algorithm. The coordinate consists of a real (i) and an
// imaginary (q) part. The real part is essentially the cosine of the
// angle and the imaginary part is the sine of the angle. The returned
// values are scaled by 2^16 for precision. The range for theta is
// for -180 degrees to +180 degrees. Passed values outside this range are
// converted before doing the actual calculation.
//
extern "C" {
    pub fn cordic_calc_iq(theta: i32) -> cordic_iq;
}
