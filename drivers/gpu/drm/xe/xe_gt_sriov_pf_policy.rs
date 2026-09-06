//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_policy.h
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
    pub fn xe_gt_sriov_pf_policy_set_sched_if_idle(gt: *mut xe_gt, enable: bool) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_set_sched_if_idle_locked(gt: *mut xe_gt, enable: bool) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_get_sched_if_idle(gt: *mut xe_gt) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_get_sched_if_idle_locked(gt: *mut xe_gt) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_set_reset_engine(gt: *mut xe_gt, enable: bool) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_get_reset_engine(gt: *mut xe_gt) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_set_sample_period(gt: *mut xe_gt, value: u32) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_get_sample_period(gt: *mut xe_gt) -> u32;
}
extern "C" {
    pub fn xe_sriov_gt_pf_policy_has_sched_groups_support(gt: *mut xe_gt) -> bool;
}
extern "C" {
    pub fn xe_sriov_gt_pf_policy_has_multi_group_modes(gt: *mut xe_gt) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_sched_groups_enabled(gt: *mut xe_gt) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_init(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_sanitize(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_restart(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_policy_print(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
