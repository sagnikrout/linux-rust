//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sync_file.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
//
// Copyright (C) 2012 Google, Inc.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

//
// struct sync_merge_data - SYNC_IOC_MERGE: merge two fences
// @name:	name of new fence
// @fd2:	file descriptor of second fence
// @fence:	returns the fd of the new fence to userspace
// @flags:	merge_data flags
// @pad:	padding for 64-bit alignment, should always be zero
//
// Creates a new fence containing copies of the sync_pts in both
// the calling fd and sync_merge_data.fd2.  Returns the new fence's
// fd in sync_merge_data.fence
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_merge_data {
    pub name: [c_char; 32],
    pub fd2: __s32,
    pub fence: __s32,
    pub flags: __u32,
    pub pad: __u32,
}

//
// struct sync_fence_info - detailed fence information
// @obj_name:		name of parent sync_timeline
// @driver_name:	name of driver implementing the parent
// @status:		status of the fence 0:active 1:signaled <0:error
// @flags:		fence_info flags
// @timestamp_ns:	timestamp of status change in nanoseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_fence_info {
    pub obj_name: [c_char; 32],
    pub driver_name: [c_char; 32],
    pub status: __s32,
    pub flags: __u32,
    pub timestamp_ns: __u64,
}

//
// struct sync_file_info - SYNC_IOC_FILE_INFO: get detailed information on a sync_file
// @name:	name of fence
// @status:	status of fence. 1: signaled 0:active <0:error
// @flags:	sync_file_info flags
// @num_fences:	number of fences in the sync_file
// @pad:	padding for 64-bit alignment, should always be zero
// @sync_fence_info: pointer to array of struct &sync_fence_info with all
// fences in the sync_file
//
// Takes a struct sync_file_info. If num_fences is 0, the field is updated
// with the actual number of fences. If num_fences is > 0, the system will
// use the pointer provided on sync_fence_info to return up to num_fences of
// struct sync_fence_info, with detailed fence information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_file_info {
    pub name: [c_char; 32],
    pub status: __s32,
    pub flags: __u32,
    pub num_fences: __u32,
    pub pad: __u32,
    pub sync_fence_info: __u64,
}

//
// struct sync_set_deadline - SYNC_IOC_SET_DEADLINE - set a deadline hint on a fence
// @deadline_ns: absolute time of the deadline
// @pad:	must be zero
//
// Allows userspace to set a deadline on a fence, see &dma_fence_set_deadline
//
// The timebase for the deadline is CLOCK_MONOTONIC (same as vblank).  For
// example
//
// clock_gettime(CLOCK_MONOTONIC, &t);
// deadline_ns = (t.tv_sec * 1000000000L) + t.tv_nsec + ns_until_deadline
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_set_deadline {
    pub deadline_ns: __u64,
// Not strictly needed for alignment but gives some possibility
// for future extension:
//
    pub pad: __u64,
}

//
// Opcodes  0, 1 and 2 were burned during a API change to avoid users of the
// old API to get weird errors when trying to handling sync_files. The API
// change happened during the de-stage of the Sync Framework when there was
// no upstream users available.
//

