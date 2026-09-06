//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/io_uring.h
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
pub struct io_rings_layout {
// size of CQ + headers + SQ offset array
    pub rings_size: usize,
    pub sq_size: usize,
    pub sq_array_offset: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_ctx_config {
    pub p: io_uring_params,
    pub layout: io_rings_layout,
    pub uptr: *mut io_uring_params __user,
}

//
// Complaint timeout for io_uring cancelation exits, and for io-wq exit
// worker waiting.
//

//
// The request has more work to do and should be retried. io_uring will
// attempt to wait on the file for eligible opcodes, but otherwise
// it'll be handed to iowq for blocking execution. It works for normal
// requests as well as for the multi shot mode.
//
// Requeue the task_work to restart operations on this request. The
// actual value isn't important, should just be not an otherwise
// valid error code, yet less than -MAX_ERRNO and valid internally.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_defer_entry {
    pub list: list_head,
    pub req: *mut io_kiocb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_wait_queue {
    pub wq: wait_queue_entry,
    pub ctx: *mut io_ring_ctx,
    pub cq_tail: unsigned,
    pub cq_min_tail: unsigned,
    pub nr_timeouts: unsigned,
    pub hit_timeout: c_int,
    pub min_timeout: ktime_t,
    pub timeout: ktime_t,
    pub t: hrtimer,

    pub napi_busy_poll_dt: ktime_t,
    pub napi_prefer_busy_poll: bool,

}

//
// Wake up if we have enough events, or if a timeout occurred since we
// started waiting. For timeouts, we always want to return to userspace,
// regardless of event count.
//
pub const IORING_MAX_ENTRIES: c_int = 32768;

extern "C" {
    pub fn io_prepare_config(config: *mut io_ctx_config) -> c_int;
}
extern "C" {
    pub fn io_cqe_cache_refill(ctx: *mut io_ring_ctx, overflow: bool, cqe32: bool) -> bool;
}
extern "C" {
    pub fn io_req_defer_failed(req: *mut io_kiocb, res: i32);
}
extern "C" {
    pub fn io_post_aux_cqe(ctx: *mut io_ring_ctx, user_data: u64, res: i32, cflags: u32) -> bool;
}
extern "C" {
    pub fn io_add_aux_cqe(ctx: *mut io_ring_ctx, user_data: u64, res: i32, cflags: u32);
}
extern "C" {
    pub fn io_req_post_cqe(req: *mut io_kiocb, res: i32, cflags: u32) -> bool;
}
extern "C" {
    pub fn io_req_post_cqe32(req: *mut io_kiocb, src_cqe[2]: io_uring_cqe) -> bool;
}
extern "C" {
    pub fn __io_commit_cqring_flush(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_linked_nr(req: *mut io_kiocb) -> unsigned;
}
extern "C" {
    pub fn io_req_track_inflight(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_req_task_queue(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_req_task_complete(tw_req: io_tw_req, tw: io_tw_token_t);
}
extern "C" {
    pub fn io_req_task_queue_fail(req: *mut io_kiocb, ret: c_int);
}
extern "C" {
    pub fn io_req_task_submit(tw_req: io_tw_req, tw: io_tw_token_t);
}
extern "C" {
    pub fn io_uring_drop_tctx_refs(task: *mut task_struct) -> __cold void;
}
extern "C" {
    pub fn io_queue_iowq(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_poll_issue(req: *mut io_kiocb, tw: io_tw_token_t) -> c_int;
}
extern "C" {
    pub fn io_submit_sqes(ctx: *mut io_ring_ctx, nr: c_uint) -> c_int;
}
extern "C" {
    pub fn io_do_iopoll(ctx: *mut io_ring_ctx, force_nonspin: bool) -> c_int;
}
extern "C" {
    pub fn io_iopoll_try_reap_events(ctx: *mut io_ring_ctx) -> __cold void;
}
extern "C" {
    pub fn __io_submit_flush_completions(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_wq_submit_work(work: *mut io_wq_work);
}
extern "C" {
    pub fn io_free_req(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_queue_next(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_task_refs_refill(tctx: *mut io_uring_task);
}
extern "C" {
    pub fn __io_alloc_req_refill(ctx: *mut io_ring_ctx) -> bool;
}
extern "C" {
    pub fn io_activate_pollwq(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_restriction_clone(dst: *mut io_restriction, src: *mut io_restriction);
}
extern "C" {
    pub fn io_poison_req(req: *mut io_kiocb);
}

//
// ->submitter_task may be NULL and we can still post a CQE,
// if the ring has been setup with IORING_SETUP_R_DISABLED.
// Not from an SQE, as those cannot be submitted, but via
// updating tagged resources.
//

extern "C" {
    pub fn IS_ENABLED(IO_RING_F_COMPAT: CONFIG_COMPAT) && unlikely(ctx->int_flags &) -> return;
}

// ret = ctx->cqe_cached;
extern "C" {
    pub fn io_get_cqe_overflow(_arg: ctx, _arg: ret, _arg: false, _arg: cqe32) -> return;
}
extern "C" {
    pub fn io_get_cqe(_arg: ctx, _arg: cqe_ret, IORING_SETUP_CQE_MIXED: ctx->flags &) -> return;
}
//
// If we can't get a cq entry, userspace overflowed the submission
// (by quite a lot).
//
// "Normal" inline submissions always hold the uring_lock, since we
// grab it from the system call. Same is true for the SQPOLL offload.
// The only exception is when we've detached the request and issue it
// from an async worker thread, grab the lock for that case.
//
// order cqe stores with ring update
//
// Pass in EPOLLIN|EPOLL_URING_WAKE as the poll wakeup key. The latter
// set in the mask so that if we recurse back into our own poll
// waitqueue handlers, we know we have a dependency between eventfd or
// epoll and should terminate multishot poll at that point.
//
// Trigger waitqueue handler on all waiters on our waitqueue. This
// won't necessarily wake up all the tasks, io_should_wake() will make
// that decision.
//
// SQPOLL must use the actual sqring head, as using the cached_sq_head
// is race prone if the SQPOLL thread has grabbed entries but not yet
// committed them to the ring. For !SQPOLL, this doesn't matter, but
// since this helper is just used for SQPOLL sqring waits (or POLLOUT),
// just read the actual sqring head unconditionally.
//
extern "C" {
    pub fn __io_sqring_full(_arg: ctx) -> return;
}
// make sure SQ entry isn't read before tail
extern "C" {
    pub fn min(_arg: entries, _arg: ctx->sq_entries) -> return;
}
extern "C" {
    pub fn __io_sqring_entries(_arg: ctx) -> return;
}
//
// Don't complete immediately but use deferred completion infrastructure.
// Protected by ->uring_lock and can only be used either with
// IO_URING_F_COMPLETE_DEFER or inside a tw handler holding the mutex.
//

// req = io_extract_req(ctx);
extern "C" {
    pub fn ktime_get() -> return;
}
extern "C" {
    pub fn ktime_get_with_offset(_arg: ctx->clock_offset) -> return;
}
