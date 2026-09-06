//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-mq.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_ctxs {
    pub kobj: kobject,
    pub queue_ctx: *mut blk_mq_ctx __percpu,
}

//
// struct blk_mq_ctx - State for a software queue facing the submitting CPUs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_ctx {
    pub lock: spinlock_t,
    pub rq_lists: [list_head; HCTX_MAX_TYPES],
    pub ____cacheline_aligned_in_smp: },
    pub cpu: c_uint,
    pub index_hw: [c_ushort; HCTX_MAX_TYPES],
    pub hctxs: [*mut blk_mq_hw_ctx; HCTX_MAX_TYPES],
    pub queue: *mut request_queue,
    pub ctxs: *mut blk_mq_ctxs,
    pub kobj: kobject,
    pub ____cacheline_aligned_in_smp: },
}

pub type blk_insert_t = u32;

extern "C" {
    pub fn blk_mq_submit_bio(bio: *mut bio);
}
extern "C" {
    pub fn blk_mq_exit_queue(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_wake_waiters(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_flush_busy_ctxs(hctx: *mut blk_mq_hw_ctx, list: *mut list_head);
}
extern "C" {
    pub fn blk_mq_put_rq_ref(rq: *mut request);
}
//
// Internal helpers for allocating/freeing the request map
//
extern "C" {
    pub fn blk_mq_free_rq_map(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags);
}
//
// CPU -> queue mappings
//
extern "C" {
    pub fn blk_mq_hw_queue_to_node(qmap: *mut blk_mq_queue_map, int: unsigned) -> c_int;
}
//
// blk_mq_map_queue_type() - map (hctx_type,cpu) to hardware queue
// @q: request queue
// @type: the hctx type index
// @cpu: CPU
//
extern "C" {
    pub fn queue_hctx(_arg: (q), _arg: (q->tag_set->map[type].mq_map[cpu])) -> return;
}
//
// The caller ensure that if REQ_POLLED, poll must be enabled.
//
// blk_mq_map_queue() - map (cmd_flags,type) to hardware queue
// @opf: operation type (REQ_OP_*) and flags (e.g. REQ_POLLED).
// @ctx: software queue cpu ctx
//
// Default to double of smaller one between hw queue_depth and
// 128, since we don't split into sync/async like the old code
// did. Additionally, this is a per-hw queue depth.
//
extern "C" {
    pub fn min_t(int: unsigned, _arg: set->queue_depth, _arg: BLKDEV_DEFAULT_RQ) -> *mut return 2;
}
//
// sysfs helpers
//
extern "C" {
    pub fn blk_mq_sysfs_init(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_sysfs_deinit(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_sysfs_register(disk: *mut gendisk) -> c_int;
}
extern "C" {
    pub fn blk_mq_sysfs_unregister(disk: *mut gendisk);
}
extern "C" {
    pub fn blk_mq_sysfs_register_hctxs(q: *mut request_queue) -> c_int;
}
extern "C" {
    pub fn blk_mq_sysfs_unregister_hctxs(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_hctx_kobj_init(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn blk_mq_free_plug_rqs(plug: *mut blk_plug);
}
extern "C" {
    pub fn blk_mq_flush_plug_list(plug: *mut blk_plug, from_schedule: bool);
}
extern "C" {
    pub fn blk_mq_cancel_work_sync(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_release(q: *mut request_queue);
}
extern "C" {
    pub fn per_cpu_ptr(_arg: q->queue_ctx, _arg: cpu) -> return;
}
//
// This assumes per-cpu software queueing queues. They could be per-node
// as well, for instance. For now this is hardcoded as-is. Note that we don't
// care about preemption, since we know the ctx's are persistent. This does
// mean that we can't rely on ctx always matching the currently running CPU.
//
extern "C" {
    pub fn __blk_mq_get_ctx(_arg: q, _arg: raw_smp_processor_id()) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_alloc_data {
// input parameter
    pub q: *mut request_queue,
    pub flags: blk_mq_req_flags_t,
    pub shallow_depth: c_uint,
    pub cmd_flags: blk_opf_t,
    pub rq_flags: req_flags_t,
// allocate multiple requests/tags in one go
    pub nr_tags: c_uint,
    pub cached_rqs: *mut rq_list,
// input & output parameter
    pub ctx: *mut blk_mq_ctx,
    pub hctx: *mut blk_mq_hw_ctx,
}

extern "C" {
    pub fn blk_mq_free_tags(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags);
}
extern "C" {
    pub fn blk_mq_get_tag(data: *mut blk_mq_alloc_data) -> c_uint;
}
extern "C" {
    pub fn blk_mq_put_tags(tags: *mut blk_mq_tags, tag_array: *mut c_int, nr_tags: c_int);
}
extern "C" {
    pub fn blk_mq_tag_wakeup_all(tags: *mut blk_mq_tags, _arg: bool);
}
extern "C" {
    pub fn sbq_wait_ptr(_arg: bt, _arg: &hctx->wait_index) -> return;
}
extern "C" {
    pub fn __blk_mq_tag_busy(: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn __blk_mq_tag_idle(: *mut blk_mq_hw_ctx);
}
// Fast path: hardware queue is not stopped most of the time.
//
// This barrier is used to order adding of dispatch list before and
// the test of BLK_MQ_S_STOPPED below. Pairs with the memory barrier
// in blk_mq_start_stopped_hw_queue() so that dispatch code could
// either see BLK_MQ_S_STOPPED is cleared or dispatch list is not
// empty to avoid missing dispatching requests.
//
extern "C" {
    pub fn test_bit(_arg: BLK_MQ_S_STOPPED, _arg: &hctx->state) -> return;
}
extern "C" {
    pub fn blk_mq_in_driver_rw(part: *mut block_device, inflight[2]: c_uint);
}
extern "C" {
    pub fn atomic_read(_arg: &hctx->queue->nr_active_requests_shared_tags) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &hctx->nr_active) -> return;
}
extern "C" {
    pub fn __blk_mq_alloc_driver_tag(rq: *mut request) -> bool;
}
// Free all requests on the list
//
// For shared tag users, we track the number of currently active users
// and attempt to provide a fair share of the tag depth for each of them.
//
// Don't try dividing an ant
//
// Allow at least some tags
//
// run the code block in @dispatch_ops with rcu/srcu read lock held

