//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sync_file.h
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


//
// include/linux/sync_file.h
//
// Copyright (C) 2012 Google, Inc.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

//
// struct sync_file - sync file to export to the userspace
// @file:		file representing this fence
// @sync_file_list:	membership in global file list
// @wq:			wait queue for fence signaling
// @flags:		flags for the sync_file
// @fence:		fence with the fences in the sync_file
// @cb:			fence callback information
//
// flags:
// POLL_ENABLED: whether userspace is currently poll()'ing or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_file {
    pub file: *mut file,
//
// @user_name:
//
// Name of the sync file provided by userspace, for merged fences.
// Otherwise generated through driver callbacks (in which case the
// entire array is 0).
//
    pub user_name: [c_char; 32],
    pub sync_file_list: list_head,

    pub wq: wait_queue_head_t,
    pub flags: c_ulong,
    pub fence: *mut dma_fence,
    pub cb: dma_fence_cb,
}

pub const POLL_ENABLED: c_int = 0;
