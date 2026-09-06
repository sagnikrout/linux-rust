//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/stats.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_stats_run {
    pub scrub_ns: u64,
    pub repair_ns: u64,
    pub retries: c_uint,
    pub repair_attempted: bool,
    pub repair_succeeded: bool,
}

extern "C" {
    pub fn xchk_global_stats_setup(parent: *mut dentry) -> int __init;
}
extern "C" {
    pub fn xchk_global_stats_teardown();
}
extern "C" {
    pub fn xchk_mount_stats_alloc(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xchk_mount_stats_free(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xchk_stats_register(cs: *mut xchk_stats, parent: *mut dentry);
}
extern "C" {
    pub fn xchk_stats_unregister(cs: *mut xchk_stats);
}
//
// If the system doesn't have a high enough resolution clock, charge at
// least one nanosecond so that our stats don't report instantaneous
// runtimes.
//

