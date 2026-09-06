//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/rnbd/rnbd-clt.h
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
//
// RDMA Network Block Driver
//
// Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
// Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
// Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
//

// time in seconds between reconnect tries, default to 30 s
pub const RECONNECT_DELAY: c_int = 30;
//
// Number of times to reconnect on error before giving up, 0 for * disabled,
// -1 for forever
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnbd_clt_dev_state {
    DEV_STATE_INIT,
    DEV_STATE_MAPPED,
    DEV_STATE_MAPPED_DISCONNECTED,
    DEV_STATE_UNMAPPED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_iu_comp {
    pub wait: wait_queue_head_t,
    pub errno: c_int,
}

pub const RNBD_INLINE_SG_CNT: c_int = 0;

pub const RNBD_INLINE_SG_CNT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_iu {
    pub /: *mut *mut *mut request rq; / for block io,
    pub /: *mut *mut *mut void buf; / for user messages,
}

// use to send msg associated with a dev
// use to send msg associated with a sess
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_cpu_qlist {
    pub requeue_list: list_head,
    pub requeue_lock: spinlock_t,
    pub cpu: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_clt_session {
    pub list: list_head,
    pub rtrs: *mut rtrs_clt_sess,
    pub rtrs_waitq: wait_queue_head_t,
    pub rtrs_ready: bool,
// cpu_queues;
    pub NR_CPUS): DECLARE_BITMAP(cpu_queues_bm,,
    pub /: *mut *mut *mut int __percpu cpu_rr; / per-cpu var for CPU round-robin,
    pub busy: core::sync::atomic::AtomicI32,
    pub queue_depth: usize,
    pub max_io_size: u32,
    pub max_segments: u32,
    pub tag_set: blk_mq_tag_set,
    pub nr_poll_queues: u32,
    pub /: *mut *mut mutex lock; / protects state and devs_list,
    pub /: *mut *mut list_head devs_list; / list of rnbd_clt_dev,
    pub refcount: refcount_t,
    pub sessname: [c_char; NAME_MAX],
    pub /: *mut *mut u8 ver; / protocol version,
}

//
// Submission queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_queue {
    pub requeue_list: list_head,
    pub in_list: c_ulong,
    pub dev: *mut rnbd_clt_dev,
    pub hctx: *mut blk_mq_hw_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_clt_dev {
    pub kobj: kobject,
    pub sess: *mut rnbd_clt_session,
    pub queue: *mut request_queue,
    pub hw_queues: *mut rnbd_queue,
    pub device_id: u32,
// local Idr index - used to track minor number allocations.
    pub clt_device_id: c_int,
    pub lock: mutex,
    pub dev_state: rnbd_clt_dev_state,
    pub refcount: refcount_t,
    pub pathname: *mut c_char,
    pub access_mode: rnbd_access_mode,
    pub nr_poll_queues: u32,
    pub /: *mut *mut u64 size; / device size in bytes,
    pub list: list_head,
    pub gd: *mut gendisk,
    pub blk_symlink_name: *mut c_char,
    pub unmap_on_rmmod_work: work_struct,
}

// rnbd-clt.c
extern "C" {
    pub fn rnbd_clt_remap_device(dev: *mut rnbd_clt_dev) -> c_int;
}
extern "C" {
    pub fn rnbd_clt_resize_disk(dev: *mut rnbd_clt_dev, newsize: sector_t) -> c_int;
}
// rnbd-clt-sysfs.c
extern "C" {
    pub fn rnbd_clt_create_sysfs_files() -> c_int;
}
extern "C" {
    pub fn rnbd_clt_destroy_sysfs_files();
}
extern "C" {
    pub fn rnbd_clt_remove_dev_symlink(dev: *mut rnbd_clt_dev);
}
