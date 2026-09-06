//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/freezer.h
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
// Freezer declarations

// Macro flag: #define FREEZER_H_INCLUDED

//
// Timeout for stopping processes
//
// Check if a process has been frozen for PM or cgroup1 freezer. Note that
// cgroup2 freezer uses the job control mechanism and does not interact with
// the PM freezer.
//
extern "C" {
    pub fn frozen(p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn freezing_slow_path(p: *mut task_struct) -> bool;
}
//
// Check if there is a request to freeze a task from PM or cgroup1 freezer.
// Note that cgroup2 freezer uses the job control mechanism and does not
// interact with the PM freezer.
//
extern "C" {
    pub fn freezing_slow_path(_arg: p) -> return;
}
// Takes and releases task alloc lock using task_lock()
extern "C" {
    pub fn __thaw_task(t: *mut task_struct);
}
extern "C" {
    pub fn __refrigerator(check_kthr_stop: bool) -> bool;
}
extern "C" {
    pub fn freeze_processes() -> c_int;
}
extern "C" {
    pub fn freeze_kernel_threads() -> c_int;
}
extern "C" {
    pub fn thaw_processes();
}
extern "C" {
    pub fn thaw_kernel_threads();
}
extern "C" {
    pub fn thaw_process(p: *mut task_struct);
}
extern "C" {
    pub fn __refrigerator(_arg: false) -> return;
}
extern "C" {
    pub fn freeze_task(p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn set_freezable() -> bool;
}

extern "C" {
    pub fn cgroup1_freezing(task: *mut task_struct) -> bool;
}

