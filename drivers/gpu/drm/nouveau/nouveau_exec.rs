//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_exec.h
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
#[derive(Copy, Clone)]
pub struct nouveau_exec_job_args {
    pub file_priv: *mut drm_file,
    pub sched: *mut nouveau_sched,
    pub chan: *mut nouveau_channel,
    pub s: *mut drm_nouveau_sync,
    pub count: u32,
    pub in_sync: },
    pub s: *mut drm_nouveau_sync,
    pub count: u32,
    pub out_sync: },
    pub s: *mut drm_nouveau_exec_push,
    pub count: u32,
    pub push: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_exec_job {
    pub base: nouveau_job,
    pub fence: *mut nouveau_fence,
    pub chan: *mut nouveau_channel,
    pub s: *mut drm_nouveau_exec_push,
    pub count: u32,
    pub push: },
}

// Limit the number of IBs per job to half the size of the ring in order
// to avoid the ring running dry between submissions and preserve one
// more slot for the job's HW fence.
//
