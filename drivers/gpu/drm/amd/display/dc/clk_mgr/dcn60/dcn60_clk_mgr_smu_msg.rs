//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn60/dcn60_clk_mgr_smu_msg.h
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
// Copyright 2025 Advanced Micro Devices, Inc.

extern "C" {
    pub fn dcn60_smu_set_hard_min_by_freq(clk_mgr: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> c_uint;
}
extern "C" {
    pub fn dcn60_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, freq_mhz: u32);
}
extern "C" {
    pub fn dcn60_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn60_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, is_idle: bool);
}
