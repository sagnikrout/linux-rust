//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/display/komeda/komeda_color_mgmt.h
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
// (C) COPYRIGHT 2019 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

pub const KOMEDA_N_YUV2RGB_COEFFS: c_int = 12;
pub const KOMEDA_N_RGB2YUV_COEFFS: c_int = 12;
pub const KOMEDA_COLOR_PRECISION: c_int = 12;
pub const KOMEDA_N_GAMMA_COEFFS: c_int = 65;

pub const KOMEDA_N_CTM_COEFFS: c_int = 9;
extern "C" {
    pub fn drm_lut_to_fgamma_coeffs(lut_blob: *mut drm_property_blob, coeffs: *mut u32);
}
extern "C" {
    pub fn drm_ctm_to_coeffs(ctm_blob: *mut drm_property_blob, coeffs: *mut u32);
}
