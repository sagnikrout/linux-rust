//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_core/dml2_core_utils.h
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

extern "C" {
    pub fn dml2_core_utils_div_rem(dividend: double, divisor: c_uint, remainder: *mut c_uint) -> double;
}
extern "C" {
    pub fn dml2_core_utils_is_420(source_format: dml2_source_format_class) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_422_planar(source_format: dml2_source_format_class) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_422_packed(source_format: dml2_source_format_class) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_print_mode_support_info(support: *const dml2_core_internal_mode_support_info, fail_only: bool);
}
extern "C" {
    pub fn dml2_core_utils_get_stream_output_bpp(out_bpp: *mut double, display_cfg: *const dml2_display_cfg);
}
extern "C" {
    pub fn dml2_core_utils_round_to_multiple(num: c_uint, multiple: c_uint, up: bool) -> c_uint;
}
extern "C" {
    pub fn dml2_core_util_get_num_active_pipes(num_planes: int unsigned, cfg_support_info: *const core_display_cfg_support_info) -> c_uint;
}
extern "C" {
    pub fn dml2_core_utils_pipe_plane_mapping(cfg_support_info: *const core_display_cfg_support_info, pipe_plane: *mut c_uint);
}
extern "C" {
    pub fn dml2_core_utils_is_phantom_pipe(plane_cfg: *const dml2_plane_parameters) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_get_tile_block_size_bytes(sw_mode: dml2_swizzle_mode, byte_per_pixel: c_uint) -> c_uint;
}
extern "C" {
    pub fn dml2_core_utils_get_segment_horizontal_contiguous(sw_mode: dml2_swizzle_mode, byte_per_pixel: c_uint) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_vertical_rotation(Scan: dml2_rotation_angle) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_linear(sw_mode: dml2_swizzle_mode) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_get_gfx_version(sw_mode: dml2_swizzle_mode) -> int unsigned;
}
extern "C" {
    pub fn dml2_core_utils_get_qos_param_index(uclk_freq_khz: c_ulong, per_uclk_dpm_params: *const dml2_dcn4_uclk_dpm_dependent_qos_params) -> c_uint;
}
extern "C" {
    pub fn dml2_core_utils_get_active_min_uclk_dpm_index(uclk_freq_khz: c_ulong, clk_table: *const dml2_soc_state_table) -> c_uint;
}
extern "C" {
    pub fn dml2_core_utils_is_dual_plane(source_format: dml2_source_format_class) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_log_and_substract_if_non_zero(a: c_uint, subtrahend: c_uint) -> c_uint;
}
extern "C" {
    pub fn dml2_core_utils_is_stream_encoder_required(stream_descriptor: *const dml2_stream_parameters) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_encoder_dsc_capable(stream_descriptor: *const dml2_stream_parameters) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_dp_encoder(stream_descriptor: *const dml2_stream_parameters) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_dio_dp_encoder(stream_descriptor: *const dml2_stream_parameters) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_hpo_dp_encoder(stream_descriptor: *const dml2_stream_parameters) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_dp_8b_10b_link_rate(rate: dml2_output_link_dp_rate) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_dp_128b_132b_link_rate(rate: dml2_output_link_dp_rate) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_is_odm_split(odm_mode: dml2_odm_mode) -> bool;
}
extern "C" {
    pub fn dml2_core_utils_get_frame_time_us(stream: *const dml2_stream_parameters) -> double;
}
