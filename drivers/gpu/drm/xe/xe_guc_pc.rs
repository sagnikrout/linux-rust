//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_pc.h
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
// Copyright © 2022 Intel Corporation
//

extern "C" {
    pub fn xe_guc_pc_init(pc: *mut xe_guc_pc) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_start(pc: *mut xe_guc_pc) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_stop(pc: *mut xe_guc_pc);
}
extern "C" {
    pub fn xe_guc_pc_print(pc: *mut xe_guc_pc, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_pc_action_set_param(pc: *mut xe_guc_pc, id: u8, value: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_action_unset_param(pc: *mut xe_guc_pc, id: u8) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_get_act_freq(pc: *mut xe_guc_pc) -> u32;
}
extern "C" {
    pub fn xe_guc_pc_get_cur_freq(pc: *mut xe_guc_pc, freq: *mut u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_get_cur_freq_fw(pc: *mut xe_guc_pc) -> u32;
}
extern "C" {
    pub fn xe_guc_pc_get_rp0_freq(pc: *mut xe_guc_pc) -> u32;
}
extern "C" {
    pub fn xe_guc_pc_get_rpa_freq(pc: *mut xe_guc_pc) -> u32;
}
extern "C" {
    pub fn xe_guc_pc_get_rpe_freq(pc: *mut xe_guc_pc) -> u32;
}
extern "C" {
    pub fn xe_guc_pc_get_rpn_freq(pc: *mut xe_guc_pc) -> u32;
}
extern "C" {
    pub fn xe_guc_pc_get_min_freq(pc: *mut xe_guc_pc, freq: *mut u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_set_min_freq(pc: *mut xe_guc_pc, freq: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_get_max_freq(pc: *mut xe_guc_pc, freq: *mut u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_set_max_freq(pc: *mut xe_guc_pc, freq: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_set_power_profile(pc: *mut xe_guc_pc, buf: *const c_char) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_get_power_profile(pc: *mut xe_guc_pc, profile: *mut c_char);
}
extern "C" {
    pub fn xe_guc_pc_c_status(pc: *mut xe_guc_pc) -> xe_gt_idle_state;
}
extern "C" {
    pub fn xe_guc_pc_rc6_residency(pc: *mut xe_guc_pc) -> u64;
}
extern "C" {
    pub fn xe_guc_pc_mc6_residency(pc: *mut xe_guc_pc) -> u64;
}
extern "C" {
    pub fn xe_guc_pc_init_early(pc: *mut xe_guc_pc);
}
extern "C" {
    pub fn xe_guc_pc_restore_stashed_freq(pc: *mut xe_guc_pc) -> c_int;
}
extern "C" {
    pub fn xe_guc_pc_raise_unslice(pc: *mut xe_guc_pc);
}
extern "C" {
    pub fn xe_guc_pc_apply_flush_freq_limit(pc: *mut xe_guc_pc);
}
extern "C" {
    pub fn xe_guc_pc_remove_flush_freq_limit(pc: *mut xe_guc_pc);
}
