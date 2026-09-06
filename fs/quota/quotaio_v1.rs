//! Automatically rewritten from C Header to Rust Module
//! Source: fs/quota/quotaio_v1.h
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
// The following constants define the amount of time given a user
// before the soft limits are treated as hard limits (usually resulting
// in an allocation failure). The timer is started when the user crosses
// their soft limit, it is reset when they go below their soft limit.
//

//
// The following structure defines the format of the disk quota file
// (as it appears on disk) - the file is an array of these structures
// indexed by user or group number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v1_disk_dqblk {
    pub /: *mut *mut __u32 dqb_bhardlimit; / absolute limit on disk blks alloc,
    pub /: *mut *mut __u32 dqb_bsoftlimit; / preferred limit on disk blks,
    pub /: *mut *mut __u32 dqb_curblocks; / current block count,
    pub /: *mut *mut __u32 dqb_ihardlimit; / absolute limit on allocated inodes,
    pub /: *mut *mut __u32 dqb_isoftlimit; / preferred inode limit,
    pub /: *mut *mut __u32 dqb_curinodes; / current # allocated inodes,
// below fields differ in length on 32-bit vs 64-bit architectures
    pub /: *mut *mut unsigned long dqb_btime; / time limit for excessive disk use,
    pub /: *mut *mut unsigned long dqb_itime; / time limit for excessive inode use,
}

