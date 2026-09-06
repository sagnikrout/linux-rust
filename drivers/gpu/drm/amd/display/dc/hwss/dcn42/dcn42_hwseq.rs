//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hwss/dcn42/dcn42_hwseq.h
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
// Copyright 2026 Advanced Micro Devices, Inc.

extern "C" {
    pub fn dcn42_init_hw(dc: *mut dc);
}
extern "C" {
    pub fn dcn42_update_mpcc(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn42_hardware_release(dc: *mut dc);
}
extern "C" {
    pub fn dcn42_optimize_bandwidth(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn42_dmub_hw_control_lock(dc: *mut dc, context: *mut dc_state, lock: bool);
}
extern "C" {
    pub fn dcn42_dmub_hw_control_lock_fast(params: *mut block_sequence_params);
}
extern "C" {
    pub fn dcn42_setup_stereo(pipe_ctx: *mut pipe_ctx, dc: *mut dc);
}
extern "C" {
    pub fn dcn42_power_down_on_boot(dc: *mut dc);
}
