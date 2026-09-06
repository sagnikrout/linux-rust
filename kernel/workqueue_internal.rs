//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/workqueue_internal.h
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
// kernel/workqueue_internal.h
//
// Workqueue internal header file.  Only to be included by workqueue and
// core kernel subsystems.
//

//
// The poor guys doing the actual heavy lifting.  All on-duty workers are
// either serving the manager role, on idle list or on busy hash.  For
// details on the locking annotation (L, I, X...), refer to workqueue.c.
//
// Only to be used in workqueue and async.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct worker {
// on idle list while idle, on busy hash table while busy
    pub /: *mut *mut list_head entry; / L: while idle,
    pub /: *mut *mut hlist_node hentry; / L: while busy,
}

// used by the scheduler to determine a worker's last known identity
// L: for rescuers
// A: runs through worker->node
//
// Opaque string set with work_set_desc().  Printed out with task
// dump for debugging - WARN, BUG, panic or sysrq.
//
// used only by rescuers to point to the target workqueue
//
// current_wq_worker - return struct worker if %current is a workqueue worker
//
extern "C" {
    pub fn kthread_data(_arg: current) -> return;
}
//
// Scheduler hooks for concurrency managed workqueue.  Only to be used from
// sched/ and workqueue.c.
//
extern "C" {
    pub fn wq_worker_running(task: *mut task_struct);
}
extern "C" {
    pub fn wq_worker_sleeping(task: *mut task_struct);
}
extern "C" {
    pub fn wq_worker_tick(task: *mut task_struct);
}
extern "C" {
    pub fn wq_worker_last_func(task: *mut task_struct) -> work_func_t;
}
