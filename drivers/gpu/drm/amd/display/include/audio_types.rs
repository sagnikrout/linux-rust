//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/audio_types.h
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

pub const AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS: c_int = 20;
pub const MAX_HW_AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS: c_int = 18;
pub const MULTI_CHANNEL_SPLIT_NO_ASSO_INFO: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_dp_link_info {
    pub link_bandwidth_kbps: u32,
    pub hblank_min_symbol_width: u32,
    pub encoding: dp_link_encoding,
    pub link_rate: dc_link_rate,
    pub lane_count: dc_lane_count,
    pub is_mst: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_crtc_info {
    pub h_total: u32,
    pub h_active: u32,
    pub v_active: u32,
    pub /: *mut *mut uint32_t requested_pixel_clock_100Hz; / in 100Hz,
    pub /: *mut *mut uint32_t calculated_pixel_clock_100Hz; / in 100Hz,
    pub dsc_bits_per_pixel: u32,
    pub dsc_num_slices: u32,
    pub color_depth: dc_color_depth,
    pub pixel_encoding: dc_pixel_encoding,
    pub refresh_rate: u16,
    pub pixel_repetition: u8,
    pub interlaced: bool,
    pub /: *mut *mut uint32_t frl_character_clock_kHz; / in KHz,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct azalia_clock_info {
    pub pixel_clock_in_10khz: u32,
    pub audio_dto_phase: u32,
    pub audio_dto_module: u32,
    pub audio_dto_wall_clock_ratio: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_dto_source {
    DTO_SOURCE_UNKNOWN = 0,
    DTO_SOURCE_ID0,
    DTO_SOURCE_ID1,
    DTO_SOURCE_ID2,
    DTO_SOURCE_ID3,
    DTO_SOURCE_ID4,
    DTO_SOURCE_ID5
}

// PLL information required for AZALIA DTO calculation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_pll_info {
    pub audio_dto_source_clock_in_khz: u32,
    pub ss_percentage: u32,
    pub dto_source: audio_dto_source,
    pub ss_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_channel_associate_info {
    pub ALL_CHANNEL_FL:4: u32,
    pub ALL_CHANNEL_FR:4: u32,
    pub ALL_CHANNEL_FC:4: u32,
    pub ALL_CHANNEL_Sub:4: u32,
    pub ALL_CHANNEL_SL:4: u32,
    pub ALL_CHANNEL_SR:4: u32,
    pub ALL_CHANNEL_BL:4: u32,
    pub ALL_CHANNEL_BR:4: u32,
    pub bits: },
    pub u32all: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_output {
// Front DIG id.
    pub engine_id: engine_id,
// encoder output signal
    pub signal: signal_type,
// video timing
    pub crtc_info: audio_crtc_info,
// DP link info
    pub dp_link_info: audio_dp_link_info,
// PLL for audio
    pub pll_info: audio_pll_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_payload {
    CHANNEL_SPLIT_MAPPINGCHANG = 0x9,
}
