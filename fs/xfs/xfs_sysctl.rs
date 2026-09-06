//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_sysctl.h
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
// Copyright (c) 2001-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

//
// Tunable xfs parameters
//
// xfs_error_level:
//
// How much error reporting will be done when internal problems are
// encountered.  These problems normally return an EFSCORRUPTED to their
// caller, with no other information reported.
//
// 0	No error reports
// 1	Report EFSCORRUPTED errors that will cause a filesystem shutdown
// 5	Report all EFSCORRUPTED errors (all of the above errors, plus any
// additional errors that are known to not cause shutdowns)
//
// xfs_panic_mask bit 0x8 turns the error reports into panics
//
// XFS_REFCACHE_SIZE = 1
// XFS_REFCACHE_PURGE = 2
// XFS_RESTRICT_CHOWN = 3
// XFS_PROBE_DMAPI = 9
// XFS_PROBE_IOOPS = 10
// XFS_PROBE_QUOTA = 11
// XFS_IO_BYPASS = 18
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_globals {

    pub /: *mut *mut int pwork_threads; / parallel workqueue threads,
    pub /: *mut *mut bool larp; / log attribute replay,

    pub /: *mut *mut int bload_leaf_slack; / btree bulk load leaf slack,
    pub /: *mut *mut int bload_node_slack; / btree bulk load node slack,
    pub /: *mut *mut int log_recovery_delay; / log recovery delay (secs),
    pub /: *mut *mut int mount_delay; / mount setup delay (secs),
    pub /: *mut *mut bool bug_on_assert; / BUG() the kernel on assert failure,
    pub /: *mut *mut bool always_cow; / use COW fork for all overwrites,
}

extern "C" {
    pub fn xfs_sysctl_register() -> c_int;
}
extern "C" {
    pub fn xfs_sysctl_unregister();
}

