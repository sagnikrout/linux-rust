//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_exec_queue.h
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
// Copyright © 2021 Intel Corporation
//

extern "C" {
    pub fn xe_exec_queue_fini(q: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_exec_queue_destroy(ref: *mut kref);
}
extern "C" {
    pub fn xe_exec_queue_assign_name(q: *mut xe_exec_queue, instance: u32);
}
//
// xe_exec_queue_is_multi_queue() - Whether an exec_queue is part of a queue group.
// @q: The exec_queue
//
// Return: True if the exec_queue is part of a queue group, false otherwise.
//
// xe_exec_queue_is_multi_queue_primary() - Whether an exec_queue is primary queue
// of a multi queue group.
// @q: The exec_queue
//
// Return: True if @q is primary queue of a queue group, false otherwise.
//
// xe_exec_queue_is_multi_queue_secondary() - Whether an exec_queue is secondary queue
// of a multi queue group.
// @q: The exec_queue
//
// Return: True if @q is secondary queue of a queue group, false otherwise.
//
extern "C" {
    pub fn xe_exec_queue_is_multi_queue(!xe_exec_queue_is_multi_queue_primary(q: q) &&) -> return;
}
//
// xe_exec_queue_multi_queue_primary() - Get multi queue group's primary queue
// @q: The exec_queue
//
// If @q belongs to a multi queue group, then the primary queue of the group will
// be returned. Otherwise, @q will be returned.
//
extern "C" {
    pub fn xe_exec_queue_is_lr(q: *mut xe_exec_queue) -> bool;
}
extern "C" {
    pub fn xe_exec_queue_is_idle(q: *mut xe_exec_queue) -> bool;
}
extern "C" {
    pub fn xe_exec_queue_kill(q: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_exec_queue_device_get_max_priority(xe: *mut xe_device) -> xe_exec_queue_priority;
}
extern "C" {
    pub fn xe_exec_queue_last_fence_put(e: *mut xe_exec_queue, vm: *mut xe_vm);
}
extern "C" {
    pub fn xe_exec_queue_last_fence_put_unlocked(e: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_exec_queue_update_run_ticks(q: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_exec_queue_contexts_hwsp_rebase(q: *mut xe_exec_queue, scratch: *mut c_void) -> c_int;
}
