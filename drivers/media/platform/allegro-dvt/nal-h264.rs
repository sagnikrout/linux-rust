//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/allegro-dvt/nal-h264.h
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
// Copyright (C) 2019 Pengutronix, Michael Tretter <kernel@pengutronix.de>
//
// Convert NAL units between raw byte sequence payloads (RBSP) and C structs.
//

//
// struct nal_h264_hrd_parameters - HRD parameters
//
// C struct representation of the sequence parameter set NAL unit as defined by
// Rec. ITU-T H.264 (04/2017) E.1.2 HRD parameters syntax.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_h264_hrd_parameters {
    pub cpb_cnt_minus1: c_uint,
    pub bit_rate_scale: c_uint,
    pub cpb_size_scale: c_uint,
    pub bit_rate_value_minus1: [c_int; 16],
    pub cpb_size_value_minus1: [c_int; 16],
    pub cbr_flag: [c_uint; 16],
}

//
// struct nal_h264_vui_parameters - VUI parameters
//
// C struct representation of the VUI parameters as defined by Rec. ITU-T
// H.264 (04/2017) E.1.1 VUI parameters syntax.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_h264_vui_parameters {
    pub aspect_ratio_info_present_flag: c_uint,
    pub aspect_ratio_idc: c_uint,
    pub sar_width: c_uint,
    pub sar_height: c_uint,
}

//
// struct nal_h264_sps - Sequence parameter set
//
// C struct representation of the sequence parameter set NAL unit as defined by
// Rec. ITU-T H.264 (04/2017) 7.3.2.1.1 Sequence parameter set data syntax.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_h264_sps {
    pub profile_idc: c_uint,
    pub constraint_set0_flag: c_uint,
    pub constraint_set1_flag: c_uint,
    pub constraint_set2_flag: c_uint,
    pub constraint_set3_flag: c_uint,
    pub constraint_set4_flag: c_uint,
    pub constraint_set5_flag: c_uint,
    pub reserved_zero_2bits: c_uint,
    pub level_idc: c_uint,
    pub seq_parameter_set_id: c_uint,
    pub chroma_format_idc: c_uint,
    pub separate_colour_plane_flag: c_uint,
    pub bit_depth_luma_minus8: c_uint,
    pub bit_depth_chroma_minus8: c_uint,
    pub qpprime_y_zero_transform_bypass_flag: c_uint,
    pub seq_scaling_matrix_present_flag: c_uint,
}

//
// struct nal_h264_pps - Picture parameter set
//
// C struct representation of the picture parameter set NAL unit as defined by
// Rec. ITU-T H.264 (04/2017) 7.3.2.2 Picture parameter set RBSP syntax.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_h264_pps {
    pub pic_parameter_set_id: c_uint,
    pub seq_parameter_set_id: c_uint,
    pub entropy_coding_mode_flag: c_uint,
    pub bottom_field_pic_order_in_frame_present_flag: c_uint,
    pub num_slice_groups_minus1: c_uint,
    pub slice_group_map_type: c_uint,
    pub run_length_minus1: [c_uint; 8],
    pub top_left: [c_uint; 8],
    pub bottom_right: [c_uint; 8],
}

//
// nal_h264_profile() - Get profile_idc for v4l2 h264 profile
// @profile: the profile as &enum v4l2_mpeg_video_h264_profile
//
// Convert the &enum v4l2_mpeg_video_h264_profile to profile_idc as specified
// in Rec. ITU-T H.264 (04/2017) A.2.
//
// Return: the profile_idc for the passed level
//
// nal_h264_level() - Get level_idc for v4l2 h264 level
// @level: the level as &enum v4l2_mpeg_video_h264_level
//
// Convert the &enum v4l2_mpeg_video_h264_level to level_idc as specified in
// Rec. ITU-T H.264 (04/2017) A.3.2.
//
// Return: the level_idc for the passed level
//
// nal_h264_full_range() - Get video_full_range_flag for v4l2 quantization
// @quantization: the quantization type as &enum v4l2_quantization
//
// Convert the &enum v4l2_quantization to video_full_range_flag as specified in
// Rec. ITU-T H.264 (04/2017) E.2.1.
//
// Return: the video_full_range_flag value for the passed quantization
//
// nal_h264_color_primaries() - Get color_primaries for v4l2 colorspace
// @colorspace: the color space as &enum v4l2_colorspace
//
// Convert the &enum v4l2_colorspace to color_primaries as specified in
// Rec. ITU-T H.264 (04/2017) E.2.1.
//
// Return: the color_primaries value for the passed colorspace
//
// nal_h264_transfer_characteristics() - Get transfer_characteristics for v4l2 xfer_func
// @colorspace: the color space as &enum v4l2_colorspace
// @xfer_func: the transfer function as &enum v4l2_xfer_func
//
// Convert the &enum v4l2_xfer_func to transfer_characteristics as specified in
// Rec. ITU-T H.264 (04/2017) E.2.1.
//
// Return: the transfer_characteristics value for the passed transfer function
//
// nal_h264_matrix_coeffs() - Get matrix_coefficients for v4l2 v4l2_ycbcr_encoding
// @colorspace: the color space as &enum v4l2_colorspace
// @ycbcr_encoding: the ycbcr encoding as &enum v4l2_ycbcr_encoding
//
// Convert the &enum v4l2_ycbcr_encoding to matrix_coefficients as specified in
// Rec. ITU-T H.264 (04/2017) E.2.1.
//
// Return: the matrix_coefficients value for the passed encoding
//
extern "C" {
    pub fn nal_h264_print_sps(dev: *const device, sps: *mut nal_h264_sps);
}
extern "C" {
    pub fn nal_h264_print_pps(dev: *const device, pps: *mut nal_h264_pps);
}
extern "C" {
    pub fn nal_h264_write_filler(dev: *const device, dest: *mut c_void, n: usize) -> isize;
}
extern "C" {
    pub fn nal_h264_read_filler(dev: *const device, src: *mut c_void, n: usize) -> isize;
}
