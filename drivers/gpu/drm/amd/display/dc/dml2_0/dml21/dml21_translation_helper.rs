//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/dml21_translation_helper.h
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
    pub fn dml21_populate_dml_init_params(dml_init: *mut dml2_initialize_instance_in_out, config: *const dml2_configuration_options, in_dc: *const dc);
}
extern "C" {
    pub fn dml21_map_dc_state_into_dml_display_cfg(in_dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context) -> bool;
}
extern "C" {
    pub fn dml21_copy_clocks_to_dc_state(in_ctx: *mut dml2_context, context: *mut dc_state);
}
extern "C" {
    pub fn dml21_extract_watermark_sets(in_dc: *const dc, watermarks: *mut dcn_watermark_set, in_ctx: *mut dml2_context);
}
extern "C" {
    pub fn dml21_map_hw_resources(dml_ctx: *mut dml2_context);
}
extern "C" {
    pub fn dml21_get_pipe_mcache_config(context: *mut dc_state, pipe_ctx: *mut pipe_ctx, pln_prog: *mut dml2_per_plane_programming, mcache_pipe_config: *mut dml2_pipe_configuration_descriptor);
}
extern "C" {
    pub fn dml21_set_dc_p_state_type(pipe_ctx: *mut pipe_ctx, stream_programming: *mut dml2_per_stream_programming, sub_vp_enabled: bool);
}
extern "C" {
    pub fn map_plane_to_dml21_display_cfg(dml_ctx: *const dml2_context, stream_id: c_uint, plane: *const dc_plane_state, context: *const dc_state) -> c_uint;
}
extern "C" {
    pub fn dml21_init_min_clocks_for_dc_state(in_ctx: *mut dml2_context, context: *mut dc_state);
}
