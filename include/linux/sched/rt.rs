//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/rt.h
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

extern "C" {
    pub fn unlikely(MAX_DL_PRIO: prio < MAX_RT_PRIO && prio >=) -> return;
}
extern "C" {
    pub fn unlikely(MAX_RT_PRIO: prio <) -> return;
}
//
// Returns true if a task has a priority that belongs to RT class. PI-boosted
// tasks will return true. Use rt_policy() to ignore PI-boosted tasks.
//
extern "C" {
    pub fn rt_prio(_arg: p->prio) -> return;
}
//
// Returns true if a task has a priority that belongs to RT or DL classes.
// PI-boosted tasks will return true. Use rt_or_dl_task_policy() to ignore
// PI-boosted tasks.
//
extern "C" {
    pub fn rt_or_dl_prio(_arg: p->prio) -> return;
}
//
// Returns true if a task has a policy that belongs to RT or DL classes.
// PI-boosted tasks will return false.
//

extern "C" {
    pub fn rt_mutex_pre_schedule();
}
extern "C" {
    pub fn rt_mutex_schedule();
}
extern "C" {
    pub fn rt_mutex_post_schedule();
}
//
// Must hold either p->pi_lock or task_rq(p)->lock.
//
extern "C" {
    pub fn rt_mutex_setprio(p: *mut task_struct, pi_task: *mut task_struct);
}
extern "C" {
    pub fn rt_mutex_adjust_pi(p: *mut task_struct);
}

extern "C" {
    pub fn normalize_rt_tasks();
}
//
// default timeslice is 100 msecs (used only for SCHED_RR tasks).
// Timeslices get refilled after they expire.
//

