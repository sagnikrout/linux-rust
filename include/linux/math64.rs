//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/math64.h
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
// div_u64_rem - unsigned 64bit divide with 32bit divisor with remainder
// @dividend: unsigned 64bit dividend
// @divisor: unsigned 32bit divisor
// @remainder: pointer to unsigned 32bit remainder
//
// Return: sets ``*remainder``, then returns dividend / divisor
//
// This is commonly provided by 32bit archs to provide an optimized 64bit
// divide.
//
// remainder = dividend % divisor;
//
// div_s64_rem - signed 64bit divide with 32bit divisor with remainder
// @dividend: signed 64bit dividend
// @divisor: signed 32bit divisor
// @remainder: pointer to signed 32bit remainder
//
// Return: sets ``*remainder``, then returns dividend / divisor
//
// remainder = dividend % divisor;
//
// div64_u64_rem - unsigned 64bit divide with 64bit divisor and remainder
// @dividend: unsigned 64bit dividend
// @divisor: unsigned 64bit divisor
// @remainder: pointer to unsigned 64bit remainder
//
// Return: sets ``*remainder``, then returns dividend / divisor
//
// remainder = dividend % divisor;
//
// div64_s64_rem - signed 64bit divide with 64bit divisor and remainder
// @dividend: signed 64bit dividend
// @divisor: signed 64bit divisor
// @remainder: pointer to signed 64bit remainder
//
// Return: sets ``*remainder``, then returns dividend / divisor
//
// remainder = dividend % divisor;
//
// div64_u64 - unsigned 64bit divide with 64bit divisor
// @dividend: unsigned 64bit dividend
// @divisor: unsigned 64bit divisor
//
// Return: dividend / divisor
//
// div64_s64 - signed 64bit divide with 64bit divisor
// @dividend: signed 64bit dividend
// @divisor: signed 64bit divisor
//
// Return: dividend / divisor
//

// remainder = do_div(dividend, divisor);

extern "C" {
    pub fn div_s64_rem(dividend: i64, divisor: i32, remainder: *mut i32) -> i64;
}

extern "C" {
    pub fn div64_u64_rem(dividend: u64, divisor: u64, remainder: *mut u64) -> u64;
}

extern "C" {
    pub fn div64_s64_rem(dividend: i64, divisor: i64, remainder: *mut i64) -> i64;
}

extern "C" {
    pub fn div64_u64(dividend: u64, divisor: u64) -> u64;
}

extern "C" {
    pub fn div64_s64(dividend: i64, divisor: i64) -> i64;
}

//
// div_u64 - unsigned 64bit divide with 32bit divisor
// @dividend: unsigned 64bit dividend
// @divisor: unsigned 32bit divisor
//
// This is the most common 64bit divide and should be used if possible,
// as many 32bit archs can optimize this variant better than a full 64bit
// divide.
//
// Return: dividend / divisor
//

extern "C" {
    pub fn div_u64_rem(_arg: dividend, _arg: divisor, _arg: &remainder) -> return;
}

//
// div_s64 - signed 64bit divide with 32bit divisor
// @dividend: signed 64bit dividend
// @divisor: signed 32bit divisor
//
// Return: dividend / divisor
//

extern "C" {
    pub fn div_s64_rem(_arg: dividend, _arg: divisor, _arg: &remainder) -> return;
}

extern "C" {
    pub fn iter_div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u64) -> u32;
}

//
// Many a GCC version messes this up and generates a 64x64 mult :-(
//

//
// Many a GCC version also messes this up.
// Zero extending b and then spilling everything to stack.
//

//
// Each of these lines computes a 64-bit intermediate result into "c",
// starting at bits 32-95.  The low 32-bits go into the result of the
// multiplication, the high 32-bits are carried into the next step.
//
// The 128-bit result of the multiplication is in rl.ll and rh.ll,
// shift it right and throw away the high part of the result.
//

//
// Extract the sign before the multiplication and put it back
// afterwards if needed.
//

// Bits 32-63 of the result will be in rh.l.low.
// Bits 0-31 of the result will be in rl.l.low.

//
// mul_u64_add_u64_div_u64 - unsigned 64bit multiply, add, and divide
// @a: first unsigned 64bit multiplicand
// @b: second unsigned 64bit multiplicand
// @c: unsigned 64bit addend
// @d: unsigned 64bit divisor
//
// Multiply two 64bit values together to generate a 128bit product
// add a third value and then divide by a fourth.
// The Generic code divides by 0 if @d is zero and returns ~0 on overflow.
// Architecture specific code may trap on zero or overflow.
//
// Return: (@a * @b + @c) / @d
//
extern "C" {
    pub fn mul_u64_add_u64_div_u64(a: u64, b: u64, c: u64, d: u64) -> u64;
}
//
// mul_u64_u64_div_u64 - unsigned 64bit multiply and divide
// @a: first unsigned 64bit multiplicand
// @b: second unsigned 64bit multiplicand
// @d: unsigned 64bit divisor
//
// Multiply two 64bit values together to generate a 128bit product
// and then divide by a third value.
// The Generic code divides by 0 if @d is zero and returns ~0 on overflow.
// Architecture specific code may trap on zero or overflow.
//
// Return: @a * @b / @d
//

//
// mul_u64_u64_div_u64_roundup - unsigned 64bit multiply and divide rounded up
// @a: first unsigned 64bit multiplicand
// @b: second unsigned 64bit multiplicand
// @d: unsigned 64bit divisor
//
// Multiply two 64bit values together to generate a 128bit product
// and then divide and round up.
// The Generic code divides by 0 if @d is zero and returns ~0 on overflow.
// Architecture specific code may trap on zero or overflow.
//
// Return: (@a * @b + @d - 1) / @d
//

//
// DIV64_U64_ROUND_UP - unsigned 64bit divide with 64bit divisor rounded up
// @ll: unsigned 64bit dividend
// @d: unsigned 64bit divisor
//
// Divide unsigned 64bit dividend by unsigned 64bit divisor
// and round up.
//
// Return: dividend / divisor rounded up
//

//
// DIV_U64_ROUND_UP - unsigned 64bit divide with 32bit divisor rounded up
// @ll: unsigned 64bit dividend
// @d: unsigned 32bit divisor
//
// Divide unsigned 64bit dividend by unsigned 32bit divisor
// and round up.
//
// Return: dividend / divisor rounded up
//

//
// DIV64_U64_ROUND_CLOSEST - unsigned 64bit divide with 64bit divisor rounded to nearest integer
// @dividend: unsigned 64bit dividend
// @divisor: unsigned 64bit divisor
//
// Divide unsigned 64bit dividend by unsigned 64bit divisor
// and round to closest integer.
//
// Return: dividend / divisor rounded to nearest integer
//

//
// DIV_U64_ROUND_CLOSEST - unsigned 64bit divide with 32bit divisor rounded to nearest integer
// @dividend: unsigned 64bit dividend
// @divisor: unsigned 32bit divisor
//
// Divide unsigned 64bit dividend by unsigned 32bit divisor
// and round to closest integer.
//
// Return: dividend / divisor rounded to nearest integer
//

//
// DIV_S64_ROUND_CLOSEST - signed 64bit divide with 32bit divisor rounded to nearest integer
// @dividend: signed 64bit dividend
// @divisor: signed 32bit divisor
//
// Divide signed 64bit dividend by signed 32bit divisor
// and round to closest integer.
//
// Return: dividend / divisor rounded to nearest integer
//

//
// roundup_u64 - Round up a 64bit value to the next specified 32bit multiple
// @x: the value to up
// @y: 32bit multiple to round up to
//
// Rounds @x to the next multiple of @y. For 32bit @x values, see roundup and
// the faster round_up() for powers of 2.
//
// Return: rounded up value.
//
