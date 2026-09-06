//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/threads.h
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
// The default limit for the nr of threads is now in
// /proc/sys/kernel/threads-max.
//
// Maximum supported processors.  Setting this smaller saves quite a
// bit of memory.  Use nr_cpu_ids instead of this except for static bitmaps.
//

// FIXME: This should be fixed in the arch's Kconfig
pub const CONFIG_NR_CPUS: c_int = 1;

// Places which use this should consider cpumask_var_t.

pub const MIN_THREADS_LEFT_FOR_ROOT: c_int = 4;
//
// This controls the default maximum pid allocated to a process
//

//
// A maximum of 4 million PIDs should be enough for a while.
// [NOTE: PID/TIDs are limited to 2^30 ~= 1 billion, see FUTEX_TID_MASK.]
//

//
// Define a minimum number of pids per cpu.  Heuristically based
// on original pid max of 32k for 32 cpus.  Also, increase the
// minimum settable value for pid_max on the running system based
// on similar defaults.  See kernel/pid.c:pid_idr_init() for details.
//
pub const PIDS_PER_CPU_DEFAULT: c_int = 1024;
pub const PIDS_PER_CPU_MIN: c_int = 8;
