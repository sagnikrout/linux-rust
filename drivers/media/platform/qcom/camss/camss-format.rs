//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-format.h
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
// camss-format.h
//
// Qualcomm MSM Camera Subsystem - Format helpers
//
// Copyright (c) 2023, The Linux Foundation. All rights reserved.
// Copyright (c) 2023 Qualcomm Technologies, Inc.
//

//
// struct fract - Represents a fraction
// @numerator: Store the numerator part of the fraction
// @denominator: Store the denominator part of the fraction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fract {
    pub numerator: u8,
    pub denominator: u8,
}

//
// struct camss_format_info - ISP media bus format information
// @code: V4L2 media bus format code
// @mbus_bpp: Media bus bits per pixel
// @pixelformat: V4L2 pixel format FCC identifier
// @planes: Number of planes
// @hsub: Horizontal subsampling (for each plane)
// @vsub: Vertical subsampling (for each plane)
// @bpp: Bits per pixel when stored in memory (for each plane)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_format_info {
    pub code: u32,
    pub mbus_bpp: u32,
    pub pixelformat: u32,
    pub planes: u8,
    pub hsub: [fract; 3],
    pub vsub: [fract; 3],
    pub bpp: [c_uint; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_formats {
    pub nformats: c_uint,
    pub formats: *const camss_format_info,
}

extern "C" {
    pub fn camss_format_get_bpp(formats: *const camss_format_info, nformats: c_uint, code: u32) -> u8;
}
extern "C" {
    pub fn camss_format_get_bpl_alignment(f: *const camss_format_info) -> c_uint;
}
extern "C" {
    pub fn camss_format_find_code(code: *mut u32, n_code: c_uint, index: c_uint, req_code: u32) -> u32;
}
