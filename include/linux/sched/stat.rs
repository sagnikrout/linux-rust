//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/stat.h
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
// Various counters maintained by the scheduler and fork(),
// exposed via /proc, sys.c or used by drivers via these APIs.
//
// ( Note that all these values are acquired without locking,
// so they can only be relied on in narrow circumstances. )
//
extern "C" {
    pub fn nr_processes() -> c_int;
}
extern "C" {
    pub fn nr_running() -> c_uint;
}
extern "C" {
    pub fn single_task_running() -> bool;
}
extern "C" {
    pub fn nr_iowait() -> c_uint;
}
extern "C" {
    pub fn nr_iowait_cpu(cpu: c_int) -> c_uint;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_SCHED_INFO) -> return;
}

extern "C" {
    pub fn force_schedstat_enabled();
}

