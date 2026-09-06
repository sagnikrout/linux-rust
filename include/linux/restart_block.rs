//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/restart_block.h
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
// Common syscall restarting data
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum timespec_type {
    TT_NONE		= 0,
    TT_NATIVE	= 1,
    TT_COMPAT	= 2,
}

//
// System call restart block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct restart_block {
    pub arch_data: c_ulong,
    pub ): *mut *mut long (fn)(struct restart_block,
// For futex_wait()
    pub uaddr: *mut u32 __user,
    pub val: u32,
    pub flags: u32,
    pub bitset: u32,
    pub time: ktime_t,
    pub uaddr2: *mut u32 __user,
    pub futex: },
// For nanosleep
    pub clockid: clockid_t,
    pub type: timespec_type,
    pub rmtp: *mut __kernel_timespec __user,
    pub compat_rmtp: *mut old_timespec32 __user,
}

// For poll
extern "C" {
    pub fn do_no_restart_syscall(parm: *mut restart_block) -> c_long;
}
