//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/unwind_deferred.h
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

// Set if the unwinding was used (directly or deferred)
extern "C" {
    pub fn unwind_task_init(task: *mut task_struct);
}
extern "C" {
    pub fn unwind_task_free(task: *mut task_struct);
}
extern "C" {
    pub fn unwind_user_faultable(trace: *mut unwind_stacktrace) -> c_int;
}
extern "C" {
    pub fn unwind_deferred_init(work: *mut unwind_work, func: unwind_callback_t) -> c_int;
}
extern "C" {
    pub fn unwind_deferred_request(work: *mut unwind_work, cookie: *mut u64) -> c_int;
}
extern "C" {
    pub fn unwind_deferred_cancel(work: *mut unwind_work);
}
extern "C" {
    pub fn unwind_deferred_task_exit(task: *mut task_struct);
}
// Was there any unwinding?
// Is a task_work going to run again before going back

