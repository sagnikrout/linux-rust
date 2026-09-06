//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/percpu-rwsem.h
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
pub struct percpu_rw_semaphore {
    pub rss: rcu_sync,
    pub read_count: *mut unsigned int __percpu,
    pub writer: rcuwait,
    pub waiters: wait_queue_head_t,
    pub block: core::sync::atomic::AtomicI32,

    pub dep_map: lockdep_map,

}

// Macro flag: #define __PERCPU_RWSEM_DEP_MAP_INIT(lockname)

extern "C" {
    pub fn __percpu_down_read(: *mut percpu_rw_semaphore, _arg: bool, _arg: bool) -> bool;
}
//
// We are in an RCU-sched read-side critical section, so the writer
// cannot both change sem->state from readers_fast and start checking
// counters while we are here. So if we see !sem->state, we know that
// the writer won't be checking until we're past the preempt_enable()
// and that once the synchronize_rcu() is done, the writer will see
// anything we did within this RCU-sched read-size critical section.
//
// The preempt_enable() prevents the compiler from
// bleeding the critical section out.
//
// Same as in percpu_down_read().
//
// The barrier() from preempt_enable() prevents the compiler from
// bleeding the critical section out.
//
extern "C" {
    pub fn __percpu_up_read(sem: *mut percpu_rw_semaphore);
}
//
// Same as in percpu_down_read().
//
extern "C" {
    pub fn percpu_is_read_locked(: *mut percpu_rw_semaphore) -> bool;
}
extern "C" {
    pub fn percpu_down_write(: *mut percpu_rw_semaphore);
}
extern "C" {
    pub fn percpu_up_write(: *mut percpu_rw_semaphore);
}
extern "C" {
    pub fn atomic_read(_arg: &sem->block) -> return;
}
extern "C" {
    pub fn percpu_free_rwsem(: *mut percpu_rw_semaphore);
}

