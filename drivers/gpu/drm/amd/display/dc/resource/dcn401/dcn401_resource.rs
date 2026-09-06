//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/resource/dcn401/dcn401_resource.h
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

// Macro flag: #define TO_DCN401_RES_POOL(pool)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_resource_pool {
    pub base: resource_pool,
}

extern "C" {
    pub fn dcn401_patch_unknown_plane_state(plane_state: *mut dc_plane_state) -> dc_status;
}
extern "C" {
    pub fn dcn401_prepare_mcache_programming(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn401_get_default_tiling_info(tiling_info: *mut dc_tiling_info);
}
extern "C" {
    pub fn dcn401_get_vstartup_for_pipe(pipe_ctx: *mut pipe_ctx) -> c_uint;
}
extern "C" {
    pub fn dcn401_get_power_profile(context: *const dc_state) -> c_int;
}
// Following are definitions for run time init of reg offsets
// HUBP

// ABM

// VPG

// Stream encoder

// Link encoder

// DPP

// OPP

// DSC

// MPC

// OPTC

// HUBBUB

// DCCG

