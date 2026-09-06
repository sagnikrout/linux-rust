//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_config.h
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
    pub fn xe_gt_sriov_pf_config_get_ggtt(gt: *mut xe_gt, vfid: c_uint) -> u64;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_ggtt(gt: *mut xe_gt, vfid: c_uint, size: u64) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_ctxs(gt: *mut xe_gt, vfid: c_uint) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_ctxs(gt: *mut xe_gt, vfid: c_uint, num_ctxs: u32) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_fair_ctxs(gt: *mut xe_gt, vfid: c_uint, num_vfs: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_dbs(gt: *mut xe_gt, vfid: c_uint) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_dbs(gt: *mut xe_gt, vfid: c_uint, num_dbs: u32) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_fair_dbs(gt: *mut xe_gt, vfid: c_uint, num_vfs: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_lmem(gt: *mut xe_gt, vfid: c_uint) -> u64;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_lmem(gt: *mut xe_gt, vfid: c_uint, size: u64) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_fair_lmem(gt: *mut xe_gt, vfid: c_uint, num_vfs: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_lmem_locked(gt: *mut xe_gt, vfid: c_uint) -> u64;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_lmem_locked(gt: *mut xe_gt, vfid: c_uint, size: u64) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_exec_quantum(gt: *mut xe_gt, vfid: c_uint) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_exec_quantum(gt: *mut xe_gt, vfid: c_uint, exec_quantum: u32) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_exec_quantum_locked(gt: *mut xe_gt, vfid: c_uint) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_bulk_set_exec_quantum_locked(gt: *mut xe_gt, exec_quantum: u32) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_preempt_timeout(gt: *mut xe_gt, vfid: c_uint) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_preempt_timeout_locked(gt: *mut xe_gt, vfid: c_uint) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_bulk_set_preempt_timeout_locked(gt: *mut xe_gt, preempt_timeout: u32) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_get_sched_priority(gt: *mut xe_gt, vfid: c_uint) -> u32;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_sched_priority(gt: *mut xe_gt, vfid: c_uint, priority: u32) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_force_sched_priority_locked(gt: *mut xe_gt, priority: u32);
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_fair_sched(gt: *mut xe_gt, num_vfs: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_set_fair(gt: *mut xe_gt, vfid: c_uint, num_vfs: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_sanitize(gt: *mut xe_gt, vfid: c_uint, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_release(gt: *mut xe_gt, vfid: c_uint, force: bool) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_push(gt: *mut xe_gt, vfid: c_uint, refresh: bool) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_save(gt: *mut xe_gt, vfid: c_uint, buf: *mut c_void, size: usize) -> isize;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_is_empty(gt: *mut xe_gt, vfid: c_uint) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_init(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_restart(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_print_ggtt(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_print_ctxs(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_print_dbs(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_print_lmem(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_config_print_available_ggtt(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
