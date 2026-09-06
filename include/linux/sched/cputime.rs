//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/cputime.h
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
// cputime accounting APIs:
//

extern "C" {
    pub fn task_gtime(t: *mut task_struct) -> u64;
}

// utime = t->utime;
// stime = t->stime;

// utimescaled = t->utimescaled;
// stimescaled = t->stimescaled;

extern "C" {
    pub fn task_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64);
}
extern "C" {
    pub fn thread_group_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64);
}
//
// Thread group CPU time accounting.
//
extern "C" {
    pub fn thread_group_cputime(tsk: *mut task_struct, times: *mut task_cputime);
}
extern "C" {
    pub fn thread_group_sample_cputime(tsk: *mut task_struct, samples: *mut u64);
}
//
// The following are functions that support scheduler-internal time accounting.
// These functions are generally called at the timer tick.  None of this depends
// on CONFIG_SCHEDSTATS.
//
// get_running_cputimer - return &tsk->signal->cputimer if cputimers are active
//
// @tsk:	Pointer to target task.
//

//
// Check whether posix CPU timers are active. If not the thread
// group accounting is not active either. Lockless check.
//
// After we flush the task's sum_exec_runtime to sig->sum_sched_runtime
// in __exit_signal(), we won't account to the signal struct further
// cputime consumed by that task, even though the task can still be
// ticking after __exit_signal().
//
// In order to keep a consistent behaviour between thread group cputime
// and thread group cputimer accounting, lets also ignore the cputime
// elapsing after __exit_signal() in any thread group timer running.
//
// This makes sure that POSIX CPU clocks and timers are synchronized, so
// that a POSIX CPU timer won't expire while the corresponding POSIX CPU
// clock delta is behind the expiring timer value.
//

//
// account_group_user_time - Maintain utime for a thread group.
//
// @tsk:	Pointer to task structure.
// @cputime:	Time value by which to increment the utime field of the
// thread_group_cputime structure.
//
// If thread group time is being maintained, get the structure for the
// running CPU and update the utime field there.
//
// account_group_system_time - Maintain stime for a thread group.
//
// @tsk:	Pointer to task structure.
// @cputime:	Time value by which to increment the stime field of the
// thread_group_cputime structure.
//
// If thread group time is being maintained, get the structure for the
// running CPU and update the stime field there.
//
// account_group_exec_runtime - Maintain exec runtime for a thread group.
//
// @tsk:	Pointer to task structure.
// @ns:		Time value by which to increment the sum_exec_runtime field
// of the thread_group_cputime structure.
//
// If thread group time is being maintained, get the structure for the
// running CPU and update the sum_exec_runtime field there.
//

extern "C" {
    pub fn dummy_steal_clock(cpu: c_int) -> u64;
}
extern "C" {
    pub fn static_call(_arg: pv_steal_clock)(cpu) -> return;
}

