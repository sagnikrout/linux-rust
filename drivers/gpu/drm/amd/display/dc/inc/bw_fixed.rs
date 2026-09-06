//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/bw_fixed.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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
pub const BW_FIXED_BITS_PER_FRACTIONAL_PART: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bw_fixed {
    pub value: i64,
}

extern "C" {
    pub fn bw_min2(_arg: bw_min2(v1, _arg: v2), _arg: v3) -> return;
}
extern "C" {
    pub fn bw_max2(_arg: bw_max2(v1, _arg: v2), _arg: v3) -> return;
}
extern "C" {
    pub fn bw_int_to_fixed_nonconst(value: i64) -> bw_fixed;
}
extern "C" {
    pub fn bw_int_to_fixed_nonconst(_arg: value) -> return;
}
extern "C" {
    pub fn bw_frc_to_fixed(num: i64, denum: i64) -> bw_fixed;
}
extern "C" {
    pub fn bw_mul(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed;
}
extern "C" {
    pub fn bw_frc_to_fixed(_arg: arg1.value, _arg: arg2.value) -> return;
}
extern "C" {
    pub fn bw_floor2(arg: bw_fixed, significance: bw_fixed) -> bw_fixed;
}
extern "C" {
    pub fn bw_ceil2(arg: bw_fixed, significance: bw_fixed) -> bw_fixed;
}
