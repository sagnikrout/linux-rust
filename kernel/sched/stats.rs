//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/stats.h
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
// Expects runqueue lock to be held for atomicity of update
//
// Expects runqueue lock to be held for atomicity of update
//

// Force schedstat enabled if a dependent tracepoint is active

extern "C" {
    pub fn psi_task_change(task: *mut task_struct, clear: c_int, set: c_int);
}

extern "C" {
    pub fn psi_account_irqtime(rq: *mut rq, curr: *mut task_struct, prev: *mut task_struct);
}

//
// PSI tracks state that persists across sleeps, such as iowaits and
// memory stalls. As a result, it has to distinguish between sleeps,
// where a task's runnable state changes, and migrations, where a task
// and its runnable state are being moved between CPUs and runqueues.
//
// A notable case is a task whose dequeue is delayed. PSI considers
// those sleeping, but because they are still on the runqueue they can
// go through migration requeues. In this case, *sleeping* states need
// to be transferred.
//
// Same runqueue, nothing changed for psi
// psi_sched_switch() will handle the flags
// CPU migration of "sleeping" task
// CPU migration of runnable task
// Wakeup of new or sleeping task
// Same runqueue, nothing changed for psi
//
// A voluntary sleep is a dequeue followed by a task switch. To
// avoid walking all ancestors twice, psi_task_switch() handles
// TSK_RUNNING and TSK_IOWAIT for us when it moves TSK_ONCPU.
// Do nothing here.
//
// In the SCHED_PROXY_EXECUTION case we may do sleeping
// dequeues that are not followed by a task switch, so check
// TSK_ONCPU is set to ensure the task switch is imminent.
// Otherwise clear the flags as usual.
//
// When migrating a task to another CPU, clear all psi
// state. The enqueue callback above will work it out.
//
// Is the task being migrated during a wakeup? Make sure to
// deregister its sleep-persistent psi states from the old
// queue, and let psi_enqueue() know it has to requeue.
//

//
// We are interested in knowing how long it was from the *first* time a
// task was queued to the time that it finally hit a CPU, we call this routine
// from dequeue_task() to account for possible rq->clock skew across CPUs. The
// delta taken on each CPU would annul the skew.
//
// Called when a task finally hits the CPU.  We can now calculate how
// long it was waiting to run.  We also note when it began so that we
// can keep stats on how long its time-slice is.
//
// This function is only called from enqueue_task(), but also only updates
// the timestamp if it is already not set.  It's assumed that
// sched_info_dequeue() will clear that stamp when appropriate.
//
// Called when a process ceases being the active-running process involuntarily
// due, typically, to expiring its time slice (this may also be called when
// switching to the idle task).  Now we can calculate how long we ran.
// Also, if the process is still in the TASK_RUNNING state, call
// sched_info_enqueue() to mark that it has now again started waiting on
// the runqueue.
//
// Called when tasks are switched involuntarily due, typically, to expiring
// their time slice.  (This may also be called when switching to or from
// the idle task.)  We are only called when prev != next.
//
// prev now departs the CPU.  It's not interesting to record
// stats about how efficient we were at scheduling the idle
// process, however.
//

