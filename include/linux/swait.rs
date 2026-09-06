//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/swait.h
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
// Simple waitqueues are semantically very different to regular wait queues
// (wait.h). The most important difference is that the simple waitqueue allows
// for deterministic behaviour -- IOW it has strictly bounded IRQ and lock hold
// times.
//
// Mainly, this is accomplished by two things. Firstly not allowing swake_up_all
// from IRQ disabled, and dropping the lock upon every wakeup, giving a higher
// priority task a chance to run.
//
// Secondly, we had to drop a fair number of features of the other waitqueue
// code; notably:
//
// - mixing INTERRUPTIBLE and UNINTERRUPTIBLE sleeps on the same waitqueue;
// all wakeups are TASK_NORMAL in order to avoid O(n) lookups for the right
// sleeper state.
//
// - the !exclusive mode; because that leads to O(n) wakeups, everything is
// exclusive. As such swake_up_one will only ever awake _one_ waiter.
//
// - custom wake callback functions; because you cannot give any guarantees
// about random code. This also allows swait to be used in RT, such that
// raw spinlock can be used for the swait queue head.
//
// As a side effect of these; the data structures are slimmer albeit more ad-hoc.
// For all the above, note that simple wait queues should _only_ be used under
// very specific realtime constraints -- it is best to stick with the regular
// wait queues in most cases.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swait_queue_head {
    pub lock: raw_spinlock_t,
    pub task_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swait_queue {
    pub task: *mut task_struct,
    pub task_list: list_head,
}

//
// swait_active -- locklessly test for waiters on the queue
// @wq: the waitqueue to test for waiters
//
// returns true if the wait list is not empty
//
// NOTE: this function is lockless and requires care, incorrect usage _will_
// lead to sporadic and non-obvious failure.
//
// NOTE2: this function has the same above implications as regular waitqueues.
//
// Use either while holding swait_queue_head::lock or when used for wakeups
// with an extra smp_mb() like:
//
// CPU0 - waker                    CPU1 - waiter
//
// for (;;) {
// @cond = true;                     prepare_to_swait_exclusive(&wq_head, &wait, state);
// smp_mb();                         // smp_mb() from set_current_state()
// if (swait_active(wq_head))        if (@cond)
// wake_up(wq_head);                      break;
// schedule();
// }
// finish_swait(&wq_head, &wait);
//
// Because without the explicit smp_mb() it's possible for the
// swait_active() load to get hoisted over the @cond store such that we'll
// observe an empty wait list while the waiter might not observe @cond.
// This, in turn, can trigger missing wakeups.
//
// Also note that this 'optimization' trades a spin_lock() for an smp_mb(),
// which (when the lock is uncontended) are of roughly equal cost.
//
// swq_has_sleeper - check if there are any waiting processes
// @wq: the waitqueue to test for waiters
//
// Returns true if @wq has waiting processes
//
// Please refer to the comment for swait_active.
//
// We need to be sure we are in sync with the list_add()
// modifications to the wait queue (task_list).
//
// This memory barrier should be paired with one on the
// waiting side.
//
extern "C" {
    pub fn swait_active(_arg: wq) -> return;
}
extern "C" {
    pub fn swake_up_one(q: *mut swait_queue_head);
}
extern "C" {
    pub fn swake_up_all(q: *mut swait_queue_head);
}
extern "C" {
    pub fn swake_up_locked(q: *mut swait_queue_head, wake_flags: c_int);
}
extern "C" {
    pub fn prepare_to_swait_exclusive(q: *mut swait_queue_head, wait: *mut swait_queue, state: c_int);
}
extern "C" {
    pub fn prepare_to_swait_event(q: *mut swait_queue_head, wait: *mut swait_queue, state: c_int) -> c_long;
}
extern "C" {
    pub fn __finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue);
}
extern "C" {
    pub fn finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue);
}
// as per ___wait_event() but for swait, therefore "exclusive == 1"

//
// swait_event_idle_exclusive - wait without system load contribution
// @wq: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_IDLE) until the @condition evaluates to
// true. The @condition is checked each time the waitqueue @wq is woken up.
//
// This function is mostly used when a kthread or workqueue waits for some
// condition and doesn't want to contribute to system load. Signals are
// ignored.
//

//
// swait_event_idle_timeout_exclusive - wait up to timeout without load contribution
// @wq: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @timeout: timeout at which we'll give up in jiffies
//
// The process is put to sleep (TASK_IDLE) until the @condition evaluates to
// true. The @condition is checked each time the waitqueue @wq is woken up.
//
// This function is mostly used when a kthread or workqueue waits for some
// condition and doesn't want to contribute to system load. Signals are
// ignored.
//
// Returns:
// 0 if the @condition evaluated to %false after the @timeout elapsed,
// 1 if the @condition evaluated to %true after the @timeout elapsed,
// or the remaining jiffies (at least 1) if the @condition evaluated
// to %true before the @timeout elapsed.
//

