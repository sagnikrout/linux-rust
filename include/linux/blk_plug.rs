//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blk_plug.h
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


// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn void(cb: *mut *mut blk_plug_cb_fn)(struct blk_plug_cb, from_schedule: bool) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_list {
    pub head: *mut request,
    pub tail: *mut request,
}

//
// blk_plug permits building a queue of related requests by holding the I/O
// fragments for a short period. This allows merging of sequential requests
// into single larger request. As the requests are moved from a per-task list to
// the device's request_queue in a batch, this results in improved scalability
// as the lock contention for request_queue lock is reduced.
//
// It is ok not to disable preemption when adding the request to the plug list
// or when attempting a merge. For details, please see schedule() where
// blk_flush_plug() is called.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_plug {
    pub /: *mut *mut rq_list mq_list; / blk-mq requests,
// if ios_left is > 1, we can batch tag/rq allocations
    pub cached_rqs: rq_list,
    pub cur_ktime: u64,
    pub nr_ios: c_ushort,
    pub rq_count: c_ushort,
    pub multiple_queues: bool,
    pub has_elevator: bool,
    pub /: *mut *mut list_head cb_list; / md requires an unplug callback,
}

extern "C" {
    pub fn blk_start_plug(: *mut blk_plug);
}
extern "C" {
    pub fn blk_start_plug_nr_ios(: *mut blk_plug, short: unsigned);
}
extern "C" {
    pub fn blk_finish_plug(: *mut blk_plug);
}
extern "C" {
    pub fn __blk_flush_plug(plug: *mut blk_plug, from_schedule: bool);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_plug_cb {
    pub list: list_head,
    pub callback: blk_plug_cb_fn,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_plug {
}

