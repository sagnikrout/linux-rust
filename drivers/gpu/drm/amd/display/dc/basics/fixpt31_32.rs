//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/basics/fixpt31_32.c
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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

    let mut dc_fixpt_two_pi: static struct fixed31_32 = { 26986075409LL };
    let mut dc_fixpt_ln2: static struct fixed31_32 = { 2977044471LL };
    let mut dc_fixpt_ln2_div_2: static struct fixed31_32 = { 1488522236LL };
    static inline unsigned long long abs_i64(
    long long arg)
    {
    if (arg > 0)
    return (unsigned long long)arg;
    else
    return (unsigned long long)(-arg);
    }
//
// @brief
// result = dividend / divisor
// *remainder = dividend % divisor
//
    static inline unsigned long long complete_integer_division_u64(
    unsigned long long dividend,
    unsigned long long divisor,
    unsigned long long *remainder)
    {
    unsigned long long result;
    result = div64_u64_rem(dividend, divisor, remainder);
    return result;
    }

    ((1ULL << FIXED31_32_BITS_PER_FRACTIONAL_PART) - 1)

    ((x) >> FIXED31_32_BITS_PER_FRACTIONAL_PART)

    (FRACTIONAL_PART_MASK & (x))
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_from_fraction(numerator: c_longlong, denominator: c_longlong) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_from_fraction(long long numerator, long long denominator)
    {
    struct fixed31_32 res;
    let mut arg1_negative: bool = numerator < 0;
    let mut arg2_negative: bool = denominator < 0;
    let mut arg1_value: c_ulonglong = arg1_negative ? -numerator : numerator;
    let mut arg2_value: c_ulonglong = arg2_negative ? -denominator : denominator;
    unsigned long long remainder;
// determine integer part
    unsigned long long res_value = complete_integer_division_u64(
    arg1_value, arg2_value, &remainder);
    ASSERT(res_value <= LONG_MAX);
// determine fractional part
    {
    let mut i: c_uint = FIXED31_32_BITS_PER_FRACTIONAL_PART;
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
    let mut summand: c_ulonglong = (remainder << 1) >= arg2_value;
    ASSERT(res_value <= LLONG_MAX - summand);
    res_value += summand;
    }
    res.value = (long long)res_value;
    if (arg1_negative ^ arg2_negative)
    res.value = -res.value;
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_mul(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_mul(struct fixed31_32 arg1, struct fixed31_32 arg2)
    {
    struct fixed31_32 res;
    let mut arg1_negative: bool = arg1.value < 0;
    let mut arg2_negative: bool = arg2.value < 0;
    let mut arg1_value: c_ulonglong = arg1_negative ? -arg1.value : arg1.value;
    let mut arg2_value: c_ulonglong = arg2_negative ? -arg2.value : arg2.value;
    let mut arg1_int: c_ulonglong = GET_INTEGER_PART(arg1_value);
    let mut arg2_int: c_ulonglong = GET_INTEGER_PART(arg2_value);
    let mut arg1_fra: c_ulonglong = GET_FRACTIONAL_PART(arg1_value);
    let mut arg2_fra: c_ulonglong = GET_FRACTIONAL_PART(arg2_value);
    unsigned long long tmp;
    res.value = arg1_int * arg2_int;
    res.value <<= FIXED31_32_BITS_PER_FRACTIONAL_PART;
    tmp = arg1_int * arg2_fra;
    ASSERT(tmp <= (unsigned long long)(LLONG_MAX - res.value));
    res.value += tmp;
    tmp = arg2_int * arg1_fra;
    ASSERT(tmp <= (unsigned long long)(LLONG_MAX - res.value));
    res.value += tmp;
    tmp = arg1_fra * arg2_fra;
    tmp = (tmp >> FIXED31_32_BITS_PER_FRACTIONAL_PART) +
    (tmp >= (unsigned long long)dc_fixpt_half.value);
    ASSERT(tmp <= (unsigned long long)(LLONG_MAX - res.value));
    res.value += tmp;
    if (arg1_negative ^ arg2_negative)
    res.value = -res.value;
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_sqr(arg: fixed31_32) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_sqr(struct fixed31_32 arg)
    {
    struct fixed31_32 res;
    let mut arg_value: c_ulonglong = abs_i64(arg.value);
    let mut arg_int: c_ulonglong = GET_INTEGER_PART(arg_value);
    let mut arg_fra: c_ulonglong = GET_FRACTIONAL_PART(arg_value);
    unsigned long long tmp;
    res.value = arg_int * arg_int;
    res.value <<= FIXED31_32_BITS_PER_FRACTIONAL_PART;
    tmp = arg_int * arg_fra;
    ASSERT(tmp <= (unsigned long long)(LLONG_MAX - res.value));
    res.value += tmp;
    ASSERT(tmp <= (unsigned long long)(LLONG_MAX - res.value));
    res.value += tmp;
    tmp = arg_fra * arg_fra;
    tmp = (tmp >> FIXED31_32_BITS_PER_FRACTIONAL_PART) +
    (tmp >= (unsigned long long)dc_fixpt_half.value);
    ASSERT(tmp <= (unsigned long long)(LLONG_MAX - res.value));
    res.value += tmp;
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_recip(arg: fixed31_32) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_recip(struct fixed31_32 arg)
    {
//
// @note
// Good idea to use Newton's method
//
    return dc_fixpt_from_fraction(
    dc_fixpt_one.value,
    arg.value);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_sinc(arg: fixed31_32) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_sinc(struct fixed31_32 arg)
    {
    struct fixed31_32 square;
    let mut res: fixed31_32 = dc_fixpt_one;
    let mut n: c_int = 27;
    let mut arg_norm: fixed31_32 = arg;
    if (dc_fixpt_le(
    dc_fixpt_two_pi,
    dc_fixpt_abs(arg))) {
    arg_norm = dc_fixpt_sub(
    arg_norm,
    dc_fixpt_mul_int(
    dc_fixpt_two_pi,
    (int)div64_s64(
    arg_norm.value,
    dc_fixpt_two_pi.value)));
    }
    square = dc_fixpt_sqr(arg_norm);
    do {
    res = dc_fixpt_sub(
    dc_fixpt_one,
    dc_fixpt_div_int(
    dc_fixpt_mul(
    square,
    res),
    n * (n - 1)));
    n -= 2;
    } while (n > 2);
    if (arg.value != arg_norm.value)
    res = dc_fixpt_div(
    dc_fixpt_mul(res, arg_norm),
    arg);
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_sin(arg: fixed31_32) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_sin(struct fixed31_32 arg)
    {
    return dc_fixpt_mul(
    arg,
    dc_fixpt_sinc(arg));
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_cos(arg: fixed31_32) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_cos(struct fixed31_32 arg)
    {
// TODO implement argument normalization
    let mut square: fixed31_32 = dc_fixpt_sqr(arg);
    let mut res: fixed31_32 = dc_fixpt_one;
    let mut n: c_int = 26;
    do {
    res = dc_fixpt_sub(
    dc_fixpt_one,
    dc_fixpt_div_int(
    dc_fixpt_mul(
    square,
    res),
    (long long)n * (n - 1)));
    n -= 2;
    } while (n != 0);
    return res;
    }
//
// @brief
// result = exp(arg),
// where abs(arg) < 1
//
// Calculated as Taylor series.
//
#[no_mangle]
unsafe extern "C" fn fixed31_32_exp_from_taylor_series(arg: fixed31_32) -> fixed31_32 {
    static struct fixed31_32 fixed31_32_exp_from_taylor_series(struct fixed31_32 arg)
    {
    let mut n: c_uint = 9;
    struct fixed31_32 res = dc_fixpt_from_fraction(
    n + 2,
    n + 1);
// TODO find correct res
    ASSERT(dc_fixpt_lt(arg, dc_fixpt_one));
    do
    res = dc_fixpt_add(
    dc_fixpt_one,
    dc_fixpt_div_int(
    dc_fixpt_mul(
    arg,
    res),
    n));
    while (--n != 1);
    return dc_fixpt_add(
    dc_fixpt_one,
    dc_fixpt_mul(
    arg,
    res));
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_exp(arg: fixed31_32) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_exp(struct fixed31_32 arg)
    {
//
// @brief
// Main equation is:
// exp(x) = exp(r + m * ln(2)) = (1 << m) * exp(r),
// where m = round(x / ln(2)), r = x - m * ln(2)
//
    if (dc_fixpt_le(
    dc_fixpt_ln2_div_2,
    dc_fixpt_abs(arg))) {
    int m = dc_fixpt_round(
    dc_fixpt_div(
    arg,
    dc_fixpt_ln2));
    struct fixed31_32 r = dc_fixpt_sub(
    arg,
    dc_fixpt_mul_int(
    dc_fixpt_ln2,
    m));
    ASSERT(m != 0);
    ASSERT(dc_fixpt_lt(
    dc_fixpt_abs(r),
    dc_fixpt_one));
    if (m > 0)
    return dc_fixpt_shl(
    fixed31_32_exp_from_taylor_series(r),
    (unsigned char)m);
    else
    return dc_fixpt_div_int(
    fixed31_32_exp_from_taylor_series(r),
    1LL << -m);
    } else if (arg.value != 0)
    return fixed31_32_exp_from_taylor_series(arg);
    else
    return dc_fixpt_one;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_log(arg: fixed31_32) -> fixed31_32 {
    struct fixed31_32 dc_fixpt_log(struct fixed31_32 arg)
    {
    let mut res: fixed31_32 = dc_fixpt_neg(dc_fixpt_one);
// TODO improve 1st estimation
    struct fixed31_32 error;
    ASSERT(arg.value > 0);
// TODO if arg is negative, return NaN
// TODO if arg is zero, return -INF
    do {
    struct fixed31_32 res1 = dc_fixpt_add(
    dc_fixpt_sub(
    res,
    dc_fixpt_one),
    dc_fixpt_div(
    arg,
    dc_fixpt_exp(res)));
    error = dc_fixpt_sub(
    res,
    res1);
    res = res1;
// TODO determine max_allowed_error based on quality of exp()
    } while (abs_i64(error.value) > 100ULL);
    return res;
    }
// this function is a generic helper to translate fixed point value to
// specified integer format that will consist of integer_bits integer part and
// fractional_bits fractional part. For example it is used in
// dc_fixpt_u2d19 to receive 2 bits integer part and 19 bits fractional
// part in 32 bits. It is used in hw programming (scaler)
//
    static inline unsigned int ux_dy(
    long long value,
    unsigned int integer_bits,
    unsigned int fractional_bits)
    {
// 1. create mask of integer part
    let mut result: c_uint = (1 << integer_bits) - 1;
// 2. mask out fractional part
    let mut fractional_part: c_uint = FRACTIONAL_PART_MASK & value;
// 3. shrink fixed point integer part to be of integer_bits width
    result &= GET_INTEGER_PART(value);
// 4. make space for fractional part to be filled in after integer
    result <<= fractional_bits;
// 5. shrink fixed point fractional part to of fractional_bits width
    fractional_part >>= FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits;
// 6. merge the result
    return result | fractional_part;
    }
    static inline unsigned int clamp_ux_dy(
    long long value,
    unsigned int integer_bits,
    unsigned int fractional_bits,
    unsigned int min_clamp)
    {
    let mut truncated_val: c_uint = ux_dy(value, integer_bits, fractional_bits);
    if (value >= (1LL << (integer_bits + FIXED31_32_BITS_PER_FRACTIONAL_PART)))
    return (1 << (integer_bits + fractional_bits)) - 1;
#[no_mangle]
pub unsafe extern "C" fn if(min_clamp: truncated_val >) -> else {
    else if (truncated_val > min_clamp)
    return truncated_val;
    else
    return min_clamp;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_u4d19(arg: fixed31_32) -> c_uint {
    unsigned int dc_fixpt_u4d19(struct fixed31_32 arg)
    {
    return ux_dy(arg.value, 4, 19);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_u3d19(arg: fixed31_32) -> c_uint {
    unsigned int dc_fixpt_u3d19(struct fixed31_32 arg)
    {
    return ux_dy(arg.value, 3, 19);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_u2d19(arg: fixed31_32) -> c_uint {
    unsigned int dc_fixpt_u2d19(struct fixed31_32 arg)
    {
    return ux_dy(arg.value, 2, 19);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_u0d19(arg: fixed31_32) -> c_uint {
    unsigned int dc_fixpt_u0d19(struct fixed31_32 arg)
    {
    return ux_dy(arg.value, 0, 19);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_clamp_u0d14(arg: fixed31_32) -> c_uint {
    unsigned int dc_fixpt_clamp_u0d14(struct fixed31_32 arg)
    {
    return clamp_ux_dy(arg.value, 0, 14, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_clamp_u0d10(arg: fixed31_32) -> c_uint {
    unsigned int dc_fixpt_clamp_u0d10(struct fixed31_32 arg)
    {
    return clamp_ux_dy(arg.value, 0, 10, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fixpt_s4d19(arg: fixed31_32) -> c_int {
    int dc_fixpt_s4d19(struct fixed31_32 arg)
    {
    if (arg.value < 0)
    return -(int)ux_dy(dc_fixpt_abs(arg).value, 4, 19);
    else
    return ux_dy(arg.value, 4, 19);
    }
    struct fixed31_32 dc_fixpt_from_ux_dy(unsigned int value,
    unsigned int integer_bits,
    unsigned int fractional_bits)
    {
    let mut fixpt_value: fixed31_32 = dc_fixpt_zero;
    let mut fixpt_int_value: fixed31_32 = dc_fixpt_zero;
    let mut frac_mask: c_longlong = ((long long)1 << (long long)integer_bits) - 1;
    fixpt_value.value = (long long)value << (FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits);
    frac_mask = frac_mask << fractional_bits;
    fixpt_int_value.value = value & frac_mask;
    fixpt_int_value.value <<= (FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits);
    fixpt_value.value |= fixpt_int_value.value;
    return fixpt_value;
    }
    struct fixed31_32 dc_fixpt_from_int_dy(unsigned int int_value,
    unsigned int frac_value,
    unsigned int integer_bits,
    unsigned int fractional_bits)
    {
    (void)integer_bits;
    let mut fixpt_value: fixed31_32 = dc_fixpt_from_int(int_value);
    fixpt_value.value |= (long long)frac_value << (FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits);
    return fixpt_value;
    }
