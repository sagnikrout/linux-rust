//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/sspl/dc_spl_scl_easf_filters.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scale_ratio_to_reg_value_lookup {
    pub numer: c_int,
    pub denom: c_int,
    pub reg_value: u32,
}

extern "C" {
    pub fn SPL_NAMESPACE(ratio): spl_get_v_bf3_mode(struct spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(ratio): spl_get_h_bf3_mode(struct spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_reducer_gain6(int, ratio): spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_reducer_gain4(int, ratio): spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_gainRing6(int, ratio): spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_gainRing4(int, ratio): spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_3tap_uptilt_maxval(int, ratio): spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_3tap_dntilt_slope(int, ratio): spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_3tap_uptilt1_slope(int, ratio): spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_3tap_uptilt2_slope(int, ratio): spl_fixed31_32) -> u32;
}
extern "C" {
    pub fn SPL_NAMESPACE(taps: spl_get_3tap_uptilt2_offset(int, ratio): spl_fixed31_32) -> u32;
}
// public API
