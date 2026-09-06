//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/lima/lima_drv.h
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
pub struct lima_drm_priv {
    pub vm: *mut lima_vm,
    pub ctx_mgr: lima_ctx_mgr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_submit {
    pub ctx: *mut lima_ctx,
    pub pipe: c_int,
    pub flags: u32,
    pub bos: *mut drm_lima_gem_submit_bo,
    pub lbos: *mut lima_bo,
    pub nr_bos: u32,
    pub in_sync: [u32; 2],
    pub out_sync: u32,
    pub task: *mut lima_sched_task,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_compatible {
    pub id: lima_gpu_id,
}
