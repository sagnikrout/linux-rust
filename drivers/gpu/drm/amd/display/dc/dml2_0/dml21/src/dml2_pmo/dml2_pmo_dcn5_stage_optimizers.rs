//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_pmo/dml2_pmo_dcn5_stage_optimizers.h
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
// Copyright 2024-2025 Advanced Micro Devices, Inc.

extern "C" {
    pub fn set_bit_in_bitfield(bit_field: *mut c_uint, bit_offset: c_uint);
}
extern "C" {
    pub fn is_bit_set_in_bitfield(bit_field: c_uint, bit_offset: c_uint) -> bool;
}
extern "C" {
    pub fn dcn5_get_vactive_pstate_margin(validation_res: *const dml2_validation_result, plane_mask: c_int) -> c_int;
}
// Output
// Input
extern "C" {
    pub fn dcn5_insert_into_candidate_list(pstate_strategy: *const dml2_pmo_pstate_strategy, stream_count: c_int, worksheet: *mut dml2_optimization_worksheet);
}
extern "C" {
    pub fn dcn5_reset_worksheet_for_uclk_pstate(worksheet: *mut dml2_optimization_worksheet);
}
extern "C" {
    pub fn dcn5_setup_planes_for_vactive_by_mask(stage: *mut dml2_pmo_stage_optimizer, worksheet: *mut dml2_optimization_worksheet, plane_mask: c_int);
}
extern "C" {
    pub fn dcn5_setup_planes_for_vblank_by_mask(stage: *mut dml2_pmo_stage_optimizer, worksheet: *mut dml2_optimization_worksheet, plane_mask: c_int);
}
extern "C" {
    pub fn dcn5_get_vactive_det_fill_latency_delay_us(validation_res: *const dml2_validation_result, plane_mask: c_int) -> c_int;
}
extern "C" {
    pub fn dcn5_get_minimum_reserved_time_us_for_planes(worksheet: *const dml2_optimization_worksheet, plane_mask: c_int) -> c_int;
}
// Public DCN5 PMO optimizers
