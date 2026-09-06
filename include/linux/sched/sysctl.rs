//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/sysctl.h
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

// used for hung_task and block/

// Avoid need for ifdefs elsewhere in the code

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sched_tunable_scaling {
    SCHED_TUNABLESCALING_NONE,
    SCHED_TUNABLESCALING_LOG,
    SCHED_TUNABLESCALING_LINEAR,
    SCHED_TUNABLESCALING_END,
}

pub const NUMA_BALANCING_DISABLED: c_uint = 0x0;
pub const NUMA_BALANCING_NORMAL: c_uint = 0x1;
pub const NUMA_BALANCING_MEMORY_TIERING: c_uint = 0x2;

pub const sysctl_numa_balancing_mode: c_int = 0;

