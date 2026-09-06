//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_control.h
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
// Copyright © 2023-2024 Intel Corporation
//

extern "C" {
    pub fn xe_gt_sriov_pf_control_init(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_restart(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_pause_vf(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_resume_vf(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_check_save_data_done(gt: *mut xe_gt, vfid: c_uint) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_check_save_failed(gt: *mut xe_gt, vfid: c_uint) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_process_save_data(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_trigger_save_vf(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_finish_save_vf(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_restore_data_done(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_check_restore_failed(gt: *mut xe_gt, vfid: c_uint) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_process_restore_data(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_trigger_restore_vf(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_finish_restore_vf(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_stop_vf(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_prepare_flr(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_trigger_flr(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_sync_flr(gt: *mut xe_gt, vfid: c_uint, sync: bool) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_control_wait_flr(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}

extern "C" {
    pub fn xe_gt_sriov_pf_control_process_guc2pf(gt: *mut xe_gt, msg: *const u32, len: u32) -> c_int;
}

