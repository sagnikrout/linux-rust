//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/lima/lima_sched.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Copyright 2017-2019 Qiang Yu <yuq825@gmail.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_sched_error_task {
    pub list: list_head,
    pub data: *mut c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_sched_task {
    pub base: drm_sched_job,
    pub vm: *mut lima_vm,
    pub frame: *mut c_void,
    pub bos: *mut lima_bo,
    pub num_bos: c_int,
    pub recoverable: bool,
    pub heap: *mut lima_bo,
// pipe fence
    pub fence: *mut dma_fence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_sched_context {
    pub base: drm_sched_entity,
}

pub const LIMA_SCHED_PIPE_MAX_MMU: c_int = 8;
pub const LIMA_SCHED_PIPE_MAX_L2_CACHE: c_int = 2;
pub const LIMA_SCHED_PIPE_MAX_PROCESSOR: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_sched_pipe {
    pub base: drm_gpu_scheduler,
    pub fence_context: u64,
    pub fence_seqno: u32,
    pub fence_lock: spinlock_t,
    pub ldev: *mut lima_device,
    pub current_task: *mut lima_sched_task,
    pub current_vm: *mut lima_vm,
    pub mmu: [*mut lima_ip; LIMA_SCHED_PIPE_MAX_MMU],
    pub num_mmu: c_int,
    pub l2_cache: [*mut lima_ip; LIMA_SCHED_PIPE_MAX_L2_CACHE],
    pub num_l2_cache: c_int,
    pub processor: [*mut lima_ip; LIMA_SCHED_PIPE_MAX_PROCESSOR],
    pub num_processor: c_int,
    pub bcast_processor: *mut lima_ip,
    pub bcast_mmu: *mut lima_ip,
    pub done: u32,
    pub error: bool,
    pub task: core::sync::atomic::AtomicI32,
    pub frame_size: c_int,
    pub task_slab: *mut kmem_cache,
    pub task): *mut *mut *mut int (task_validate)(struct lima_sched_pipe pipe, struct lima_sched_task,
    pub task): *mut *mut *mut void (task_run)(struct lima_sched_pipe pipe, struct lima_sched_task,
    pub pipe): *mut *mut void (task_fini)(struct lima_sched_pipe,
    pub pipe): *mut *mut void (task_error)(struct lima_sched_pipe,
    pub pipe): *mut *mut void (task_mmu_error)(struct lima_sched_pipe,
    pub pipe): *mut *mut int (task_recover)(struct lima_sched_pipe,
    pub pipe): *mut *mut void (task_mask_irq)(struct lima_sched_pipe,
    pub recover_work: work_struct,
}

extern "C" {
    pub fn lima_sched_task_fini(task: *mut lima_sched_task);
}
extern "C" {
    pub fn lima_sched_pipe_init(pipe: *mut lima_sched_pipe, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn lima_sched_pipe_fini(pipe: *mut lima_sched_pipe);
}
extern "C" {
    pub fn lima_sched_pipe_task_done(pipe: *mut lima_sched_pipe);
}
extern "C" {
    pub fn lima_sched_slab_init() -> c_int;
}
extern "C" {
    pub fn lima_sched_slab_fini();
}
