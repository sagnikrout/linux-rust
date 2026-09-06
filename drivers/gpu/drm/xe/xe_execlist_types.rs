//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_execlist_types.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_execlist_port {
    pub hwe: *mut xe_hw_engine,
    pub lock: spinlock_t,
    pub active: [list_head; XE_EXEC_QUEUE_PRIORITY_COUNT],
    pub last_ctx_id: u32,
    pub running_exl: *mut xe_execlist_exec_queue,
    pub irq_fail: timer_list,
    pub lrc: *mut xe_lrc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_execlist_exec_queue {
    pub q: *mut xe_exec_queue,
    pub sched: drm_gpu_scheduler,
    pub entity: drm_sched_entity,
    pub port: *mut xe_execlist_port,
    pub has_run: bool,
    pub destroy_async: work_struct,
    pub active_priority: xe_exec_queue_priority,
    pub active_link: list_head,
}
