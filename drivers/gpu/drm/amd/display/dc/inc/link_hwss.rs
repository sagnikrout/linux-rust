//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/link_hwss.h
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
// include basic type headers only

// forward declare dc core types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_hwss_ext {
// function pointers below may require to check for NULL if caller
// considers missing implementation as expected in some cases or none
// critical to be investigated immediately
//
    pub throttled_vcp_size): fixed31_32,
    pub throttled_vcp_size): fixed31_32,
    pub link_settings): *const dc_link_settings,
    pub tp_params): *mut encoder_set_dp_phy_pattern_param,
    pub lane_settings[LANE_COUNT_DP_MAX]): dc_lane_settings,
    pub table): *const link_mst_stream_allocation_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_hwss {
    pub ext: link_hwss_ext,
// function pointers below MUST be assigned to all types of link_hwss
//
    pub pipe_ctx): *mut *mut void (setup_stream_encoder)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut void (reset_stream_encoder)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut void (setup_stream_attribute)(struct pipe_ctx,
    pub signal): signal_type,
    pub audio_inst): *mut *mut audio_output audio_output, uint32_t,
    pub pipe_ctx): *mut *mut void (enable_audio_packet)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut void (disable_audio_packet)(struct pipe_ctx,
}
