//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/dml/calcs/dcn_calc_math.c
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
// Copyright 2017 Advanced Micro Devices, Inc.
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

//
// NOTE:
// This file is gcc-parseable HW gospel, coming straight from HW engineers.
//
// It doesn't adhere to Linux kernel style and sometimes will do things in odd
// ways. Unless there is something clearly wrong with it the code should
// remain as-is as it provides us with a guarantee from HW that it is correct.
//
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_mod(arg1: float, arg2: float) -> float {
    float dcn_bw_mod(const float arg1, const float arg2)
    {
    if (isNaN(arg1))
    return arg2;
    if (isNaN(arg2))
    return arg1;
    return arg1 - arg1 * ((int) (arg1 / arg2));
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_min2(arg1: float, arg2: float) -> float {
    float dcn_bw_min2(const float arg1, const float arg2)
    {
    if (isNaN(arg1))
    return arg2;
    if (isNaN(arg2))
    return arg1;
    return arg1 < arg2 ? arg1 : arg2;
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_max(arg1: c_uint, arg2: c_uint) -> c_uint {
    unsigned int dcn_bw_max(const unsigned int arg1, const unsigned int arg2)
    {
    return arg1 > arg2 ? arg1 : arg2;
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_max2(arg1: float, arg2: float) -> float {
    float dcn_bw_max2(const float arg1, const float arg2)
    {
    if (isNaN(arg1))
    return arg2;
    if (isNaN(arg2))
    return arg1;
    return arg1 > arg2 ? arg1 : arg2;
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_floor2(arg: float, significance: float) -> float {
    float dcn_bw_floor2(const float arg, const float significance)
    {
    ASSERT(significance != 0);
    return ((int) (arg / significance)) * significance;
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_floor(arg: float) -> float {
    float dcn_bw_floor(const float arg)
    {
    return (float)((int)(arg));
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_ceil(arg: float) -> float {
    float dcn_bw_ceil(const float arg)
    {
    return (float)((int)(arg + 0.99999f));
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_ceil2(arg: float, significance: float) -> float {
    float dcn_bw_ceil2(const float arg, const float significance)
    {
    ASSERT(significance != 0);
    return ((int) (arg / significance + 0.99999)) * significance;
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_max3(v1: float, v2: float, v3: float) -> float {
    float dcn_bw_max3(float v1, float v2, float v3)
    {
    return v3 > dcn_bw_max2(v1, v2) ? v3 : dcn_bw_max2(v1, v2);
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_max5(v1: float, v2: float, v3: float, v4: float, v5: float) -> float {
    float dcn_bw_max5(float v1, float v2, float v3, float v4, float v5)
    {
    return dcn_bw_max3(v1, v2, v3) > dcn_bw_max2(v4, v5) ? dcn_bw_max3(v1, v2, v3) : dcn_bw_max2(v4, v5);
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_pow(a: float, exp: float) -> float {
    float dcn_bw_pow(float a, float exp)
    {
    float temp;
// ASSERT(exp == (int)exp);
    if ((int)exp == 0)
    return 1;
    temp = dcn_bw_pow(a, (float)((int)(exp / 2)));
    if (((int)exp % 2) == 0) {
    return temp * temp;
    } else {
    if ((int)exp > 0)
    return a * temp * temp;
    else
    return (temp * temp) / a;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_fabs(a: double) -> double {
    double dcn_bw_fabs(double a)
    {
    if (a > 0)
    return (a);
    else
    return (-a);
    }
#[no_mangle]
pub unsafe extern "C" fn dcn_bw_log(a: float, b: float) -> float {
    float dcn_bw_log(float a, float b)
    {
    let mut exp_ptr: *mut int  const = (int *)(&a);
    let mut x: c_int = *exp_ptr;
    let mut log_2: c_int = ((x >> 23) & 255) - 128;
    x &= ~(255 << 23);
    x += 127 << 23;
// exp_ptr = x;
    a = ((-1.0f / 3) * a + 2) * a - 2.0f / 3;
    if (b > 2.00001 || b < 1.99999)
    return (a + log_2) / dcn_bw_log(b, 2);
    else
    return (a + log_2);
    }
