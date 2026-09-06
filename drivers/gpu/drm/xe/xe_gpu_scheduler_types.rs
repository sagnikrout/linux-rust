//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gpu_scheduler_types.h
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
// struct xe_sched_msg - an in-band (relative to GPU scheduler run queue)
// message
//
// Generic enough for backend defined messages, backend can expand if needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sched_msg {
// @link: list link into the gpu scheduler list of messages
    pub link: list_head,
//
// @private_data: opaque pointer to message private data (backend defined)
//
    pub private_data: *mut c_void,
// @opcode: opcode of message (backend defined)
    pub opcode: c_uint,
}

//
// struct xe_sched_backend_ops - Define the backend operations called by the
// scheduler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sched_backend_ops {
//
// @process_msg: Process a message. Allowed to block, it is this
// function's responsibility to free message if dynamically allocated.
//
    pub msg): *mut *mut void (process_msg)(struct xe_sched_msg,
}

//
// struct xe_gpu_scheduler - Xe GPU scheduler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gpu_scheduler {
// @base: DRM GPU scheduler
    pub base: drm_gpu_scheduler,
// @ops: Xe scheduler ops
    pub ops: *const xe_sched_backend_ops,
// @msgs: list of messages to be processed in @work_process_msg
    pub msgs: list_head,
// @msg_lock: Message lock
    pub msg_lock: spinlock_t,
// @work_process_msg: processes messages
    pub work_process_msg: work_struct,
}

