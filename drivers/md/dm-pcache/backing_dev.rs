//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-pcache/backing_dev.h
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

extern "C" {
    pub fn void(backing_req: *mut *mut backing_req_end_fn_t)(struct pcache_backing_dev_req, ret: c_int) -> typedef;
}
pub const BACKING_DEV_REQ_TYPE_REQ: c_int = 1;
pub const BACKING_DEV_REQ_TYPE_KMEM: c_int = 2;
pub const BACKING_DEV_REQ_INLINE_BVECS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_backing_dev_req {
    pub type: u8,
    pub bio: bio,
    pub backing_dev: *mut pcache_backing_dev,
    pub priv_data: *mut c_void,
    pub end_req: backing_req_end_fn_t,
    pub node: list_head,
    pub ret: c_int,
    pub upper_req: *mut pcache_request,
    pub bio_off: u32,
    pub req: },
    pub inline_bvecs: [bio_vec; BACKING_DEV_REQ_INLINE_BVECS],
    pub bvecs: *mut bio_vec,
    pub n_vecs: u32,
    pub kmem: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_backing_dev {
    pub cache: *mut pcache_cache,
    pub dm_dev: *mut dm_dev,
    pub req_pool: mempool_t,
    pub bvec_pool: mempool_t,
    pub submit_list: list_head,
    pub submit_lock: spinlock_t,
    pub req_submit_work: work_struct,
    pub complete_list: list_head,
    pub complete_lock: spinlock_t,
    pub req_complete_work: work_struct,
    pub inflight_reqs: core::sync::atomic::AtomicI32,
    pub inflight_wq: wait_queue_head_t,
    pub dev_size: u64,
}

extern "C" {
    pub fn backing_dev_start(pcache: *mut dm_pcache) -> c_int;
}
extern "C" {
    pub fn backing_dev_stop(pcache: *mut dm_pcache);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_backing_dev_req_opts {
    pub type: u32,
    pub upper_req: *mut pcache_request,
    pub req_off: u32,
    pub len: u32,
    pub req: },
    pub data: *mut c_void,
    pub opf: blk_opf_t,
    pub len: u32,
    pub backing_off: u64,
    pub kmem: },
}

extern "C" {
    pub fn backing_dev_req_submit(backing_req: *mut pcache_backing_dev_req, direct: bool);
}
extern "C" {
    pub fn backing_dev_req_end(backing_req: *mut pcache_backing_dev_req);
}
extern "C" {
    pub fn backing_dev_flush(backing_dev: *mut pcache_backing_dev);
}
extern "C" {
    pub fn pcache_backing_init() -> c_int;
}
extern "C" {
    pub fn pcache_backing_exit();
}
