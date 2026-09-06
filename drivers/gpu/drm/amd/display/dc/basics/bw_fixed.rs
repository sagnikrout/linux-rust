//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/basics/bw_fixed.c
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
//
// Copyright 2023 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

    ((int64_t)((1ULL << 63) - 1))

    (-MAX_I64 - 1)

    ((1ULL << BW_FIXED_BITS_PER_FRACTIONAL_PART) - 1)

    (FRACTIONAL_PART_MASK & (x))
#[no_mangle]
unsafe extern "C" fn abs_i64(arg: i64) -> u64 {
    static uint64_t abs_i64(int64_t arg)
    {
    if (arg >= 0)
    return (uint64_t)(arg);
    else
    return (uint64_t)(-arg);
    }
#[no_mangle]
pub unsafe extern "C" fn bw_int_to_fixed_nonconst(value: i64) -> bw_fixed {
    struct bw_fixed bw_int_to_fixed_nonconst(int64_t value)
    {
    struct bw_fixed res;
    ASSERT(value < BW_FIXED_MAX_I32 && value > BW_FIXED_MIN_I32);
    res.value = value << BW_FIXED_BITS_PER_FRACTIONAL_PART;
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn bw_frc_to_fixed(numerator: i64, denominator: i64) -> bw_fixed {
    struct bw_fixed bw_frc_to_fixed(int64_t numerator, int64_t denominator)
    {
    struct bw_fixed res;
    let mut arg1_negative: bool = numerator < 0;
    let mut arg2_negative: bool = denominator < 0;
    uint64_t arg1_value;
    uint64_t arg2_value;
    uint64_t remainder;
// determine integer part
    uint64_t res_value;
    ASSERT(denominator != 0);
    arg1_value = abs_i64(numerator);
    arg2_value = abs_i64(denominator);
    res_value = div64_u64_rem(arg1_value, arg2_value, &remainder);
    ASSERT(res_value <= BW_FIXED_MAX_I32);
// determine fractional part
    {
    let mut i: u32 = BW_FIXED_BITS_PER_FRACTIONAL_PART;
    do {
    remainder <<= 1;
    res_value <<= 1;
    if (remainder >= arg2_value) {
    res_value |= 1;
    remainder -= arg2_value;
    }
    } while (--i != 0);
    }
// round up LSB
    {
    let mut summand: u64 = (remainder << 1) >= arg2_value;
    ASSERT(res_value <= MAX_I64 - summand);
    res_value += summand;
    }
    res.value = (int64_t)(res_value);
    if (arg1_negative ^ arg2_negative)
    res.value = -res.value;
    return res;
    }
    struct bw_fixed bw_floor2(const struct bw_fixed arg,
    const struct bw_fixed significance)
    {
    struct bw_fixed result;
    int64_t multiplicand;
    multiplicand = div64_s64(arg.value, abs_i64(significance.value));
    result.value = abs_i64(significance.value) * multiplicand;
    ASSERT(abs_i64(result.value) <= abs_i64(arg.value));
    return result;
    }
    struct bw_fixed bw_ceil2(const struct bw_fixed arg,
    const struct bw_fixed significance)
    {
    struct bw_fixed result;
    int64_t multiplicand;
    multiplicand = div64_s64(arg.value, abs_i64(significance.value));
    result.value = abs_i64(significance.value) * multiplicand;
    if (abs_i64(result.value) < abs_i64(arg.value)) {
    if (arg.value < 0)
    result.value -= abs_i64(significance.value);
    else
    result.value += abs_i64(significance.value);
    }
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn bw_mul(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed {
    struct bw_fixed bw_mul(const struct bw_fixed arg1, const struct bw_fixed arg2)
    {
    struct bw_fixed res;
    let mut arg1_negative: bool = arg1.value < 0;
    let mut arg2_negative: bool = arg2.value < 0;
    let mut arg1_value: u64 = abs_i64(arg1.value);
    let mut arg2_value: u64 = abs_i64(arg2.value);
    let mut arg1_int: u64 = BW_FIXED_GET_INTEGER_PART(arg1_value);
    let mut arg2_int: u64 = BW_FIXED_GET_INTEGER_PART(arg2_value);
    let mut arg1_fra: u64 = GET_FRACTIONAL_PART(arg1_value);
    let mut arg2_fra: u64 = GET_FRACTIONAL_PART(arg2_value);
    uint64_t tmp;
    res.value = arg1_int * arg2_int;
    ASSERT(res.value <= BW_FIXED_MAX_I32);
    res.value <<= BW_FIXED_BITS_PER_FRACTIONAL_PART;
    tmp = arg1_int * arg2_fra;
    ASSERT(tmp <= (uint64_t)(MAX_I64 - res.value));
    res.value += tmp;
    tmp = arg2_int * arg1_fra;
    ASSERT(tmp <= (uint64_t)(MAX_I64 - res.value));
    res.value += tmp;
    tmp = arg1_fra * arg2_fra;
    tmp = (tmp >> BW_FIXED_BITS_PER_FRACTIONAL_PART) +
    (tmp >= (uint64_t)(bw_frc_to_fixed(1, 2).value));
    ASSERT(tmp <= (uint64_t)(MAX_I64 - res.value));
    res.value += tmp;
    if (arg1_negative ^ arg2_negative)
    res.value = -res.value;
    return res;
    }
