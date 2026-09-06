//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/wait.h
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
// ->cq_wait_nr is armed with the number of lazy task_work adds the waiter
// still needs, and counted down by the add side, with the add reaching zero
// issuing the (single) wake up for this wait cycle. Zero and below means no
// wake up is to be issued: IO_CQ_WAKE_INIT when no task is waiting (also
// what a forced wake up resets it to when claiming one), zero once the
// countdown has fired.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_arg {
    pub argsz: usize,
    pub ts: timespec64,
    pub sig: *const sigset_t __user,
    pub min_time: ktime_t,
    pub ts_set: bool,
    pub iowait: bool,
}

extern "C" {
    pub fn io_run_task_work_sig(ctx: *mut io_ring_ctx) -> c_int;
}
extern "C" {
    pub fn io_cqring_do_overflow_flush(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_cqring_overflow_flush_locked(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn READ_ONCE(READ_ONCE(rings->cq.head: rings->cq.tail) -) -> return;
}
//
// Reads the tail/head of the CQ ring while providing an acquire ordering,
// see comment at top of io_uring.c.
//
extern "C" {
    pub fn __io_cqring_events(_arg: ctx) -> return;
}
