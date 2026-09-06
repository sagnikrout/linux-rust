//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/resource.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// Resource control/accounting header file for linux
//
// Definition of struct rusage taken from BSD 4.3 Reno
//
// We don't support all of these yet, but we might as well have them....
// Otherwise, each time we add new items, programs which depend on this
// structure will lose.  This reduces the chances of that happening.
//
pub const RUSAGE_SELF: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rusage {
    pub /: *mut *mut __kernel_old_timeval ru_utime; / user time used,
    pub /: *mut *mut __kernel_old_timeval ru_stime; / system time used,
    pub /: *mut *mut __kernel_long_t ru_maxrss; / maximum resident set size,
    pub /: *mut *mut __kernel_long_t ru_ixrss; / integral shared memory size,
    pub /: *mut *mut __kernel_long_t ru_idrss; / integral unshared data size,
    pub /: *mut *mut __kernel_long_t ru_isrss; / integral unshared stack size,
    pub /: *mut *mut __kernel_long_t ru_minflt; / page reclaims,
    pub /: *mut *mut __kernel_long_t ru_majflt; / page faults,
    pub /: *mut *mut __kernel_long_t ru_nswap; / swaps,
    pub /: *mut *mut __kernel_long_t ru_inblock; / block input operations,
    pub /: *mut *mut __kernel_long_t ru_oublock; / block output operations,
    pub /: *mut *mut __kernel_long_t ru_msgsnd; / messages sent,
    pub /: *mut *mut __kernel_long_t ru_msgrcv; / messages received,
    pub /: *mut *mut __kernel_long_t ru_nsignals; / signals received,
    pub /: *mut *mut __kernel_long_t ru_nvcsw; / voluntary context switches,
    pub /: *mut *mut __kernel_long_t ru_nivcsw; / involuntary ",
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlimit64 {
    pub rlim_cur: __u64,
    pub rlim_max: __u64,
}

pub const PRIO_MAX: c_int = 20;
pub const PRIO_PROCESS: c_int = 0;
pub const PRIO_PGRP: c_int = 1;
pub const PRIO_USER: c_int = 2;
//
// Limit the stack by to some sane default: root can always
// increase this limit if needed..  8MB seems reasonable.
//

//
// Limit the amount of locked memory by some sane default:
// root can always increase this limit if needed.
//
// The main use-cases are (1) preventing sensitive memory
// from being swapped; (2) real-time operations; (3) via
// IOURING_REGISTER_BUFFERS.
//
// The first two don't need much. The latter will take as
// much as it can get. 8MB is a reasonably sane default.
//

//
// Due to binary compatibility, the actual resource numbers
// may be different for different linux versions..
//

