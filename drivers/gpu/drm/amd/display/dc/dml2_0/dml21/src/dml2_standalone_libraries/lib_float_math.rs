//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_standalone_libraries/lib_float_math.h
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
// Copyright 2024 Advanced Micro Devices, Inc.
extern "C" {
    pub fn math_mod(arg1: double, arg2: double) -> double;
}
extern "C" {
    pub fn math_min2(arg1: double, arg2: double) -> double;
}
extern "C" {
    pub fn math_max2(arg1: double, arg2: double) -> double;
}
extern "C" {
    pub fn math_floor2(arg: double, significance: double) -> double;
}
extern "C" {
    pub fn math_floor(arg: double) -> double;
}
extern "C" {
    pub fn math_ceil(arg: double) -> double;
}
extern "C" {
    pub fn math_ceil2(arg: double, significance: double) -> double;
}
extern "C" {
    pub fn math_max3(v1: double, v2: double, v3: double) -> double;
}
extern "C" {
    pub fn math_max4(v1: double, v2: double, v3: double, v4: double) -> double;
}
extern "C" {
    pub fn math_max5(v1: double, v2: double, v3: double, v4: double, v5: double) -> double;
}
extern "C" {
    pub fn math_pow(a: float, exp: float) -> float;
}
extern "C" {
    pub fn math_fabs(a: double) -> double;
}
extern "C" {
    pub fn math_log(a: float, b: float) -> float;
}
extern "C" {
    pub fn math_log2(a: float) -> float;
}
extern "C" {
    pub fn math_log2_approx(a: c_uint) -> c_uint;
}
extern "C" {
    pub fn math_round(a: double) -> double;
}
