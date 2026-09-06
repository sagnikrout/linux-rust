//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_policy_types.h
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
// enum xe_sriov_sched_group_modes - list of possible scheduler group modes
// @XE_SRIOV_SCHED_GROUPS_DISABLED: no separate groups (i.e., all engines in group 0)
// @XE_SRIOV_SCHED_GROUPS_MEDIA_SLICES: separate groups for each media slice
// @XE_SRIOV_SCHED_GROUPS_MODES_COUNT: number of valid modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_sriov_sched_group_modes {
    XE_SRIOV_SCHED_GROUPS_DISABLED = 0,
    XE_SRIOV_SCHED_GROUPS_MEDIA_SLICES,
    XE_SRIOV_SCHED_GROUPS_MODES_COUNT /* must be last */
}

//
// struct xe_gt_sriov_scheduler_groups - Scheduler groups policy info
// @max_groups: max number of groups supported by the GuC for the platform
// @supported_modes: mask of supported modes
// @current_mode: active scheduler groups mode
// @modes: array of masks and their number for each mode
// @modes.groups: array of engine instance groups in given mode, with each group
// consisting of GUC_MAX_ENGINE_CLASSES engine instances masks. A
// A NULL value indicates that all the engines are in the same
// group for this mode on this GT.
// @modes.num_groups: number of groups in given mode, zero if all the engines
// are in the same group.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_scheduler_groups {
    pub max_groups: u8,
    pub supported_modes: u32,
    pub current_mode: xe_sriov_sched_group_modes,
    pub groups: *mut guc_sched_group,
    pub num_groups: u32,
    pub modes: [}; XE_SRIOV_SCHED_GROUPS_MODES_COUNT],
}

//
// struct xe_gt_sriov_guc_policies - GuC SR-IOV policies.
// @sched_if_idle: controls strict scheduling policy.
// @reset_engine: controls engines reset on VF switch policy.
// @sample_period: adverse events sampling period (in milliseconds).
// @sched_groups: available scheduling group configurations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_guc_policies {
    pub sched_if_idle: bool,
    pub reset_engine: bool,
    pub sample_period: u32,
    pub sched_groups: xe_gt_sriov_scheduler_groups,
}

//
// struct xe_gt_sriov_pf_policy - PF policy data.
// @guc: GuC scheduling policies.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_pf_policy {
    pub guc: xe_gt_sriov_guc_policies,
}
