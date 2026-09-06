//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/tw.h
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

pub const IO_LOCAL_TW_DEFAULT_MAX: c_int = 20;
//
// Terminate the request if either of these conditions are true:
//
// 1) It's being executed by the original task, but that task is marked
// with PF_EXITING as it's exiting.
// 2) PF_KTHREAD is set, in which case the invoker of the task_work is
// our fallback task_work.
// 3) The ring has been closed and is going away.
//
extern "C" {
    pub fn io_req_task_work_add_remote(req: *mut io_kiocb, flags: unsigned);
}
extern "C" {
    pub fn tctx_task_work(cb: *mut callback_head);
}
extern "C" {
    pub fn io_tctx_fallback_work(work: *mut work_struct);
}
extern "C" {
    pub fn io_run_local_work(ctx: *mut io_ring_ctx, min_events: c_int, max_events: c_int) -> c_int;
}
extern "C" {
    pub fn io_run_task_work_sig(ctx: *mut io_ring_ctx) -> c_int;
}
extern "C" {
    pub fn io_cancel_local_task_work(ctx: *mut io_ring_ctx) -> __cold void;
}
extern "C" {
    pub fn io_run_local_work_locked(ctx: *mut io_ring_ctx, min_events: c_int) -> c_int;
}
extern "C" {
    pub fn io_req_local_work_add(req: *mut io_kiocb, flags: unsigned);
}
extern "C" {
    pub fn io_req_normal_work_add(req: *mut io_kiocb);
}
extern "C" {
    pub fn tctx_task_work_run(tctx: *mut io_uring_task, max_entries: c_uint, count: *mut c_uint);
}
//
// Always check-and-clear the task_work notification signal. With how
// signaling works for task_work, we can find it set with nothing to
// run. We need to clear it for that case, like get_signal() does.
//
// PF_IO_WORKER never returns to userspace, so check here if we have
// notify work that needs processing.
//
extern "C" {
    pub fn task_work_pending(io_local_work_pending(ctx: current) ||) -> return;
}
extern "C" {
    pub fn likely(current: ctx->submitter_task ==) -> return;
}
