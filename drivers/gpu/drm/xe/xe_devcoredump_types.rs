//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_devcoredump_types.h
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
// Copyright © 2023 Intel Corporation
//

//
// struct xe_devcoredump_snapshot - Crash snapshot
//
// This struct contains all the useful information quickly captured at the time
// of the crash. So, any subsequent reads of the coredump points to a data that
// shows the state of the GPU of when the issue has happened.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_devcoredump_snapshot {
// @snapshot_time:  Time of this capture.
    pub snapshot_time: ktime_t,
// @boot_time:  Relative boot time so the uptime can be calculated.
    pub boot_time: ktime_t,
// @process_name: Name of process that triggered this gpu hang
    pub process_name: [c_char; TASK_COMM_LEN],
// @pid: Process id of process that triggered this gpu hang
    pub pid: pid_t,
// @reason: The reason the coredump was triggered
    pub reason: *mut c_char,
// @gt: Affected GT, used by forcewake for delayed capture
    pub gt: *mut xe_gt,
// @work: Workqueue for deferred capture outside of signaling context
    pub work: work_struct,
// @guc: GuC snapshots
// @guc.ct: GuC CT snapshot
    pub ct: *mut xe_guc_ct_snapshot,
// @guc.log: GuC log snapshot
    pub log: *mut xe_guc_log_snapshot,
    pub guc: },
// @ge: GuC Submission Engine snapshot
    pub ge: *mut xe_guc_submit_exec_queue_snapshot,
// @hwe: HW Engine snapshot array
    pub hwe: [*mut xe_hw_engine_snapshot; XE_NUM_HW_ENGINES],
// @job: Snapshot of job state
    pub job: *mut xe_sched_job_snapshot,
//
// @matched_node: The matched capture node for timedout job
// this single-node tracker works because devcoredump will always only
// produce one hw-engine capture per devcoredump event
//
    pub matched_node: *mut __guc_capture_parsed_output,
// @vm: Snapshot of VM state
    pub vm: *mut xe_vm_snapshot,
// @read: devcoredump in human readable format
// @read.size: size of devcoredump in human readable format
    pub size: isize,
// @read.chunk_position: position of devcoredump chunk
    pub chunk_position: isize,
// @read.buffer: buffer of devcoredump in human readable format
    pub buffer: *mut c_char,
    pub read: },
}

//
// struct xe_devcoredump - Xe devcoredump main structure
//
// This struct represents the live and active dev_coredump node.
// It is created/populated at the time of a crash/error. Then it
// is read later when user access the device coredump data file
// for reading the information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_devcoredump {
// @lock: protects access to entire structure
    pub lock: mutex,
// @captured: The snapshot of the first hang has already been taken
    pub captured: bool,
// @snapshot: Snapshot is captured at time of the first crash
    pub snapshot: xe_devcoredump_snapshot,
}
