//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/io-wq.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum io_wq_cancel {
    IO_WQ_CANCEL_OK,	/* cancelled before started */
    IO_WQ_CANCEL_RUNNING,	/* found, running, and attempted cancelled */
    IO_WQ_CANCEL_NOTFOUND,	/* work not found */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_wq_hash {
    pub refs: refcount_t,
    pub map: c_ulong,
    pub wait: wait_queue_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_wq_data {
    pub hash: *mut io_wq_hash,
    pub task: *mut task_struct,
}

extern "C" {
    pub fn io_wq_exit_start(wq: *mut io_wq);
}
extern "C" {
    pub fn io_wq_put_and_exit(wq: *mut io_wq);
}
extern "C" {
    pub fn io_wq_set_exit_on_idle(wq: *mut io_wq, enable: bool);
}
extern "C" {
    pub fn io_wq_enqueue(wq: *mut io_wq, work: *mut io_wq_work);
}
extern "C" {
    pub fn io_wq_hash_work(work: *mut io_wq_work, val: *mut c_void);
}
extern "C" {
    pub fn io_wq_cpu_affinity(tctx: *mut io_uring_task, mask: cpumask_var_t) -> c_int;
}
extern "C" {
    pub fn io_wq_max_workers(wq: *mut io_wq, new_count: *mut c_int) -> c_int;
}
extern "C" {
    pub fn io_wq_worker_stopped() -> bool;
}
extern "C" {
    pub fn __io_wq_is_hashed(_arg: atomic_read(&work->flags)) -> return;
}
extern "C" {
    pub fn bool(: *mut work_cancel_fn)(struct io_wq_work, : *mut c_void) -> typedef;
}

extern "C" {
    pub fn io_wq_worker_sleeping(: *mut task_struct);
}
extern "C" {
    pub fn io_wq_worker_running(: *mut task_struct);
}

