//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_control_types.h
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
// Copyright © 2024 Intel Corporation
//

//
// enum xe_gt_sriov_control_bits - Various bits used by the PF to represent a VF state
//
// @XE_GT_SRIOV_STATE_WIP: indicates that some operations are in progress.
// @XE_GT_SRIOV_STATE_FLR_WIP: indicates that a VF FLR is in progress.
// @XE_GT_SRIOV_STATE_FLR_PREPARE: indicates that the PF received early VF FLR prepare notification.
// @XE_GT_SRIOV_STATE_FLR_SEND_START: indicates that the PF wants to send a FLR START command.
// @XE_GT_SRIOV_STATE_FLR_WAIT_GUC: indicates that the PF awaits for a response from the GuC.
// @XE_GT_SRIOV_STATE_FLR_GUC_DONE: indicates that the PF has received a response from the GuC.
// @XE_GT_SRIOV_STATE_FLR_SYNC: indicates that the PF awaits to synchronize with other GuCs.
// @XE_GT_SRIOV_STATE_FLR_RESET_CONFIG: indicates that the PF needs to clear VF's resources.
// @XE_GT_SRIOV_STATE_FLR_RESET_DATA: indicates that the PF needs to clear VF's data.
// @XE_GT_SRIOV_STATE_FLR_RESET_MMIO: indicates that the PF needs to reset VF's registers.
// @XE_GT_SRIOV_STATE_FLR_SEND_FINISH: indicates that the PF wants to send a FLR FINISH message.
// @XE_GT_SRIOV_STATE_FLR_FAILED: indicates that VF FLR sequence failed.
// @XE_GT_SRIOV_STATE_PAUSE_WIP: indicates that a VF pause operation is in progress.
// @XE_GT_SRIOV_STATE_PAUSE_SEND_PAUSE: indicates that the PF is about to send a PAUSE command.
// @XE_GT_SRIOV_STATE_PAUSE_WAIT_GUC: indicates that the PF awaits for a response from the GuC.
// @XE_GT_SRIOV_STATE_PAUSE_GUC_DONE: indicates that the PF has received a response from the GuC.
// @XE_GT_SRIOV_STATE_PAUSE_FAILED: indicates that a VF pause operation has failed.
// @XE_GT_SRIOV_STATE_PAUSED: indicates that the VF is paused.
// @XE_GT_SRIOV_STATE_SAVE_WIP: indicates that VF save operation is in progress.
// @XE_GT_SRIOV_STATE_SAVE_PROCESS_DATA: indicates that VF migration data is being produced.
// @XE_GT_SRIOV_STATE_SAVE_WAIT_DATA: indicates that PF awaits for space in migration data ring.
// @XE_GT_SRIOV_STATE_SAVE_DATA_DONE: indicates that all migration data was produced by Xe.
// @XE_GT_SRIOV_STATE_SAVE_FAILED: indicates that VF save operation has failed.
// @XE_GT_SRIOV_STATE_SAVED: indicates that VF data is saved.
// @XE_GT_SRIOV_STATE_RESTORE_WIP: indicates that VF restore operation is in progress.
// @XE_GT_SRIOV_STATE_RESTORE_PROCESS_DATA: indicates that VF migration data is being consumed.
// @XE_GT_SRIOV_STATE_RESTORE_WAIT_DATA: indicates that PF awaits for data in migration data ring.
// @XE_GT_SRIOV_STATE_RESTORE_DATA_DONE: indicates that all migration data was produced by the user.
// @XE_GT_SRIOV_STATE_RESTORE_FAILED: indicates that VF restore operation has failed.
// @XE_GT_SRIOV_STATE_RESTORED: indicates that VF data is restored.
// @XE_GT_SRIOV_STATE_RESUME_WIP: indicates the a VF resume operation is in progress.
// @XE_GT_SRIOV_STATE_RESUME_SEND_RESUME: indicates that the PF is about to send RESUME command.
// @XE_GT_SRIOV_STATE_RESUME_FAILED: indicates that a VF resume operation has failed.
// @XE_GT_SRIOV_STATE_RESUMED: indicates that the VF was resumed.
// @XE_GT_SRIOV_STATE_STOP_WIP: indicates that a VF stop operation is in progress.
// @XE_GT_SRIOV_STATE_STOP_SEND_STOP: indicates that the PF wants to send a STOP command.
// @XE_GT_SRIOV_STATE_STOP_FAILED: indicates that the VF stop operation has failed
// @XE_GT_SRIOV_STATE_STOPPED: indicates that the VF was stopped.
// @XE_GT_SRIOV_STATE_MISMATCH: indicates that the PF has detected a VF state mismatch.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_gt_sriov_control_bits {
    XE_GT_SRIOV_STATE_WIP = 1,

    XE_GT_SRIOV_STATE_FLR_WIP,
    XE_GT_SRIOV_STATE_FLR_PREPARE,
    XE_GT_SRIOV_STATE_FLR_SEND_START,
    XE_GT_SRIOV_STATE_FLR_WAIT_GUC,
    XE_GT_SRIOV_STATE_FLR_GUC_DONE,
    XE_GT_SRIOV_STATE_FLR_SYNC,
    XE_GT_SRIOV_STATE_FLR_RESET_CONFIG,
    XE_GT_SRIOV_STATE_FLR_RESET_DATA,
    XE_GT_SRIOV_STATE_FLR_RESET_MMIO,
    XE_GT_SRIOV_STATE_FLR_SEND_FINISH,
    XE_GT_SRIOV_STATE_FLR_FAILED,

    XE_GT_SRIOV_STATE_PAUSE_WIP,
    XE_GT_SRIOV_STATE_PAUSE_SEND_PAUSE,
    XE_GT_SRIOV_STATE_PAUSE_WAIT_GUC,
    XE_GT_SRIOV_STATE_PAUSE_GUC_DONE,
    XE_GT_SRIOV_STATE_PAUSE_FAILED,
    XE_GT_SRIOV_STATE_PAUSED,

    XE_GT_SRIOV_STATE_SAVE_WIP,
    XE_GT_SRIOV_STATE_SAVE_PROCESS_DATA,
    XE_GT_SRIOV_STATE_SAVE_WAIT_DATA,
    XE_GT_SRIOV_STATE_SAVE_DATA_DONE,
    XE_GT_SRIOV_STATE_SAVE_FAILED,
    XE_GT_SRIOV_STATE_SAVED,

    XE_GT_SRIOV_STATE_RESTORE_WIP,
    XE_GT_SRIOV_STATE_RESTORE_PROCESS_DATA,
    XE_GT_SRIOV_STATE_RESTORE_WAIT_DATA,
    XE_GT_SRIOV_STATE_RESTORE_DATA_DONE,
    XE_GT_SRIOV_STATE_RESTORE_FAILED,
    XE_GT_SRIOV_STATE_RESTORED,

    XE_GT_SRIOV_STATE_RESUME_WIP,
    XE_GT_SRIOV_STATE_RESUME_SEND_RESUME,
    XE_GT_SRIOV_STATE_RESUME_FAILED,
    XE_GT_SRIOV_STATE_RESUMED,

    XE_GT_SRIOV_STATE_STOP_WIP,
    XE_GT_SRIOV_STATE_STOP_SEND_STOP,
    XE_GT_SRIOV_STATE_STOP_FAILED,
    XE_GT_SRIOV_STATE_STOPPED,

    XE_GT_SRIOV_STATE_MISMATCH, /* always keep as last */
}

//
// struct xe_gt_sriov_control_state - GT-level per-VF control state.
//
// Used by the PF driver to maintain per-VF control data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_control_state {
// @state: VF state bits
    pub XE_GT_SRIOV_NUM_STATES): DECLARE_BITMAP(state,,
// @done: completion of async operations
    pub done: completion,
// @link: link into worker list
    pub link: list_head,
}

//
// struct xe_gt_sriov_pf_control - GT-level control data.
//
// Used by the PF driver to maintain its data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_pf_control {
// @worker: worker that executes a VF operations
    pub worker: work_struct,
// @list: list of VF entries that have a pending work
    pub list: list_head,
// @lock: protects VF pending list
    pub lock: spinlock_t,
}
