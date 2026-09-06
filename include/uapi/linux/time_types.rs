//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/time_types.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_timespec {
    pub /: *mut *mut __kernel_time64_t tv_sec; / seconds,
    pub /: *mut *mut long long tv_nsec; / nanoseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_itimerspec {
    pub /: *mut *mut __kernel_timespec it_interval; / timer period,
    pub /: *mut *mut __kernel_timespec it_value; / timer expiration,
}

//
// legacy timeval structure, only embedded in structures that
// traditionally used 'timeval' to pass time intervals (not absolute
// times). Do not add new users. If user space fails to compile
// here, this is probably because it is not y2038 safe and needs to
// be changed to use another interface.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_old_timeval {
    pub tv_sec: __kernel_long_t,
    pub tv_usec: __kernel_long_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_old_timespec {
    pub /: *mut *mut __kernel_old_time_t tv_sec; / seconds,
    pub /: *mut *mut __kernel_long_t tv_nsec; / nanoseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_old_itimerval {
    pub /: *mut *mut __kernel_old_timeval it_interval;/ timer interval,
    pub /: *mut *mut __kernel_old_timeval it_value; / current value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_sock_timeval {
    pub tv_sec: __s64,
    pub tv_usec: __s64,
}
