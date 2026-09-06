//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ioprio.h
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
// Default IO priority.
//

//
// Check that a priority value has a valid class.
//
// if process has set io priority explicitly, use that. if not, convert
// the cpu scheduler nice value to an io priority
//
// This is for the case where the task hasn't asked for a specific IO class.
// Check for idle and rt task process, and return appropriate IO class.
//

//
// If the task has set an I/O priority, use that. Otherwise, return
// the default I/O priority.
//
// Expected to be called for current task or with task_lock() held to keep
// io_context stable.
//

extern "C" {
    pub fn __get_task_ioprio(_arg: current) -> return;
}
extern "C" {
    pub fn set_task_ioprio(task: *mut task_struct, ioprio: c_int) -> c_int;
}

extern "C" {
    pub fn ioprio_check_cap(ioprio: c_int) -> c_int;
}

