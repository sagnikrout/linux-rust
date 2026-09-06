//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml/dml1_frl_cap_chk.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

pub const TB_BORROWED_MAX: c_int = 400;
pub const C_FRL_CB: c_int = 510;

pub const ACR_RATE_MAX: c_int = 1500;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_frl_pixel_encoding {
    HDMI_FRL_PIXEL_ENCODING_444,
    HDMI_FRL_PIXEL_ENCODING_422,
    HDMI_FRL_PIXEL_ENCODING_420
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frl_cap_chk_result {
    FRL_CAP_CHK_OK = 0,

    FRL_CAP_CHK_ERROR_AUDIO_BW   = -1,
    FRL_CAP_CHK_ERROR_BORROW     = -2,
    FRL_CAP_CHK_ERROR_MAX_BORROW = -3,
    FRL_CAP_CHK_ERROR_MARGIN     = -4,

    FRL_CAP_CHK_ERROR_UNSUPPORTED_AUDIO = -1000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frl_borrow_mode {
    FRL_BORROW_MODE_NONE,
    FRL_BORROW_MODE_FROM_ACTIVE,
    FRL_BORROW_MODE_FROM_BLANK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frl_link_rate {
    FRL_LINK_RATE_DISABLE = 0,
    FRL_LINK_RATE_3GBPS,
    FRL_LINK_RATE_6GBPS,
    FRL_LINK_RATE_6GBPS_4LANE,
    FRL_LINK_RATE_8GBPS,
    FRL_LINK_RATE_10GBPS,
    FRL_LINK_RATE_12GBPS,
    FRL_LINK_RATE_16GBPS,
    FRL_LINK_RATE_20GBPS,
    FRL_LINK_RATE_24GBPS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_dml_borrow_params {
    pub audio_packets_line: c_int,
    pub hc_active_target: c_int,
    pub hc_blank_target: c_int,
    pub borrow_mode: frl_borrow_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_primary_format {
    pub vic: u32,
    pub frl_rate: u32,
    pub frl_lanes: u32,
    pub hc_active: u32,
    pub hc_blank: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_cap_chk_intermediates {
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
pub struct frl_cap_chk_params {
    pub lanes: c_int,
    pub /: *mut *mut double f_pixel_clock_nominal; / Pixel Clock rate (Hz),
    pub /: *mut *mut double r_bit_nominal; / FRL bitrate (bps),
    pub audio_packet_type: c_int,
    pub /: *mut *mut double f_audio; / Audio rate (Hz),
    pub /: *mut *mut int h_active; / Active pixels per line,
    pub /: *mut *mut int h_blank; / Blanking pixels per line,
    pub /: *mut *mut int bpc; / Bits per component,
    pub /: *mut *mut int vic; / Video Identification Code,
    pub pixel_encoding: hdmi_frl_pixel_encoding,
    pub /: *mut *mut bool compressed; / set to true if DSC is enabled,
    pub /: *mut *mut bool bypass_hc_target_calc; / debug only,
    pub /: *mut *mut bool allow_all_bpp; / dsc_all_bpp,
// DSC parameters
    pub slices: c_int,
    pub slice_width: c_int,
    pub bpp_target: double,
    pub is_ovt: bool,
    pub layout: c_int,
    pub /: *mut *mut int acat; / not supported,
// outputs
    pub borrow_params: frl_dml_borrow_params,
    pub average_tribyte_rate: c_int,
}

extern "C" {
    pub fn dml1_frl_cap_chk(params: *mut frl_cap_chk_params) -> frl_cap_chk_result;
}
