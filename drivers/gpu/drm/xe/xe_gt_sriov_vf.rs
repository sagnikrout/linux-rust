//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_vf.h
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
    pub fn xe_gt_sriov_vf_reset(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_bootstrap(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_query_config(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_connect(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_query_runtime(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_migrated_event_handler(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_sriov_vf_init_early(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_init(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_recovery_pending(gt: *mut xe_gt) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_vf_gmdid(gt: *mut xe_gt) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_vf_guc_ids(gt: *mut xe_gt) -> u16;
}
extern "C" {
    pub fn xe_gt_sriov_vf_lmem(gt: *mut xe_gt) -> u64;
}
extern "C" {
    pub fn xe_gt_sriov_vf_sched_groups_enabled(gt: *mut xe_gt) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_vf_paging_engines(gt: *mut xe_gt) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_vf_read32(gt: *mut xe_gt, reg: xe_reg) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_vf_write32(gt: *mut xe_gt, reg: xe_reg, val: u32);
}
extern "C" {
    pub fn xe_gt_sriov_vf_print_config(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_print_runtime(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_print_version(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_vf_wait_valid_ggtt(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_vf_migration_fixups_complete_count(gt: *mut xe_gt) -> c_int;
}
