//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/deadline.h
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
// SCHED_DEADLINE tasks has negative priorities, reflecting
// the fact that any of them has higher prio than RT and
// NORMAL/BATCH tasks.
//

extern "C" {
    pub fn unlikely(MAX_DL_PRIO: prio <) -> return;
}
//
// Returns true if a task has a priority that belongs to DL class. PI-boosted
// tasks will return true. Use dl_policy() to ignore PI-boosted tasks.
//
extern "C" {
    pub fn dl_prio(_arg: p->prio) -> return;
}
extern "C" {
    pub fn dl_add_task_root_domain(p: *mut task_struct);
}
extern "C" {
    pub fn dl_clear_root_domain(rd: *mut root_domain);
}
extern "C" {
    pub fn dl_clear_root_domain_cpu(cpu: c_int);
}
//
// Return whether moving DL task @p to @new_mask requires moving DL
// bandwidth accounting between root domains. This helper is specific to
// DL bandwidth move accounting semantics and is shared by
// cpuset_can_attach() and set_cpus_allowed_dl() so both paths use the
// same source root-domain test.
//
extern "C" {
    pub fn dl_bw_visited(cpu: c_int, cookie: u64) -> bool;
}
extern "C" {
    pub fn container_of(_arg: dl_se, task_struct: struct, _arg: dl) -> return;
}
//
// Regarding the deadline, a task with implicit deadline has a relative
// deadline == relative period. A task with constrained deadline has a
// relative deadline <= relative period.
//
// We support constrained deadline tasks. However, there are some restrictions
// applied only for tasks which do not have an implicit deadline. See
// update_dl_entity() to know more about such restrictions.
//
// The dl_is_implicit() returns true if the task has an implicit deadline.
//
