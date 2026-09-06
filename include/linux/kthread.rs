//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kthread.h
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
// Simple interface for creating and stopping kernel threads without mess.

// opaque kthread data
//
// When "(p->flags & PF_KTHREAD)" is set the task is a kthread and will
// always remain a kthread.  For kthreads p->worker_private always
// points to a struct kthread.  For tasks that are not kthreads
// p->worker_private is used to point to other things.
//
// Return NULL for any task that is not a kthread.
//
// kthread_create - create a kthread on the current node
// @threadfn: the function to run in the thread
// @data: data pointer for @threadfn()
// @namefmt: printf-style format string for the thread name
// @arg: arguments for @namefmt.
//
// This macro will create a kthread on the current node, leaving it in
// the stopped state.  This is just a helper for kthread_create_on_node();
// see the documentation there for more details.
//

extern "C" {
    pub fn get_kthread_comm(buf: *mut c_char, buf_size: usize, tsk: *mut task_struct);
}
extern "C" {
    pub fn set_kthread_struct(p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn kthread_set_per_cpu(k: *mut task_struct, cpu: c_int);
}
extern "C" {
    pub fn kthread_is_per_cpu(k: *mut task_struct) -> bool;
}
//
// kthread_run - create and wake a thread.
// @threadfn: the function to run until signal_pending(current).
// @data: data ptr for @threadfn.
// @namefmt: printf-style name for the thread.
//
// Description: Convenient wrapper for kthread_create() followed by
// wake_up_process().  Returns the kthread or ERR_PTR(-ENOMEM).
//

//
// kthread_run_on_cpu - create and wake a cpu bound thread.
// @threadfn: the function to run until signal_pending(current).
// @data: data ptr for @threadfn.
// @cpu: The cpu on which the thread should be bound,
// @namefmt: printf-style name for the thread. Format is restricted
// to "name.*%u". Code fills in cpu number.
//
// Description: Convenient wrapper for kthread_create_on_cpu()
// followed by wake_up_process().  Returns the kthread or
// ERR_PTR(-ENOMEM).
//
extern "C" {
    pub fn free_kthread_struct(k: *mut task_struct);
}
extern "C" {
    pub fn kthread_bind(k: *mut task_struct, cpu: c_uint);
}
extern "C" {
    pub fn kthread_bind_mask(k: *mut task_struct, mask: *const cpumask);
}
extern "C" {
    pub fn kthread_affine_preferred(p: *mut task_struct, mask: *const cpumask) -> c_int;
}
extern "C" {
    pub fn kthread_stop(k: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn kthread_stop_put(k: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn kthread_should_stop() -> bool;
}
extern "C" {
    pub fn kthread_should_park() -> bool;
}
extern "C" {
    pub fn kthread_should_stop_or_park() -> bool;
}
extern "C" {
    pub fn kthread_freezable_should_stop(was_frozen: *mut bool) -> bool;
}
extern "C" {
    pub fn kthread_park(k: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn kthread_unpark(k: *mut task_struct);
}
extern "C" {
    pub fn kthread_parkme();
}

extern "C" {
    pub fn kthreads_update_housekeeping() -> c_int;
}
extern "C" {
    pub fn kthread_do_exit(: *mut kthread, _arg: c_long);
}
extern "C" {
    pub fn kthreadd(unused: *mut c_void) -> c_int;
}
extern "C" {
    pub fn tsk_fork_get_node(tsk: *mut task_struct) -> c_int;
}
//
// Simple work processor based on kthread.
//
// This provides easier way to make use of kthreads.  A kthread_work
// can be queued and flushed using queue/kthread_flush_work()
// respectively.  Queued kthread_works are processed by a kthread
// running kthread_worker_fn().
//
extern "C" {
    pub fn void(work: *mut *mut kthread_work_func_t)(struct kthread_work) -> typedef;
}
extern "C" {
    pub fn kthread_delayed_work_timer_fn(t: *mut timer_list);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kthread_worker {
    pub flags: c_uint,
    pub lock: raw_spinlock_t,
    pub work_list: list_head,
    pub delayed_work_list: list_head,
    pub task: *mut task_struct,
    pub current_work: *mut kthread_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kthread_work {
    pub node: list_head,
    pub func: kthread_work_func_t,
    pub worker: *mut kthread_worker,
// Number of canceling calls that are running at the moment.
    pub canceling: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kthread_delayed_work {
    pub work: kthread_work,
    pub timer: timer_list,
}

extern "C" {
    pub fn kthread_worker_fn(worker_ptr: *mut c_void) -> c_int;
}

//
// kthread_run_worker - create and wake a kthread worker.
// @flags: flags modifying the default behavior of the worker
// @namefmt: printf-style name for the thread.
//
// Description: Convenient wrapper for kthread_create_worker() followed by
// wake_up_process().  Returns the kthread_worker or ERR_PTR(-ENOMEM).
//

//
// kthread_run_worker_on_cpu - create and wake a cpu bound kthread worker.
// @cpu: CPU number
// @flags: flags modifying the default behavior of the worker
// @namefmt: printf-style name for the thread. Format is restricted
// to "name.*%u". Code fills in cpu number.
//
// Description: Convenient wrapper for kthread_create_worker_on_cpu()
// followed by wake_up_process().  Returns the kthread_worker or
// ERR_PTR(-ENOMEM).
//
extern "C" {
    pub fn kthread_flush_work(work: *mut kthread_work);
}
extern "C" {
    pub fn kthread_flush_worker(worker: *mut kthread_worker);
}
extern "C" {
    pub fn kthread_cancel_work_sync(work: *mut kthread_work) -> bool;
}
extern "C" {
    pub fn kthread_cancel_delayed_work_sync(work: *mut kthread_delayed_work) -> bool;
}
extern "C" {
    pub fn kthread_destroy_worker(worker: *mut kthread_worker);
}
extern "C" {
    pub fn kthread_use_mm(mm: *mut mm_struct);
}
extern "C" {
    pub fn kthread_unuse_mm(mm: *mut mm_struct);
}

extern "C" {
    pub fn kthread_associate_blkcg(css: *mut cgroup_subsys_state);
}

