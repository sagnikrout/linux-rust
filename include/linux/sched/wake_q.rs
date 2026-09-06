//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/wake_q.h
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
// Wake-queues are lists of tasks with a pending wakeup, whose
// callers have already marked the task as woken internally,
// and can thus carry on. A common use case is being able to
// do the wakeups once the corresponding user lock as been
// released.
//
// We hold reference to each task in the list across the wakeup,
// thus guaranteeing that the memory is still valid by the time
// the actual wakeups are performed in wake_up_q().
//
// One per task suffices, because there's never a need for a task to be
// in two wake queues simultaneously; it is forbidden to abandon a task
// in a wake queue (a call to wake_up_q() _must_ follow), so if a task is
// already in a wake queue, the wakeup will happen soon and the second
// waker can just skip it.
//
// The DEFINE_WAKE_Q macro declares and initializes the list head.
// wake_up_q() does NOT reinitialize the list; it's expected to be
// called near the end of a function. Otherwise, the list can be
// re-initialized for later re-use by wake_q_init().
//
// NOTE that this can cause spurious wakeups. schedule() callers
// must ensure the call is done inside a loop, confirming that the
// wakeup condition has in fact occurred.
//
// NOTE that there is no guarantee the wakeup will happen any later than the
// wake_q_add() location. Therefore task must be ready to be woken at the
// location of the wake_q_add().
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head {
    pub first: *mut wake_q_node,
    pub lastp: *mut wake_q_node,
}

extern "C" {
    pub fn wake_q_add(head: *mut wake_q_head, task: *mut task_struct);
}
extern "C" {
    pub fn wake_q_add_safe(head: *mut wake_q_head, task: *mut task_struct);
}
extern "C" {
    pub fn wake_up_q(head: *mut wake_q_head);
}
// Spin unlock helpers to unlock and call wake_up_q with preempt disabled
