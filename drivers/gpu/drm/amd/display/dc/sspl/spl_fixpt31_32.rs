//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/sspl/spl_fixpt31_32.h
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


// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.

pub const FIXED31_32_BITS_PER_FRACTIONAL_PART: c_int = 32;

//
// @brief
// Arithmetic operations on real numbers
// represented as fixed-point numbers.
// There are: 1 bit for sign,
// 31 bit for integer part,
// 32 bits for fractional part.
//
// @note
// Currently, overflows and underflows are asserted;
// no special result returned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_fixed31_32 {
    pub value: c_longlong,
}

//
// @brief
// Useful constants
//
// @brief
// Initialization routines
//
// @brief
// result = numerator / denominator
//
// @brief
// result = arg
//
// @brief
// Unary operators
//
// @brief
// result = -arg
//
// @brief
// result = abs(arg) := (arg >= 0) ? arg : -arg
//
extern "C" {
    pub fn spl_fixpt_neg(_arg: arg) -> return;
}
//
// @brief
// Binary relational operators
//
// @brief
// result = arg1 < arg2
//
// @brief
// result = arg1 <= arg2
//
// @brief
// result = arg1 == arg2
//
// @brief
// result = min(arg1, arg2) := (arg1 <= arg2) ? arg1 : arg2
//
// @brief
// result = max(arg1, arg2) := (arg1 <= arg2) ? arg2 : arg1
//
// @brief
// | min_value, when arg <= min_value
// result = | arg, when min_value < arg < max_value
// | max_value, when arg >= max_value
//
// @brief
// Binary shift operators
//
// @brief
// result = arg << shift
//
// @brief
// result = arg >> shift
//
// @brief
// Binary additive operators
//
// @brief
// result = arg1 + arg2
//
// @brief
// result = arg1 + arg2
//
extern "C" {
    pub fn spl_fixpt_add(_arg: arg1, _arg: spl_fixpt_from_int(arg2)) -> return;
}
//
// @brief
// result = arg1 - arg2
//
// @brief
// result = arg1 - arg2
//
extern "C" {
    pub fn spl_fixpt_sub(_arg: arg1, _arg: spl_fixpt_from_int(arg2)) -> return;
}
//
// @brief
// Binary multiplicative operators
//
// @brief
// result = arg1 * arg2
//
// @brief
// result = arg1 * arg2
//
extern "C" {
    pub fn SPL_NAMESPACE(_arg: spl_fixpt_mul(arg1, _arg: spl_fixpt_from_int(arg2))) -> return;
}
//
// @brief
// result = square(arg) := arg * arg
//
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_sqr(struct spl_fixed31_32) -> spl_fixed31_32;
}
//
// @brief
// result = arg1 / arg2
//
// @brief
// result = arg1 / arg2
//
extern "C" {
    pub fn SPL_NAMESPACE(_arg: spl_fixpt_from_fraction(arg1.value, _arg: arg2.value)) -> return;
}
//
// @brief
// Reciprocal function
//
// @brief
// result = reciprocal(arg) := 1 / arg
//
// @note
// No special actions taken in case argument is zero.
//
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_recip(struct spl_fixed31_32) -> spl_fixed31_32;
}
//
// @brief
// Trigonometric functions
//
// @brief
// result = sinc(arg) := sin(arg) / arg
//
// @note
// Argument specified in radians,
// internally it's normalized to [-2pi...2pi] range.
//
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_sinc(struct spl_fixed31_32) -> spl_fixed31_32;
}
//
// @brief
// result = sin(arg)
//
// @note
// Argument specified in radians,
// internally it's normalized to [-2pi...2pi] range.
//
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_sin(struct spl_fixed31_32) -> spl_fixed31_32;
}
//
// @brief
// result = cos(arg)
//
// @note
// Argument specified in radians
// and should be in [-2pi...2pi] range -
// passing arguments outside that range
// will cause incorrect result!
//
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_cos(struct spl_fixed31_32) -> spl_fixed31_32;
}
//
// @brief
// Transcendent functions
//
// @brief
// result = exp(arg)
//
// @note
// Currently, function is verified for abs(arg) <= 1.
//
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_exp(struct spl_fixed31_32) -> spl_fixed31_32;
}
//
// @brief
// result = log(arg)
//
// @note
// Currently, abs(arg) should be less than 1.
// No normalization is done.
// Currently, no special actions taken
// in case of invalid argument(s). Take care!
//
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_log(struct spl_fixed31_32) -> spl_fixed31_32;
}
//
// @brief
// Power function
//
// @brief
// result = pow(arg1, arg2)
//
// @note
// Currently, abs(arg1) should be less than 1. Take care!
//
// @brief
// Rounding functions
//
// @brief
// result = floor(arg) := greatest integer lower than or equal to arg
//
// @brief
// result = round(arg) := integer nearest to arg
//
// @brief
// result = ceil(arg) := lowest integer greater than or equal to arg
//
// the following two function are used in scaler hw programming to convert fixed
// point value to format 2 bits from integer part and 19 bits from fractional
// part. The same applies for u0d19, 0 bits from integer part and 19 bits from
// fractional
//
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_u4d19(struct spl_fixed31_32) -> c_uint;
}
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_u3d19(struct spl_fixed31_32) -> c_uint;
}
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_u2d19(struct spl_fixed31_32) -> c_uint;
}
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_u0d19(struct spl_fixed31_32) -> c_uint;
}
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_clamp_u0d14(struct spl_fixed31_32) -> c_uint;
}
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_clamp_u0d10(struct spl_fixed31_32) -> c_uint;
}
extern "C" {
    pub fn SPL_NAMESPACE(arg): spl_fixpt_s4d19(struct spl_fixed31_32) -> c_int;
}
