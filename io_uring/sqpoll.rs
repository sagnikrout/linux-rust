//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/sqpoll.h
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
pub struct io_sq_data {
    pub refs: refcount_t,
    pub park_pending: core::sync::atomic::AtomicI32,
    pub lock: mutex,
// ctx's that are using this sqd
    pub ctx_list: list_head,
    pub thread: *mut task___rcu,
    pub wait: wait_queue_head,
    pub sq_thread_idle: unsigned,
    pub sq_cpu: c_int,
    pub task_pid: pid_t,
    pub task_tgid: pid_t,
    pub work_time: u64,
    pub state: c_ulong,
    pub exited: completion,
}

extern "C" {
    pub fn io_sq_offload_create(ctx: *mut io_ring_ctx, p: *mut io_uring_params) -> c_int;
}
extern "C" {
    pub fn io_sq_thread_finish(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_sq_thread_stop(sqd: *mut io_sq_data);
}
extern "C" {
    pub fn io_sq_thread_park(sqd: *mut io_sq_data);
}
extern "C" {
    pub fn io_sq_thread_unpark(sqd: *mut io_sq_data);
}
extern "C" {
    pub fn io_put_sq_data(sqd: *mut io_sq_data);
}
extern "C" {
    pub fn io_sqpoll_wait_sq(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_sqpoll_wq_cpu_affinity(ctx: *mut io_ring_ctx, mask: cpumask_var_t) -> c_int;
}
extern "C" {
    pub fn io_sq_cpu_usec(tsk: *mut task_struct) -> u64;
}
