//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/davinci/vpif.h
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
// VPIF header file
//
// Copyright (C) 2009 Texas Instruments Incorporated - https://www.ti.com
//

// Maximum channel allowed

// Macros to read/write registers

// Register Address Offsets

// Functions for bit Manipulation
// Macro for Generating mask

// Bit positions in the channel control registers

// Mask various length

// VPIF masks for registers

// bit posotion of interrupt vpif_ch_intr register

// bit position of clock and channel enable in vpif_chn_ctrl register

pub const VPIF_CH2_CLIP_ANC_EN: c_int = 14;
pub const VPIF_CH2_CLIP_ACTIVE_EN: c_int = 13;
pub const VPIF_CH3_CLIP_ANC_EN: c_int = 14;
pub const VPIF_CH3_CLIP_ACTIVE_EN: c_int = 13;
// enabled interrupt on both the fields on vpid_ch0_ctrl register

// enabled interrupt on both the fields on vpid_ch1_ctrl register

// enabled interrupt on both the fields on vpid_ch0_ctrl register

// enabled interrupt on both the fields on vpid_ch1_ctrl register

// inline function to enable/disable channel0
// inline function to enable/disable channel1
// inline function to enable interrupt for channel0
// inline function to enable interrupt for channel1
// inline function to set buffer addresses in case of Y/C non mux mode
// inline function to set buffer addresses in VPIF registers for video data
// Inline function to enable raw vbi in the given channel
// inline function to enable/disable channel2
// inline function to enable/disable channel3
// inline function to enable interrupt for channel2
// inline function to enable interrupt for channel3
// inline function to enable raw vbi data for channel2
// inline function to enable raw vbi data for channel3
// function to enable clipping (for both active and blanking regions) on ch 2
// function to enable clipping (for both active and blanking regions) on ch 3
// inline function to set buffer addresses in case of Y/C non mux mode
// inline function to set buffer addresses in VPIF registers for video data
// inline function to set buffer addresses in VPIF registers for vbi data

// This structure will store size parameters as per the mode selected by user
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_channel_config_params {
    pub /: *mut *mut char name[VPIF_MAX_NAME]; / Name of the mode,
    pub /: *mut *mut u16 width; / Indicates width of the image,
    pub /: *mut *mut u16 height; / Indicates height of the image,
    pub /: *mut *mut u8 frm_fmt; / Interlaced (0) or progressive (1),
    pub (1): *mut *mut u8 ycmux_mode; / This mode requires one (0) or two,
    pub /: *mut *mut u16 eav2sav; / length of eav 2 sav,
    pub /: *mut *mut u16 sav2eav; / length of sav 2 eav,
    pub /: *mut *mut u16 l1, l3, l5, l7, l9, l11; / Other parameter configurations,
    pub /: *mut *mut u16 vsize; / Vertical size of the image,
    pub format: *mut *mut u8 capture_format; / Indicates whether capture,
// is in BT or in CCD/CMOS
    pub mode: *mut *mut u8 vbi_supported; / Indicates whether this,
// supports capturing vbi or not
    pub /: *mut *mut u8 hd_sd; / HDTV (1) or SDTV (0) format,
    pub /: *mut *mut v4l2_std_id stdid; / SDTV format,
    pub /: *mut *mut v4l2_dv_timings dv_timings; / HDTV format,
}

extern "C" {
    pub fn vpif_set_video_params(vpifparams: *mut vpif_params, channel_id: u8) -> c_int;
}
extern "C" {
    pub fn vpif_channel_getfid(channel_id: u8) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_size {
    _8BITS = 0,
    _10BITS,
    _12BITS,
}

// Structure for vpif parameters for raw vbi data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_vbi_params {
    pub /: *mut *mut __u32 hstart0; / Horizontal start of raw vbi data for first field,
    pub /: *mut *mut __u32 vstart0; / Vertical start of raw vbi data for first field,
    pub /: *mut *mut __u32 hsize0; / Horizontal size of raw vbi data for first field,
    pub /: *mut *mut __u32 vsize0; / Vertical size of raw vbi data for first field,
    pub /: *mut *mut __u32 hstart1; / Horizontal start of raw vbi data for second field,
    pub /: *mut *mut __u32 vstart1; / Vertical start of raw vbi data for second field,
    pub /: *mut *mut __u32 hsize1; / Horizontal size of raw vbi data for second field,
    pub /: *mut *mut __u32 vsize1; / Vertical size of raw vbi data for second field,
}

// structure for vpif parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_video_params {
    pub /: *mut *mut __u8 storage_mode; / Indicates field or frame mode,
    pub hpitch: c_ulong,
    pub stdid: v4l2_std_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_params {
    pub iface: vpif_interface,
    pub video_params: vpif_video_params,
    pub std_info: vpif_channel_config_params,
#[repr(C)]
#[derive(Copy, Clone)]
pub union param {
    pub vbi_params: vpif_vbi_params,
    pub data_sz: data_size,
    pub params: },
}
