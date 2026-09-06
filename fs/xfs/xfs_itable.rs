//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_itable.h
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
// Copyright (c) 2000-2001 Silicon Graphics, Inc.  All Rights Reserved.
//
// In-memory representation of a userspace request for batch inode data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_ibulk {
    pub mp: *mut xfs_mount,
    pub idmap: *mut mnt_idmap,
    pub /: *mut *mut *mut void __user ubuffer; / user output buffer,
    pub /: *mut *mut xfs_ino_t startino; / start with this inode,
    pub /: *mut *mut unsigned int icount; / number of elements in ubuffer,
    pub /: *mut *mut unsigned int ocount; / number of records returned,
    pub /: *mut *mut *mut unsigned int flags; / XFS_IBULK_FLAG_,
    pub /: *mut *mut *mut unsigned int iwalk_flags; / XFS_IWALK_FLAG_,
}

// Fill out the bs_extents64 field if set.

// Signal that we can return metadata directories.

//
// Advance the user buffer pointer by one record of the given size.  If the
// buffer is now full, return the appropriate error code.
//
// Return stat information in bulk (by-inode) for the filesystem.
//
// Return codes for the formatter function are 0 to continue iterating, and
// non-zero to stop iterating.  Any non-zero value will be passed up to the
// bulkstat/inumbers caller.  The special value -ECANCELED can be used to stop
// iteration, as neither bulkstat nor inumbers will ever generate that error
// code on their own.
//
extern "C" {
    pub fn xfs_bulkstat_one(breq: *mut xfs_ibulk, formatter: bulkstat_one_fmt_pf) -> c_int;
}
extern "C" {
    pub fn xfs_bulkstat(breq: *mut xfs_ibulk, formatter: bulkstat_one_fmt_pf) -> c_int;
}
extern "C" {
    pub fn xfs_inumbers(breq: *mut xfs_ibulk, formatter: inumbers_fmt_pf) -> c_int;
}
