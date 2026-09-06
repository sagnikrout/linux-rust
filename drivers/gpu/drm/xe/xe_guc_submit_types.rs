//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_submit_types.h
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

// Work item for submitting workloads into work queue of GuC.
pub const WQ_STATUS_ACTIVE: c_int = 1;
pub const WQ_STATUS_SUSPENDED: c_int = 2;
pub const WQ_STATUS_CMD_ERROR: c_int = 3;
pub const WQ_STATUS_ENGINE_ID_NOT_USED: c_int = 4;
pub const WQ_STATUS_SUSPENDED_FROM_RESET: c_int = 5;
pub const WQ_TYPE_NOOP: c_uint = 0x4;
pub const WQ_TYPE_MULTI_LRC: c_uint = 0x5;

pub const PARALLEL_SCRATCH_SIZE: c_int = 2048;

pub const CACHELINE_BYTES: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_sched_wq_desc {
    pub head: u32,
    pub tail: u32,
    pub error_offset: u32,
    pub wq_status: u32,
    pub reserved: [u32; 28],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_semaphore {
    pub semaphore: u32,
    pub sizeof(u32)]: u8 unused[CACHELINE_BYTES -,
}

//
// struct guc_submit_parallel_scratch - A scratch shared mapped buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_submit_parallel_scratch {
// @wq_desc: Guc scheduler workqueue descriptor
    pub wq_desc: guc_sched_wq_desc,
// @go: Go Semaphore
    pub go: sync_semaphore,
// @join: Joined semaphore for the relevant hw engine instances
    pub join: [sync_semaphore; XE_HW_ENGINE_MAX_INSTANCE],
// @unused: Unused/Reserved memory space
    pub 1)]: (XE_HW_ENGINE_MAX_INSTANCE +,
// @wq: Workqueue info
    pub sizeof(u32)]: u32 wq[WQ_SIZE /,
}

//
// struct xe_guc_submit_exec_queue_snapshot - Snapshot for devcoredump
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_submit_exec_queue_snapshot {
// @name: name of this exec queue
    pub name: [c_char; MAX_FENCE_NAME_LEN],
// @class: class of this exec queue
    pub class: xe_engine_class,
//
// @logical_mask: logical mask of where job submitted to exec queue can run
//
    pub logical_mask: u32,
// @width: width (number BB submitted per exec) of this exec queue
    pub width: u16,
// @refcount: ref count of this exec queue
    pub refcount: u32,
//
// @sched_timeout: the time after which a job is removed from the
// scheduler.
//
    pub sched_timeout: c_long,
// @sched_props: scheduling properties
// @sched_props.timeslice_us: timeslice period in micro-seconds
    pub timeslice_us: u32,
// @sched_props.preempt_timeout_us: preemption timeout in micro-seconds
    pub preempt_timeout_us: u32,
    pub sched_props: },
// @lrc: LRC Snapshot
    pub lrc: *mut xe_lrc_snapshot,
// @schedule_state: Schedule State at the moment of Crash
    pub schedule_state: u32,
// @exec_queue_flags: Flags of the faulty exec_queue
    pub exec_queue_flags: c_ulong,
// @guc: GuC Engine Snapshot
// @guc.wqi_head: work queue item head
    pub wqi_head: u32,
// @guc.wqi_tail: work queue item tail
    pub wqi_tail: u32,
// @guc.id: GuC id for this exec_queue
    pub id: u16,
    pub guc: },
//
// @parallel_execution: Indication if the failure was during parallel
// execution
//
    pub parallel_execution: bool,
// @parallel: snapshot of the useful parallel scratch
// @parallel.wq_desc: Workqueue description
// @parallel.wq_desc.head: Workqueue Head
    pub head: u32,
// @parallel.wq_desc.tail: Workqueue Tail
    pub tail: u32,
// @parallel.wq_desc.status: Workqueue Status
    pub status: u32,
    pub wq_desc: },
// @wq: Workqueue Items
    pub sizeof(u32)]: u32 wq[WQ_SIZE /,
    pub parallel: },
// @multi_queue: snapshot of the multi queue information
//
// @multi_queue.primary: GuC id of the primary exec queue
// of the multi queue group.
//
    pub primary: u32,
// @multi_queue.pos: Position of the exec queue within the multi queue group
    pub pos: u8,
// @multi_queue.valid: The exec queue is part of a multi queue group
    pub valid: bool,
    pub multi_queue: },
}
