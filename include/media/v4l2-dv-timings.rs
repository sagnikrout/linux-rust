//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-dv-timings.h
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
// v4l2-dv-timings - Internal header with dv-timings helper functions
//
// Copyright 2013 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

//
// v4l2_calc_timeperframe - helper function to calculate timeperframe based
// v4l2_dv_timings fields.
// @t: Timings for the video mode.
//
// Calculates the expected timeperframe using the pixel clock value and
// horizontal/vertical measures. This means that v4l2_dv_timings structure
// must be correctly and fully filled.
//
extern "C" {
    pub fn v4l2_calc_timeperframe(t: *const v4l2_dv_timings) -> v4l2_fract;
}
//
// v4l2_dv_timings_presets: list of all dv_timings presets.
//
// typedef v4l2_check_dv_timings_fnc - timings check callback
//
// @t: the v4l2_dv_timings struct.
// @handle: a handle from the driver.
//
// Returns true if the given timings are valid.
//
extern "C" {
    pub fn v4l2_check_dv_timings_fnc(t: *const v4l2_dv_timings, handle: *mut c_void) -> typedef bool;
}
//
// v4l2_valid_dv_timings() - are these timings valid?
//
// @t:	  the v4l2_dv_timings struct.
// @cap: the v4l2_dv_timings_cap capabilities.
// @fnc: callback to check if this timing is OK. May be NULL.
// @fnc_handle: a handle that is passed on to @fnc.
//
// Returns true if the given dv_timings struct is supported by the
// hardware capabilities and the callback function (if non-NULL), returns
// false otherwise.
//
// v4l2_enum_dv_timings_cap() - Helper function to enumerate possible DV
// timings based on capabilities
//
// @t:	  the v4l2_enum_dv_timings struct.
// @cap: the v4l2_dv_timings_cap capabilities.
// @fnc: callback to check if this timing is OK. May be NULL.
// @fnc_handle: a handle that is passed on to @fnc.
//
// This enumerates dv_timings using the full list of possible CEA-861 and DMT
// timings, filtering out any timings that are not supported based on the
// hardware capabilities and the callback function (if non-NULL).
//
// If a valid timing for the given index is found, it will fill in @t and
// return 0, otherwise it returns -EINVAL.
//
// v4l2_find_dv_timings_cap() - Find the closest timings struct
//
// @t:	  the v4l2_enum_dv_timings struct.
// @cap: the v4l2_dv_timings_cap capabilities.
// @pclock_delta: maximum delta between t->pixelclock and the timing struct
// under consideration.
// @fnc: callback to check if a given timings struct is OK. May be NULL.
// @fnc_handle: a handle that is passed on to @fnc.
//
// This function tries to map the given timings to an entry in the
// full list of possible CEA-861 and DMT timings, filtering out any timings
// that are not supported based on the hardware capabilities and the callback
// function (if non-NULL).
//
// On success it will fill in @t with the found timings and it returns true.
// On failure it will return false.
//
// v4l2_find_dv_timings_cea861_vic() - find timings based on CEA-861 VIC
// @t:		the timings data.
// @vic:	CEA-861 VIC code
//
// On success it will fill in @t with the found timings and it returns true.
// On failure it will return false.
//
extern "C" {
    pub fn v4l2_find_dv_timings_cea861_vic(t: *mut v4l2_dv_timings, vic: u8) -> bool;
}
//
// v4l2_match_dv_timings() - do two timings match?
//
// @measured:	  the measured timings data.
// @standard:	  the timings according to the standard.
// @pclock_delta: maximum delta in Hz between standard->pixelclock and
// the measured timings.
// @match_reduced_fps: if true, then fail if V4L2_DV_FL_REDUCED_FPS does not
// match.
//
// Returns true if the two timings match, returns false otherwise.
//
// v4l2_print_dv_timings() - log the contents of a dv_timings struct
// @dev_prefix:device prefix for each log line.
// @prefix:	additional prefix for each log line, may be NULL.
// @t:		the timings data.
// @detailed:	if true, give a detailed log.
//
// v4l2_detect_cvt - detect if the given timings follow the CVT standard
//
// @frame_height: the total height of the frame (including blanking) in lines.
// @hfreq: the horizontal frequency in Hz.
// @vsync: the height of the vertical sync in lines.
// @active_width: active width of image (does not include blanking). This
// information is needed only in case of version 2 of reduced blanking.
// In other cases, this parameter does not have any effect on timings.
// @polarities: the horizontal and vertical polarities (same as struct
// v4l2_bt_timings polarities).
// @interlaced: if this flag is true, it indicates interlaced format
// @cap: the v4l2_dv_timings_cap capabilities.
// @fmt: the resulting timings.
//
// This function will attempt to detect if the given values correspond to a
// valid CVT format. If so, then it will return true, and fmt will be filled
// in with the found CVT timings.
//
// v4l2_detect_gtf - detect if the given timings follow the GTF standard
//
// @frame_height: the total height of the frame (including blanking) in lines.
// @hfreq: the horizontal frequency in Hz.
// @vsync: the height of the vertical sync in lines.
// @polarities: the horizontal and vertical polarities (same as struct
// v4l2_bt_timings polarities).
// @interlaced: if this flag is true, it indicates interlaced format
// @aspect: preferred aspect ratio. GTF has no method of determining the
// aspect ratio in order to derive the image width from the
// image height, so it has to be passed explicitly. Usually
// the native screen aspect ratio is used for this. If it
// is not filled in correctly, then 16:9 will be assumed.
// @cap: the v4l2_dv_timings_cap capabilities.
// @fmt: the resulting timings.
//
// This function will attempt to detect if the given values correspond to a
// valid GTF format. If so, then it will return true, and fmt will be filled
// in with the found GTF timings.
//
// v4l2_calc_aspect_ratio - calculate the aspect ratio based on bytes
// 0x15 and 0x16 from the EDID.
//
// @hor_landscape: byte 0x15 from the EDID.
// @vert_portrait: byte 0x16 from the EDID.
//
// Determines the aspect ratio from the EDID.
// See VESA Enhanced EDID standard, release A, rev 2, section 3.6.2:
// "Horizontal and Vertical Screen Size or Aspect Ratio"
//
extern "C" {
    pub fn v4l2_calc_aspect_ratio(hor_landscape: u8, vert_portrait: u8) -> v4l2_fract;
}
//
// v4l2_dv_timings_aspect_ratio - calculate the aspect ratio based on the
// v4l2_dv_timings information.
//
// @t: the timings data.
//
extern "C" {
    pub fn v4l2_dv_timings_aspect_ratio(t: *const v4l2_dv_timings) -> v4l2_fract;
}
//
// can_reduce_fps - check if conditions for reduced fps are true.
// @bt: v4l2 timing structure
//
// For different timings reduced fps is allowed if the following conditions
// are met:
//
// - For CVT timings: if reduced blanking v2 (vsync == 8) is true.
// - For CEA861 timings: if %V4L2_DV_FL_CAN_REDUCE_FPS flag is true.
//
// struct v4l2_hdmi_colorimetry - describes the HDMI colorimetry information
// @colorspace:		enum v4l2_colorspace, the colorspace
// @ycbcr_enc:		enum v4l2_ycbcr_encoding, Y'CbCr encoding
// @quantization:	enum v4l2_quantization, colorspace quantization
// @xfer_func:		enum v4l2_xfer_func, colorspace transfer function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_hdmi_colorimetry {
    pub colorspace: v4l2_colorspace,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
    pub xfer_func: v4l2_xfer_func,
}

extern "C" {
    pub fn v4l2_num_edid_blocks(edid: *const u8, max_blocks: c_uint) -> c_uint;
}
extern "C" {
    pub fn v4l2_set_edid_phys_addr(edid: *mut u8, size: c_uint, phys_addr: u16);
}
extern "C" {
    pub fn v4l2_phys_addr_for_input(phys_addr: u16, input: u8) -> u16;
}
extern "C" {
    pub fn v4l2_phys_addr_validate(phys_addr: u16, parent: *mut u16, port: *mut u16) -> c_int;
}
// Add support for exporting InfoFrames to debugfs
//
// HDMI InfoFrames start with a 3 byte header, then a checksum,
// followed by the actual IF payload.
//
// The payload length is limited to 30 bytes according to the HDMI spec,
// but since the length is encoded in 5 bits, it can be 31 bytes theoretically.
// So set the max length as 31 + 3 (header) + 1 (checksum) = 35.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_debugfs_if {
    pub if_dir: *mut dentry,
    pub priv: *mut c_void,
    pub if_read: v4l2_debugfs_if_read_t,
}

extern "C" {
    pub fn v4l2_debugfs_if_free(infoframes: *mut v4l2_debugfs_if);
}

