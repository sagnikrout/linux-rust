//! Automatically rewritten from C Header to Rust Module
//! Source: block/elevator.h
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

//
// Return values from elevator merger
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum elv_merge {
    ELEVATOR_NO_MERGE	= 0,
    ELEVATOR_FRONT_MERGE	= 1,
    ELEVATOR_BACK_MERGE	= 2,
    ELEVATOR_DISCARD_MERGE	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elevator_tags {
// num. of hardware queues for which tags are allocated
    pub nr_hw_queues: c_uint,
// depth used while allocating tags
    pub nr_requests: c_uint,
// shared tag is stored at index 0
    pub tags: [*mut blk_mq_tags; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elevator_resources {
// holds elevator data
    pub data: *mut c_void,
// holds elevator tags
    pub et: *mut elevator_tags,
}

// Holding context data for changing elevator
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elv_change_ctx {
    pub name: *const c_char,
    pub no_uevent: bool,
// for unregistering old elevator
    pub old: *mut elevator_queue,
// for registering new elevator
    pub new: *mut elevator_queue,
// store elevator type
    pub type: *mut elevator_type,
// store elevator resources
    pub res: elevator_resources,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elevator_mq_ops {
    pub ): *mut *mut *mut int (init_sched)(struct request_queue , struct elevator_queue,
    pub ): *mut *mut void (exit_sched)(struct elevator_queue,
    pub int): *mut *mut *mut int (init_hctx)(struct blk_mq_hw_ctx , unsigned,
    pub int): *mut *mut *mut void (exit_hctx)(struct blk_mq_hw_ctx , unsigned,
    pub ): *mut *mut void (depth_updated)(struct request_queue,
    pub ): *mut *mut *mut void (alloc_sched_data)(struct request_queue,
    pub ): *mut *mut void (free_sched_data)(void,
    pub ): *mut *mut *mut *mut bool (allow_merge)(struct request_queue , struct request , struct bio,
    pub int): *mut *mut *mut *mut bool (bio_merge)(struct request_queue , struct bio , unsigned,
    pub ): *mut *mut *mut *mut *mut int (request_merge)(struct request_queue q, struct request , struct bio,
    pub elv_merge): *mut *mut *mut *mut void (request_merged)(struct request_queue , struct request , enum,
    pub ): *mut *mut *mut *mut void (requests_merged)(struct request_queue , struct request , struct request,
    pub ): *mut *mut void (limit_depth)(blk_opf_t, struct blk_mq_alloc_data,
    pub ): *mut *mut void (prepare_request)(struct request,
    pub ): *mut *mut void (finish_request)(struct request,
    pub flags): blk_insert_t,
    pub ): *mut *mut *mut request (dispatch_request)(blk_mq_hw_ctx,
    pub ): *mut *mut bool (has_work)(struct blk_mq_hw_ctx,
    pub u64): *mut *mut *mut void (completed_request)(struct request ,,
    pub ): *mut *mut void (requeue_request)(struct request,
    pub ): *mut *mut *mut *mut request (former_request)(request_queue , request,
    pub ): *mut *mut *mut *mut request (next_request)(request_queue , request,
    pub ): *mut *mut void (init_icq)(struct io_cq,
    pub ): *mut *mut void (exit_icq)(struct io_cq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elv_fs_entry {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct elevator_queue , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct elevator_queue , char ,,
}

//
// identifies an elevator type, such as AS or deadline
//
// managed by elevator core
// fields provided by elevator implementation

// managed by elevator core
extern "C" {
    pub fn try_module_get(_arg: e->elevator_owner) -> return;
}
pub const ELV_HASH_BITS: c_int = 6;
extern "C" {
    pub fn elv_rqhash_del(q: *mut request_queue, rq: *mut request);
}
extern "C" {
    pub fn elv_rqhash_add(q: *mut request_queue, rq: *mut request);
}
extern "C" {
    pub fn elv_rqhash_reposition(q: *mut request_queue, rq: *mut request);
}
//
// each queue has an elevator_queue associated with it
//
pub const ELEVATOR_FLAG_REGISTERED: c_int = 0;
pub const ELEVATOR_FLAG_DYING: c_int = 1;
//
// block elevator interface
//
// io scheduler registration
//
extern "C" {
    pub fn elv_register(: *mut elevator_type) -> c_int;
}
extern "C" {
    pub fn elv_unregister(: *mut elevator_type);
}
//
// io scheduler sysfs switching
//
extern "C" {
    pub fn elv_iosched_show(disk: *mut gendisk, page: *mut c_char) -> isize;
}
extern "C" {
    pub fn elv_iosched_store(disk: *mut gendisk, page: *const c_char, count: usize) -> isize;
}
extern "C" {
    pub fn elv_bio_merge_ok(: *mut request, : *mut bio) -> bool;
}
//
// Helper functions.
//
// rb support functions.
//
extern "C" {
    pub fn elv_rb_add(: *mut rb_root, : *mut request);
}
extern "C" {
    pub fn elv_rb_del(: *mut rb_root, : *mut request);
}
//
// Insertion selection
//
pub const ELEVATOR_INSERT_FRONT: c_int = 1;
pub const ELEVATOR_INSERT_BACK: c_int = 2;
pub const ELEVATOR_INSERT_SORT: c_int = 3;
pub const ELEVATOR_INSERT_REQUEUE: c_int = 4;
pub const ELEVATOR_INSERT_FLUSH: c_int = 5;
pub const ELEVATOR_INSERT_SORT_MERGE: c_int = 6;

extern "C" {
    pub fn blk_mq_sched_reg_debugfs(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_sched_unreg_debugfs(q: *mut request_queue);
}
