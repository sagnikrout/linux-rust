//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/debug.h
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
// Various scheduler/task debugging interfaces:
//
extern "C" {
    pub fn dump_cpu_task(cpu: c_int);
}
//
// Only dump TASK_* tasks. (0 for all tasks)
//
extern "C" {
    pub fn show_state_filter(state_filter: c_uint);
}
extern "C" {
    pub fn show_regs(: *mut pt_regs);
}
//
// TASK is a pointer to the task whose backtrace we want to see (or NULL for current
// task), SP is the stack pointer of the first frame that should be shown in the back
// trace (or NULL if the entire call-chain of the task should be shown).
//
extern "C" {
    pub fn sched_show_task(p: *mut task_struct);
}
extern "C" {
    pub fn proc_sched_set_task(p: *mut task_struct);
}
// Attach to any functions which should be ignored in wchan output.

// Linker adds these: start and end of __sched functions
// Is this address in the __sched functions?
extern "C" {
    pub fn in_sched_functions(addr: c_ulong) -> c_int;
}
