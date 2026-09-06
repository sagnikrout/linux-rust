//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_standalone_libraries/alternate_pstate_shared_lib.h
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
// SPDX-License-Identifier: MIT
//
// Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.
//

pub const END_SWATH_REC: c_uint = 0xFFFF;
pub const END_SWATH_PRE: c_uint = 0xFFFFF;
pub const NEXT_FRAME_MASK: c_uint = 0x80000000;
pub const SWATH_MASK: c_uint = 0x7FFFFFFF;
pub const MAX_FRAME_COUNT: c_uint = 0xFFFFFF;
pub const PROGRAM_GO_IMMEDIATE: c_uint = 0xFFFFFFFF;
pub const MAX_SUBVP_HEIGHT: c_uint = 0xFFF;
pub const MAX_SUBVP_START_LINE: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_swath_deadlines_params {
// inputs
    pub base: *mut dmub_fams2_cmd_stream_static_base_state,
    pub alternate_static_state: *mut dmub_fams2_cmd_alternate_stream_static_state,
    pub plane_index: u8,
    pub vtotal: u16,
    pub rec_y_start: u16,
    pub chroma_plane: bool,
// outputs
    pub output: *mut *mut uint16_t swath_array; // caller must allocate it's own memory for the,
    pub array_size: *mut u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct calculate_hubp_start_end_lines_params {
// inputs
    pub base: *mut dmub_fams2_cmd_stream_static_base_state,
    pub alternate_static_state: *mut dmub_fams2_cmd_alternate_stream_static_state,
    pub line]: uint32_t current_otg_line; // [dst,
    pub count: uint32_t current_frame_count; // reference frame,
    pub line]: uint32_t otg_pstate_target; // [dst,
    pub allow: uint32_t target_frame_count; // target frame count that we expect to assert P-State,
    pub vtotal: u16,
    pub rec_y_start: u16,
    pub plane_index: u8,
    pub cursor_size: u8,
    pub chroma_plane: bool,
// outputs
    pub svp0_start_line: u16,
    pub svp0_height: u16,
    pub svp0_height_next: u16,
    pub svp1_start_line: u16,
    pub svp1_height: u16,
    pub svp1_height_next: u16,
    pub svp_position: u8,
    pub program_go_line: u32,
    pub program_go_frame_count: u32,
// for debug
    pub svp0_start_dst_line: u16,
    pub svp0_end_dst_line: u16,
    pub svp1_start_dst_line: u16,
    pub svp1_end_dst_line: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct calculate_copy_from_primary_params {
// inputs
    pub target_frame: u32,
    pub flip_pending: u32,
    pub flip_pending_clear_frame: u32,
// outputs
    pub copy_from_primary: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svp_params {
    pub start_line: u16,
    pub height: u16,
    pub height_next: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct calculate_lsdma_copy_params {
// inputs
    pub base: *mut dmub_fams2_cmd_stream_static_base_state,
    pub alternate_static_state: *mut dmub_fams2_cmd_alternate_stream_static_state,
    pub plane_index: u8,
    pub svp1: svp_params svp[2]; // array of 2 for svp0 and,
    pub svp1: svp_params svp_c[2]; // array of 2 for svp0 and,
// outputs
    pub svp1: lsdma_outputs out[2]; // array of 2 for svp0 and,
    pub svp1: lsdma_outputs out_c[2]; // array of 2 for svp0 and,
}

extern "C" {
    pub fn calculate_lsdma_copy(p: *mut calculate_lsdma_copy_params);
}
extern "C" {
    pub fn calculate_copy_from_primary(p: *mut calculate_copy_from_primary_params);
}
extern "C" {
    pub fn get_swath_deadlines(p: *mut get_swath_deadlines_params);
}
extern "C" {
    pub fn calculate_hubp_start_end_lines(p: *mut calculate_hubp_start_end_lines_params);
}
extern "C" {
    pub fn get_prefetch_start_line_x1000(vtotal: u32, vblank_end: u16, recout_y: u16, dst_y_prefetch_x1000: u16, prefetch_relative_vblank: u8, dst_y_after_scaler: u16) -> i32;
}
extern "C" {
    pub fn get_prefetch_end_line(vtotal: u32, vblank_end: u16, recout_y: u16, prefetch_relative_vblank: u8, dst_y_after_scaler: u16) -> i32;
}
extern "C" {
    pub fn get_effective_vblank_start(vblank_start: u16, vblank_end: u16, recout_y: u16, recout_height: u16) -> u16;
}
extern "C" {
    pub fn in_circular_range(start: u32, end: u32, value: u32) -> bool;
}
