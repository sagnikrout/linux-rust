//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_config_types.h
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

//
// struct xe_gt_sriov_config - GT level per-VF configuration data.
//
// Used by the PF driver to maintain per-VF provisioning data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_config {
// @ggtt_region: GGTT region assigned to the VF.
    pub ggtt_region: *mut xe_ggtt_node,
// @lmem_obj: LMEM allocation for use by the VF.
    pub lmem_obj: *mut xe_bo,
// @num_ctxs: number of GuC contexts IDs.
    pub num_ctxs: u16,
// @begin_ctx: start index of GuC context ID range.
    pub begin_ctx: u16,
// @num_dbs: number of GuC doorbells IDs.
    pub num_dbs: u16,
// @begin_db: start index of GuC doorbell ID range.
    pub begin_db: u16,
// @exec_quantum: execution-quantum in milliseconds.
    pub exec_quantum: [u32; GUC_MAX_SCHED_GROUPS],
// @preempt_timeout: preemption timeout in microseconds.
    pub preempt_timeout: [u32; GUC_MAX_SCHED_GROUPS],
// @sched_priority: scheduling priority.
    pub sched_priority: u32,
// @thresholds: GuC thresholds for adverse events notifications.
    pub thresholds: [u32; XE_GUC_KLV_NUM_THRESHOLDS],
}

//
// struct xe_gt_sriov_spare_config - GT-level PF spare configuration data.
//
// Used by the PF driver to maintain it's own reserved (spare) provisioning
// data that is not applicable to be tracked in struct xe_gt_sriov_config.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_spare_config {
// @ggtt_size: GGTT size.
    pub ggtt_size: u64,
// @lmem_size: LMEM size.
    pub lmem_size: u64,
// @num_ctxs: number of GuC submission contexts.
    pub num_ctxs: u16,
// @num_dbs: number of GuC doorbells.
    pub num_dbs: u16,
}
