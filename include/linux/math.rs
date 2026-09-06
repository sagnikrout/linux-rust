//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/math.h
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
// This looks more complex than it should be. But we need to
// get the type for the ~ right in round_down (it needs to be
// as wide as the result!), and we want to evaluate the macro
// arguments just once each.
//

//
// round_up - round up to next specified power of 2
// @x: the value to round
// @y: multiple to round up to (must be a power of 2)
//
// Rounds @x up to next multiple of @y (which must be a power of 2).
// To perform arbitrary rounding up, use roundup() below.
//

//
// round_down - round down to next specified power of 2
// @x: the value to round
// @y: multiple to round down to (must be a power of 2)
//
// Rounds @x down to next multiple of @y (which must be a power of 2).
// To perform arbitrary rounding down, use rounddown() below.
//

//
// DIV_ROUND_UP_POW2 - divide and round up
// @n: numerator
// @d: denominator (must be a power of 2)
//
// Divides @n by @d and rounds up to next multiple of @d (which must be a power
// of 2). Avoids integer overflows that may occur with __KERNEL_DIV_ROUND_UP().
// Performance is roughly equivalent to __KERNEL_DIV_ROUND_UP().
//

//
// roundup - round up to the next specified multiple
// @x: the value to up
// @y: multiple to round up to
//
// Rounds @x up to next multiple of @y. If @y will always be a power
// of 2, consider using the faster round_up().
//

//
// rounddown - round down to next specified multiple
// @x: the value to round
// @y: multiple to round down to
//
// Rounds @x down to next multiple of @y. If @y will always be a power
// of 2, consider using the faster round_down().
//

//
// Same as above but for u64 dividends. divisor must be a 32-bit
// number.
//

// Calculate "x * n / d" without unnecessary overflow or loss of precision.

//
// abs - return absolute value of an argument
// @x: the value.
//
// If it is unsigned type, @x is converted to signed type first.
// char is treated as if it was signed (regardless of whether it really is)
// but the macro's return type is preserved as char.
//
// NOTE, for signed type if @x is the minimum, the returned result is undefined
// as there is not enough bits to represent it as a positive number.
//
// Return: an absolute value of @x.
//

//
// abs_diff - return absolute value of the difference between the arguments
// @a: the first argument
// @b: the second argument
//
// @a and @b have to be of the same type. With this restriction we compare
// signed to signed and unsigned to unsigned. The result is the subtraction
// the smaller of the two from the bigger, hence result is always a positive
// value.
//
// Return: an absolute value of the difference between the @a and @b.
//

//
// reciprocal_scale - "scale" a value into range [0, ep_ro)
// @val: value
// @ep_ro: right open interval endpoint
//
// Perform a "reciprocal multiplication" in order to "scale" a value into
// range [0, @ep_ro), where the upper interval endpoint is right-open.
// This is useful, e.g. for accessing a index of an array containing
// @ep_ro elements, for example. Think of it as sort of modulus, only that
// the result isn't that of modulo. ;) Note that if initial input is a
// small value, then result will return 0.
//
// Return: a result based on @val in interval [0, @ep_ro).
//
extern "C" {
    pub fn int_pow(base: u64, exp: c_uint) -> u64;
}
extern "C" {
    pub fn int_sqrt(long: unsigned) -> c_ulong;
}

extern "C" {
    pub fn int_sqrt64(x: u64) -> u32;
}

