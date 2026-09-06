//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/allegro-dvt/nal-hevc.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_hevc_profile_tier_level {
    pub general_profile_space: c_uint,
    pub general_tier_flag: c_uint,
    pub general_profile_idc: c_uint,
    pub general_profile_compatibility_flag: [c_uint; 32],
    pub general_progressive_source_flag: c_uint,
    pub general_interlaced_source_flag: c_uint,
    pub general_non_packed_constraint_flag: c_uint,
    pub general_frame_only_constraint_flag: c_uint,
    pub general_max_12bit_constraint_flag: c_uint,
    pub general_max_10bit_constraint_flag: c_uint,
    pub general_max_8bit_constraint_flag: c_uint,
    pub general_max_422chroma_constraint_flag: c_uint,
    pub general_max_420chroma_constraint_flag: c_uint,
    pub general_max_monochrome_constraint_flag: c_uint,
    pub general_intra_constraint_flag: c_uint,
    pub general_one_picture_only_constraint_flag: c_uint,
    pub general_lower_bit_rate_constraint_flag: c_uint,
    pub general_max_14bit_constraint_flag: c_uint,
    pub general_reserved_zero_33bits: c_uint,
}

// unsigned int general_one_picture_only_constraint_flag;
//
// struct nal_hevc_vps - Video parameter set
//
// C struct representation of the video parameter set NAL unit as defined by
// Rec. ITU-T H.265 (02/2018) 7.3.2.1 Video parameter set RBSP syntax
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_hevc_vps {
    pub video_parameter_set_id: c_uint,
    pub base_layer_internal_flag: c_uint,
    pub base_layer_available_flag: c_uint,
    pub max_layers_minus1: c_uint,
    pub max_sub_layers_minus1: c_uint,
    pub temporal_id_nesting_flag: c_uint,
    pub profile_tier_level: nal_hevc_profile_tier_level,
    pub sub_layer_ordering_info_present_flag: c_uint,
    pub max_dec_pic_buffering_minus1: [c_uint; 7],
    pub max_num_reorder_pics: [c_uint; 7],
    pub max_latency_increase_plus1: [c_uint; 7],
}

// hrd_parameters( cprms_present_flag[ i ], max_sub_layers_minus1 )
pub const N_HRD_PARAMS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_hevc_sub_layer_hrd_parameters {
    pub bit_rate_value_minus1: [c_uint; N_HRD_PARAMS],
    pub cpb_size_value_minus1: [c_uint; N_HRD_PARAMS],
    pub cbr_flag: [c_uint; N_HRD_PARAMS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_hevc_hrd_parameters {
    pub nal_hrd_parameters_present_flag: c_uint,
    pub vcl_hrd_parameters_present_flag: c_uint,
    pub sub_pic_hrd_params_present_flag: c_uint,
    pub tick_divisor_minus2: c_uint,
    pub du_cpb_removal_delay_increment_length_minus1: c_uint,
    pub sub_pic_cpb_params_in_pic_timing_sei_flag: c_uint,
    pub dpb_output_delay_du_length_minus1: c_uint,
}

//
// struct nal_hevc_vui_parameters - VUI parameters
//
// C struct representation of the VUI parameters as defined by Rec. ITU-T
// H.265 (02/2018) E.2.1 VUI parameters syntax.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_hevc_vui_parameters {
    pub aspect_ratio_info_present_flag: c_uint,
    pub aspect_ratio_idc: c_uint,
    pub sar_width: c_uint,
    pub sar_height: c_uint,
}

//
// struct nal_hevc_sps - Sequence parameter set
//
// C struct representation of the video parameter set NAL unit as defined by
// Rec. ITU-T H.265 (02/2018) 7.3.2.2 Sequence parameter set RBSP syntax
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_hevc_sps {
    pub video_parameter_set_id: c_uint,
    pub max_sub_layers_minus1: c_uint,
    pub temporal_id_nesting_flag: c_uint,
    pub profile_tier_level: nal_hevc_profile_tier_level,
    pub seq_parameter_set_id: c_uint,
    pub chroma_format_idc: c_uint,
    pub separate_colour_plane_flag: c_uint,
    pub pic_width_in_luma_samples: c_uint,
    pub pic_height_in_luma_samples: c_uint,
    pub conformance_window_flag: c_uint,
    pub conf_win_left_offset: c_uint,
    pub conf_win_right_offset: c_uint,
    pub conf_win_top_offset: c_uint,
    pub conf_win_bottom_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_hevc_pps {
    pub pps_pic_parameter_set_id: c_uint,
    pub pps_seq_parameter_set_id: c_uint,
    pub dependent_slice_segments_enabled_flag: c_uint,
    pub output_flag_present_flag: c_uint,
    pub num_extra_slice_header_bits: c_uint,
    pub sign_data_hiding_enabled_flag: c_uint,
    pub cabac_init_present_flag: c_uint,
    pub num_ref_idx_l0_default_active_minus1: c_uint,
    pub num_ref_idx_l1_default_active_minus1: c_uint,
    pub init_qp_minus26: c_int,
    pub constrained_intra_pred_flag: c_uint,
    pub transform_skip_enabled_flag: c_uint,
    pub cu_qp_delta_enabled_flag: c_uint,
    pub diff_cu_qp_delta_depth: c_uint,
    pub pps_cb_qp_offset: c_int,
    pub pps_cr_qp_offset: c_int,
    pub pps_slice_chroma_qp_offsets_present_flag: c_uint,
    pub weighted_pred_flag: c_uint,
    pub weighted_bipred_flag: c_uint,
    pub transquant_bypass_enabled_flag: c_uint,
    pub tiles_enabled_flag: c_uint,
    pub entropy_coding_sync_enabled_flag: c_uint,
    pub num_tile_columns_minus1: c_uint,
    pub num_tile_rows_minus1: c_uint,
    pub uniform_spacing_flag: c_uint,
    pub column_width_minus1: [c_uint; 1],
    pub row_height_minus1: [c_uint; 1],
}

//
// nal_hevc_profile() - Get profile_idc for v4l2 hevc profile
// @profile: the profile as &enum v4l2_mpeg_video_hevc_profile
//
// Convert the &enum v4l2_mpeg_video_hevc_profile to profile_idc as specified
// in Rec. ITU-T H.265 (02/2018) A.3.
//
// Return: the profile_idc for the passed level
//
// nal_hevc_tier() - Get tier_flag for v4l2 hevc tier
// @tier: the tier as &enum v4l2_mpeg_video_hevc_tier
//
// Convert the &enum v4l2_mpeg_video_hevc_tier to tier_flag as specified
// in Rec. ITU-T H.265 (02/2018) A.4.1.
//
// Return: the tier_flag for the passed tier
//
// nal_hevc_level() - Get level_idc for v4l2 hevc level
// @level: the level as &enum v4l2_mpeg_video_hevc_level
//
// Convert the &enum v4l2_mpeg_video_hevc_level to level_idc as specified in
// Rec. ITU-T H.265 (02/2018) A.4.1.
//
// Return: the level_idc for the passed level
//
// T-Rec-H.265 p. 280: general_level_idc and sub_layer_level_idc[ i ]
// shall be set equal to a value of 30 times the level number
// specified in Table A.6.
//
extern "C" {
    pub fn nal_hevc_write_filler(dev: *const device, dest: *mut c_void, n: usize) -> isize;
}
extern "C" {
    pub fn nal_hevc_read_filler(dev: *const device, src: *mut c_void, n: usize) -> isize;
}
