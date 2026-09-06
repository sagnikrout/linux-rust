//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_sched.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nouveau_job_state {
    NOUVEAU_JOB_UNINITIALIZED = 0,
    NOUVEAU_JOB_INITIALIZED,
    NOUVEAU_JOB_SUBMIT_SUCCESS,
    NOUVEAU_JOB_SUBMIT_FAILED,
    NOUVEAU_JOB_RUN_SUCCESS,
    NOUVEAU_JOB_RUN_FAILED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_job_args {
    pub file_priv: *mut drm_file,
    pub sched: *mut nouveau_sched,
    pub credits: u32,
    pub resv_usage: dma_resv_usage,
    pub sync: bool,
    pub s: *mut drm_nouveau_sync,
    pub count: u32,
    pub in_sync: },
    pub s: *mut drm_nouveau_sync,
    pub count: u32,
    pub out_sync: },
    pub ops: *const nouveau_job_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_job {
    pub base: drm_sched_job,
    pub state: nouveau_job_state,
    pub sched: *mut nouveau_sched,
    pub entry: list_head,
    pub file_priv: *mut drm_file,
    pub cli: *mut nouveau_cli,
    pub resv_usage: dma_resv_usage,
    pub done_fence: *mut dma_fence,
    pub sync: bool,
    pub data: *mut drm_nouveau_sync,
    pub count: u32,
    pub in_sync: },
    pub data: *mut drm_nouveau_sync,
    pub objs: *mut drm_syncobj,
    pub chains: *mut dma_fence_chain,
    pub count: u32,
    pub out_sync: },
// If .submit() returns without any error, it is guaranteed that
// armed_submit() is called.
//
    pub ): *mut *mut *mut int (submit)(struct nouveau_job , struct drm_gpuvm_exec,
    pub ): *mut *mut *mut void (armed_submit)(struct nouveau_job , struct drm_gpuvm_exec,
    pub ): *mut *mut *mut dma_fence (run)(nouveau_job,
    pub ): *mut *mut void (free)(struct nouveau_job,
    pub ): *mut *mut drm_gpu_sched_stat (timeout)(struct nouveau_job,
    pub ops: *mut },
}

extern "C" {
    pub fn nouveau_job_fini(job: *mut nouveau_job);
}
extern "C" {
    pub fn nouveau_job_submit(job: *mut nouveau_job) -> c_int;
}
extern "C" {
    pub fn nouveau_job_done(job: *mut nouveau_job);
}
extern "C" {
    pub fn nouveau_job_free(job: *mut nouveau_job);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_sched {
    pub base: drm_gpu_scheduler,
    pub entity: drm_sched_entity,
    pub wq: *mut workqueue_struct,
    pub mutex: mutex,
    pub head: list_head,
    pub lock: spinlock_t,
    pub list: },
    pub wq: wait_queue_head,
    pub job: },
}

extern "C" {
    pub fn nouveau_sched_destroy(psched: *mut nouveau_sched);
}
