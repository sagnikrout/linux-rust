//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fixp-arith.h
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
// Simplistic fixed-point arithmetics.
// Hmm, I'm probably duplicating some code :(
//
// Copyright (c) 2002 Johann Deneux
//
// Should you need to contact me, the author, you can do so by
// e-mail - mail your message to <johann.deneux@gmail.com>
//

//
// __fixp_sin32() - returns the sin of an angle in degrees
//
// @degrees: angle, in degrees, from 0 to 360.
//
// The returned value ranges from -0x7fffffff to +0x7fffffff.
//
// fixp_sin32() - returns the sin of an angle in degrees
//
// @degrees: angle, in degrees. The angle can be positive or negative
//
// The returned value ranges from -0x7fffffff to +0x7fffffff.
//
extern "C" {
    pub fn __fixp_sin32(_arg: degrees) -> return;
}
// cos(x) = sin(x + 90 degrees)

//
// 16 bits variants
//
// The returned value ranges from -0x7fff to 0x7fff
//

//
// fixp_sin32_rad() - calculates the sin of an angle in radians
//
// @radians: angle, in radians
// @twopi: value to be used for 2*pi
//
// Provides a variant for the cases where just 360
// values is not enough. This function uses linear
// interpolation to a wider range of values given by
// twopi var.
//
// Experimental tests gave a maximum difference of
// 0.000038 between the value calculated by sin() and
// the one produced by this function, when twopi is
// equal to 360000. That seems to be enough precision
// for practical purposes.
//
// Please notice that two high numbers for twopi could cause
// overflows, so the routine will not allow values of twopi
// bigger than 1^18.
//
// Avoid too large values for twopi, as we don't want overflows.
//
// cos(x) = sin(x + pi/2 radians)

//
// fixp_linear_interpolate() - interpolates a value from two known points
//
// @x0: x value of point 0
// @y0: y value of point 0
// @x1: x value of point 1
// @y1: y value of point 1
// @x: the linear interpolant
//
