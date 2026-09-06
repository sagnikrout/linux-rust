//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pxp_types.h
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
// Copyright(c) 2024, Intel Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_pxp_status {
    XE_PXP_ERROR = -1,
    XE_PXP_NEEDS_TERMINATION = 0, /* starting status */
    XE_PXP_NEEDS_ADDITIONAL_TERMINATION,
    XE_PXP_TERMINATION_IN_PROGRESS,
    XE_PXP_READY_TO_START,
    XE_PXP_START_IN_PROGRESS,
    XE_PXP_ACTIVE,
    XE_PXP_SUSPENDED,
}

//
// struct xe_pxp_gsc_client_resources - resources for GSC submission by a PXP
// client. The GSC FW supports multiple GSC client active at the same time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pxp_gsc_client_resources {
//
// @host_session_handle: handle used to identify the client in messages
// sent to the GSC firmware.
//
    pub host_session_handle: u64,
// @vm: VM used for PXP submissions to the GSCCS
    pub vm: *mut xe_vm,
// @q: GSCCS exec queue for PXP submissions
    pub q: *mut xe_exec_queue,
//
// @bo: BO used for submissions to the GSCCS and GSC FW. It includes
// space for the GSCCS batch and the input/output buffers read/written
// by the FW
//
    pub bo: *mut xe_bo,
// @inout_size: size of each of the msg_in/out sections individually
    pub inout_size: u32,
// @batch: iosys_map to the batch memory within the BO
    pub batch: iosys_map,
// @msg_in: iosys_map to the input memory within the BO
    pub msg_in: iosys_map,
// @msg_out: iosys_map to the output memory within the BO
    pub msg_out: iosys_map,
}

//
// struct xe_pxp - pxp state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pxp {
// @xe: Backpoiner to the xe_device struct
    pub xe: *mut xe_device,
//
// @gt: pointer to the gt that owns the submission-side of PXP
// (VDBOX, KCR and GSC)
//
    pub gt: *mut xe_gt,
// @vcs_exec: kernel-owned objects for PXP submissions to the VCS
// @vcs_exec.q: kernel-owned VCS exec queue used for PXP terminations
    pub q: *mut xe_exec_queue,
// @vcs_exec.bo: BO used for submissions to the VCS
    pub bo: *mut xe_bo,
    pub vcs_exec: },
// @gsc_res: kernel-owned objects for PXP submissions to the GSCCS
    pub gsc_res: xe_pxp_gsc_client_resources,
// @events: wrapper for the worker and queue used for PXP event handling
// @events.work: worker that manages termination events.
    pub work: work_struct,
// @events.wq: workqueue on which to queue the work.
    pub wq: *mut workqueue_struct,
// @events.pending: pending events
    pub pending: core::sync::atomic::AtomicI32,

    pub events: },
// @mutex: protects the pxp status and the queue list
    pub mutex: mutex,
// @status: the current pxp status
    pub status: xe_pxp_status,
// @activation: completion struct that tracks pxp start
    pub activation: completion,
// @termination: completion struct that tracks terminations
    pub termination: completion,
// @queues: management of exec_queues that use PXP
// @queues.lock: spinlock protecting the queue management
    pub lock: spinlock_t,
// @queues.list: list of exec_queues that use PXP
    pub list: list_head,
    pub queues: },
//
// @key_instance: keep track of the current iteration of the PXP key.
// Note that, due to the time needed for PXP termination and re-start
// to complete, the minimum time between 2 subsequent increases of this
// variable is 50ms, and even that only if there is a continuous attack;
// normal behavior is for this to increase much much slower than that.
// This means that we don't expect this to ever wrap and don't implement
// that case in the code.
//
    pub key_instance: u32,
//
// @last_suspend_key_instance: value of key_instance at the last
// suspend. Used to check if any PXP session has been created between
// suspend cycles.
//
    pub last_suspend_key_instance: u32,
//
// @needs_termination_on_resume: indicates if PXP termination is needed
// on resume. This is set if PXP was active when we suspend and it is
// cleared when we queue the termination on resume. Since the suspend
// and resume calls cannot execute at the same time, this variable does
// not need to be protected by the PXP lock.
//
    pub needs_termination_on_resume: bool,
}
