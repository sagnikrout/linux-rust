//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/types.h
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
// struct task_cputime - collected CPU time counts
// @stime:		time spent in kernel mode, in nanoseconds
// @utime:		time spent in user mode, in nanoseconds
// @sum_exec_runtime:	total time spent on the CPU, in nanoseconds
//
// This structure groups together three kinds of CPU time that are tracked for
// threads and thread groups.  Most things considering CPU time want to group
// these counts together and treat all three of them in parallel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_cputime {
    pub stime: u64,
    pub utime: u64,
    pub sum_exec_runtime: c_ulonglong,
}
