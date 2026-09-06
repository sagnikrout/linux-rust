//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/coredump.h
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
// Task dumpability mode.  Gates core dump production and ptrace_attach()
// authorization.  The numeric values are stable ABI (suid_dumpable
// sysctl, prctl(PR_SET_DUMPABLE)); do not renumber.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum task_dumpable {
    TASK_DUMPABLE_OFF	= 0,	/* no dump; ptrace needs CAP_SYS_PTRACE */
    TASK_DUMPABLE_OWNER	= 1,	/* default; dump and ptrace by uid match */
    TASK_DUMPABLE_ROOT	= 2,	/* dump as root; ptrace needs CAP_SYS_PTRACE */
}

extern "C" {
    pub fn task_exec_state_set_dumpable(value: task_dumpable);
}
extern "C" {
    pub fn task_exec_state_get_dumpable(task: *mut task_struct) -> task_dumpable;
}
