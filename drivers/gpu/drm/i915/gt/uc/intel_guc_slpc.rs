//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_guc_slpc.h
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
// Copyright © 2021 Intel Corporation
//

pub const SLPC_MAX_FREQ_MHZ: c_int = 4250;
extern "C" {
    pub fn intel_guc_submission_is_used(intel_guc_slpc_is_wanted(guc: guc) &&) -> return;
}
extern "C" {
    pub fn intel_guc_slpc_init_early(slpc: *mut intel_guc_slpc);
}
extern "C" {
    pub fn intel_guc_slpc_init(slpc: *mut intel_guc_slpc) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_enable(slpc: *mut intel_guc_slpc) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_fini(slpc: *mut intel_guc_slpc);
}
extern "C" {
    pub fn intel_guc_slpc_set_max_freq(slpc: *mut intel_guc_slpc, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_set_min_freq(slpc: *mut intel_guc_slpc, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_set_boost_freq(slpc: *mut intel_guc_slpc, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_get_max_freq(slpc: *mut intel_guc_slpc, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_get_min_freq(slpc: *mut intel_guc_slpc, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_print_info(slpc: *mut intel_guc_slpc, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_set_media_ratio_mode(slpc: *mut intel_guc_slpc, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_pm_intrmsk_enable(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_guc_slpc_boost(slpc: *mut intel_guc_slpc);
}
extern "C" {
    pub fn intel_guc_slpc_dec_waiters(slpc: *mut intel_guc_slpc);
}
extern "C" {
    pub fn intel_guc_slpc_set_ignore_eff_freq(slpc: *mut intel_guc_slpc, val: bool) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_set_strategy(slpc: *mut intel_guc_slpc, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_slpc_set_power_profile(slpc: *mut intel_guc_slpc, val: u32) -> c_int;
}
