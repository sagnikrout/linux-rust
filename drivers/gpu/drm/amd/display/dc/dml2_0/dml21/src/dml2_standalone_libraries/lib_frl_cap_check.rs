//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_standalone_libraries/lib_frl_cap_check.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lib_frl_cap_check_pixel_encoding {
    LIB_FRL_CAP_CHECK_PIXEL_ENCODING_444,
    LIB_FRL_CAP_CHECK_PIXEL_ENCODING_422,
    LIB_FRL_CAP_CHECK_PIXEL_ENCODING_420
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lib_frl_cap_check_borrow_mode {
    LIB_FRL_CAP_CHECK_BORROW_MODE_NONE,
    LIB_FRL_CAP_CHECK_BORROW_MODE_FROM_ACTIVE,
    LIB_FRL_CAP_CHECK_BORROW_MODE_FROM_BLANK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lib_frl_cap_check_status {
    LIB_FRL_CAP_CHECK_OK = 0,

    LIB_FRL_CAP_CHECK_ERROR_AUDIO_BW = -1,
    LIB_FRL_CAP_CHECK_ERROR_BORROW = -2,
    LIB_FRL_CAP_CHECK_ERROR_MAX_BORROW = -3,
    LIB_FRL_CAP_CHECK_ERROR_MARGIN = -4,

    LIB_FRL_CAP_CHECK_ERROR_UNSUPPORTED_AUDIO = -1000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lib_frl_cap_check_intermediates {
    pub c_frl_sb: c_int,
    pub overhead_sb: double,
    pub overhead_rs: double,
    pub overhead_map: double,
    pub overhead_min: double,
    pub overhead_max: double,
    pub f_pixel_clock_max: double,
    pub t_line: double,
    pub r_bit_min: double,
    pub r_frl_char_min: double,
    pub c_frl_line: double,
    pub ap: double,
    pub r_ap: double,
    pub avg_audio_packets_line: double,
    pub audio_packets_line: c_int,
    pub blank_audio_min: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lib_frl_cap_check_params {
    pub lanes: c_int,
    pub /: *mut *mut double f_pixel_clock_nominal; / Pixel Clock rate (Hz),
    pub /: *mut *mut double r_bit_nominal; / FRL bitrate (bps),
    pub audio_packet_type: c_int,
    pub /: *mut *mut double f_audio; / Audio rate (Hz),
    pub /: *mut *mut int h_active; / Active pixels per line,
    pub /: *mut *mut int h_blank; / Blanking pixels per line,
    pub /: *mut *mut int bpc; / Bits per component,
    pub pixel_encoding: lib_frl_cap_check_pixel_encoding,
    pub compressed: bool,
    pub bypass_hc_target_calc: bool,
// DSC parameters
    pub slices: c_int,
    pub slice_width: c_int,
    pub bpp_target: double,
    pub /: *mut *mut int layout; / not supported,
    pub /: *mut *mut int acat; / not supported,
// outputs
    pub audio_packets_line: c_int,
// inputs or outputs
    pub hc_active_target: c_int,
    pub hc_blank_target: c_int,
    pub borrow_mode: lib_frl_cap_check_borrow_mode,
}

extern "C" {
    pub fn frl_cap_check(params: *mut lib_frl_cap_check_params) -> lib_frl_cap_check_status;
}
extern "C" {
    pub fn frl_cap_check_intermediates(params: *mut lib_frl_cap_check_params, inter: *mut lib_frl_cap_check_intermediates) -> lib_frl_cap_check_status;
}
