//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cgroupstats.h
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


// SPDX-License-Identifier: LGPL-2.1 WITH Linux-syscall-note
// cgroupstats.h - exporting per-cgroup statistics
//
// Copyright IBM Corporation, 2007
// Author Balbir Singh <balbir@linux.vnet.ibm.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of version 2.1 of the GNU Lesser General Public License
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it would be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//

//
// Data shared between user space and kernel space on a per cgroup
// basis. This data is shared using taskstats.
//
// Most of these states are derived by looking at the task->state value
//
// Each member is aligned to a 8 byte boundary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroupstats {
    pub /: *mut *mut __u64 nr_sleeping; / Number of tasks sleeping,
    pub /: *mut *mut __u64 nr_running; / Number of tasks running,
    pub /: *mut *mut __u64 nr_stopped; / Number of tasks in stopped state,
    pub /: *mut *mut __u64 nr_uninterruptible; / Number of tasks in uninterruptible,
// state
    pub /: *mut *mut __u64 nr_io_wait; / Number of tasks waiting on IO,
}

//
// Commands sent from userspace
// Not versioned. New commands should only be inserted at the enum's end
// prior to __CGROUPSTATS_CMD_MAX
//

