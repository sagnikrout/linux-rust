//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dp/dp_utils.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2024, The Linux Foundation. All rights reserved.
//

pub const HEADER_BYTE_0_BIT: c_int = 0;
pub const PARITY_BYTE_0_BIT: c_int = 8;
pub const HEADER_BYTE_1_BIT: c_int = 16;
pub const PARITY_BYTE_1_BIT: c_int = 24;
pub const HEADER_BYTE_2_BIT: c_int = 0;
pub const PARITY_BYTE_2_BIT: c_int = 8;
pub const HEADER_BYTE_3_BIT: c_int = 16;
pub const PARITY_BYTE_3_BIT: c_int = 24;

extern "C" {
    pub fn msm_dp_utils_get_g0_value(data: u8) -> u8;
}
extern "C" {
    pub fn msm_dp_utils_get_g1_value(data: u8) -> u8;
}
extern "C" {
    pub fn msm_dp_utils_calculate_parity(data: u32) -> u8;
}
extern "C" {
    pub fn msm_dp_utils_pack_sdp_header(sdp_header: *mut dp_sdp_header, header_buff[2]: u32);
}
