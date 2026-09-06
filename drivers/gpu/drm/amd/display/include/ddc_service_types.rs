//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/ddc_service_types.h
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
// 0010FA dongles (ST Micro) external converter chip id
pub const DP_BRANCH_DEVICE_ID_0010FA: c_uint = 0x0010FA;
// 0022B9 external converter chip id
pub const DP_BRANCH_DEVICE_ID_0022B9: c_uint = 0x0022B9;
pub const DP_BRANCH_DEVICE_ID_00001A: c_uint = 0x00001A;
pub const DP_BRANCH_DEVICE_ID_0080E1: c_uint = 0x0080e1;
pub const DP_BRANCH_DEVICE_ID_90CC24: c_uint = 0x90CC24;
pub const DP_BRANCH_DEVICE_ID_00E04C: c_uint = 0x00E04C;
pub const DP_BRANCH_DEVICE_ID_006037: c_uint = 0x006037;
pub const DP_BRANCH_DEVICE_ID_001CF8: c_uint = 0x001CF8;
pub const DP_BRANCH_DEVICE_ID_0060AD: c_uint = 0x0060AD;
pub const DP_BRANCH_DEVICE_ID_001FF2: c_uint = 0x001FF2;
pub const DP_BRANCH_HW_REV_10: c_uint = 0x10;
pub const DP_BRANCH_HW_REV_20: c_uint = 0x20;
pub const DP_DEVICE_ID_0022B9: c_uint = 0x0022B9;
pub const DP_DEVICE_ID_38EC11: c_uint = 0x38EC11;
pub const DP_DEVICE_ID_BA4159: c_uint = 0xBA4159;
pub const DP_FORCE_PSRSU_CAPABILITY: c_uint = 0x40F;
pub const DP_SINK_PSR_ACTIVE_VTOTAL_CONTROL_CAP: c_uint = 0x370;
pub const DP_SINK_PSR_ACTIVE_VTOTAL: c_uint = 0x373;
pub const DP_SINK_PSR_ACTIVE_VTOTAL_CONTROL_MODE: c_uint = 0x375;
pub const DP_SOURCE_PSR_ACTIVE_VTOTAL: c_uint = 0x376;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ddc_result {
    DDC_RESULT_UNKNOWN = 0,
    DDC_RESULT_SUCESSFULL,
    DDC_RESULT_FAILED_CHANNEL_BUSY,
    DDC_RESULT_FAILED_TIMEOUT,
    DDC_RESULT_FAILED_PROTOCOL_ERROR,
    DDC_RESULT_FAILED_NACK,
    DDC_RESULT_FAILED_INCOMPLETE,
    DDC_RESULT_FAILED_OPERATION,
    DDC_RESULT_FAILED_INVALID_OPERATION,
    DDC_RESULT_FAILED_BUFFER_OVERFLOW,
    DDC_RESULT_FAILED_HPD_DISCON
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ddc_service_type {
    DDC_SERVICE_TYPE_CONNECTOR,
    DDC_SERVICE_TYPE_DISPLAY_PORT_MST,
}

//
// display sink capability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_sink_capability {
// dongle type (DP converter, CV smart dongle)
    pub dongle_type: display_dongle_type,
    pub is_dongle_type_one: bool,
//
// Dongle's downstream count.
    pub downstrm_sink_count: u32,
// Is dongle's downstream count info field (downstrm_sink_count)
// valid.
    pub downstrm_sink_count_valid: bool,
// Maximum additional audio delay in microsecond (us)
    pub additional_audio_delay: u32,
// Audio latency value in microsecond (us)
    pub audio_latency: u32,
// Interlace video latency value in microsecond (us)
    pub video_latency_interlace: u32,
// Progressive video latency value in microsecond (us)
    pub video_latency_progressive: u32,
// Dongle caps: Maximum pixel clock supported over dongle for HDMI
    pub max_hdmi_pixel_clock: u32,
// Dongle caps: Maximum deep color supported over dongle for HDMI
    pub max_hdmi_deep_color: dc_color_depth,
//
// support for Spread Spectrum(SS)
    pub ss_supported: bool,
// DP link settings (laneCount, linkRate, Spread)
    pub dp_link_lane_count: u32,
    pub dp_link_rate: u32,
    pub dp_link_spead: u32,
// If dongle_type == DISPLAY_DONGLE_DP_HDMI_CONVERTER,
    pub is_dp_hdmi_s3d_converter: bool,
// to check if we have queried the display capability
// for eDP panel already.
    pub is_edp_sink_cap_valid: bool,
    pub transaction_type: ddc_transaction_type,
    pub signal: signal_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct av_sync_data {
    pub /: *mut *mut uint8_t av_granularity;/ DPCD 00023h,
    pub /: *mut *mut uint8_t aud_dec_lat1;/ DPCD 00024h,
    pub /: *mut *mut uint8_t aud_dec_lat2;/ DPCD 00025h,
    pub /: *mut *mut uint8_t aud_pp_lat1;/ DPCD 00026h,
    pub /: *mut *mut uint8_t aud_pp_lat2;/ DPCD 00027h,
    pub /: *mut *mut uint8_t vid_inter_lat;/ DPCD 00028h,
    pub /: *mut *mut uint8_t vid_prog_lat;/ DPCD 00029h,
    pub /: *mut *mut uint8_t aud_del_ins1;/ DPCD 0002Bh,
    pub /: *mut *mut uint8_t aud_del_ins2;/ DPCD 0002Ch,
    pub /: *mut *mut uint8_t aud_del_ins3;/ DPCD 0002Dh,
}
