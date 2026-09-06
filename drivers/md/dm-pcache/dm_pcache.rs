//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-pcache/dm_pcache.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

pub const PCACHE_STATE_RUNNING: c_int = 1;
pub const PCACHE_STATE_STOPPING: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_pcache {
    pub ti: *mut dm_target,
    pub cache_dev: pcache_cache_dev,
    pub backing_dev: pcache_backing_dev,
    pub cache: pcache_cache,
    pub opts: pcache_cache_options,
    pub defered_req_list_lock: spinlock_t,
    pub defered_req_list: list_head,
    pub task_wq: *mut workqueue_struct,
    pub defered_req_work: work_struct,
    pub state: core::sync::atomic::AtomicI32,
    pub inflight_reqs: core::sync::atomic::AtomicI32,
    pub inflight_wq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_request {
    pub pcache: *mut dm_pcache,
    pub bio: *mut bio,
    pub off: u64,
    pub data_len: u32,
    pub ref: kref,
    pub ret: c_int,
    pub list_node: list_head,
}

extern "C" {
    pub fn pcache_req_get(pcache_req: *mut pcache_request);
}
extern "C" {
    pub fn pcache_req_put(pcache_req: *mut pcache_request, ret: c_int);
}
extern "C" {
    pub fn pcache_defer_reqs_kick(pcache: *mut dm_pcache);
}
