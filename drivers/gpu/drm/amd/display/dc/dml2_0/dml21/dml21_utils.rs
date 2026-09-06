//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/dml21_utils.h
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
    pub fn dml21_helper_find_dml_pipe_idx_by_stream_id(ctx: *mut dml2_context, stream_id: c_uint) -> c_int;
}
extern "C" {
    pub fn dml21_find_dml_pipe_idx_by_plane_id(ctx: *mut dml2_context, plane_id: c_uint) -> c_int;
}
extern "C" {
    pub fn dml21_get_plane_id(state: *const dc_state, plane: *const dc_plane_state, plane_id: *mut c_uint) -> bool;
}
extern "C" {
    pub fn check_dp2p0_output_encoder(pipe_ctx: *const pipe_ctx) -> bool;
}
extern "C" {
    pub fn find_valid_pipe_idx_for_stream_index(dml_ctx: *const dml2_context, dml_pipe_idx: *mut c_uint, stream_index: c_uint);
}
extern "C" {
    pub fn dml21_handle_phantom_streams_planes(in_dc: *const dc, context: *mut dc_state, dml_ctx: *mut dml2_context);
}
extern "C" {
    pub fn dml21_get_dc_plane_idx_from_plane_id(plane_id: c_uint) -> c_uint;
}
extern "C" {
    pub fn dml21_is_plane1_enabled(source_format: dml2_source_format_class) -> bool;
}
