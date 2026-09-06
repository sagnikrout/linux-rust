//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_exec_queue_types.h
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
// Copyright © 2022 Intel Corporation
//

//
// struct xe_guc_exec_queue - GuC specific state for an xe_exec_queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_exec_queue {
// @q: Backpointer to parent xe_exec_queue
    pub q: *mut xe_exec_queue,
// @rcu: For safe freeing of exported dma fences
    pub rcu: rcu_head,
// @sched: GPU scheduler for this xe_exec_queue
    pub sched: xe_gpu_scheduler,
//
// @name: Scheduler timeline name, kept with @sched until RCU free.
//
    pub name: [c_char; MAX_FENCE_NAME_LEN],
// @entity: Scheduler entity for this xe_exec_queue
    pub entity: xe_sched_entity,
//
// @static_msgs: Static messages for this xe_exec_queue, used when
// a message needs to sent through the GPU scheduler but memory
// allocations are not allowed.
//
pub const MAX_STATIC_MSG_TYPE: c_int = 3;
    pub static_msgs: [xe_sched_msg; MAX_STATIC_MSG_TYPE],
// @destroy_async: do final destroy async from this worker
    pub destroy_async: work_struct,
// @resume_time: time of last resume
    pub resume_time: u64,
// @state: GuC specific state for this xe_exec_queue
    pub state: core::sync::atomic::AtomicI32,
// @wqi_head: work queue item tail
    pub wqi_head: u32,
// @wqi_tail: work queue item tail
    pub wqi_tail: u32,
// @id: GuC id for this exec_queue
    pub id: u16,
// @suspend_wait: wait queue used to wait on pending suspends
    pub suspend_wait: wait_queue_head_t,
// @suspend_pending: a suspend of the exec_queue is pending
    pub suspend_pending: bool,
//
// @suspend_count: Reference count of active suspend requests. The
// exec_queue remains suspended while this is non-zero, allowing
// multiple concurrent callers to independently hold a suspend without
// prematurely re-enabling the queue. Protected by @sched.msg_lock.
//
    pub suspend_count: c_int,
//
// @needs_cleanup: Needs a cleanup message during VF post migration
// recovery.
//
    pub needs_cleanup: bool,
//
// @needs_suspend: Needs a suspend message during VF post migration
// recovery.
//
    pub needs_suspend: bool,
//
// @needs_resume: Needs a resume message during VF post migration
// recovery.
//
    pub needs_resume: bool,
}
