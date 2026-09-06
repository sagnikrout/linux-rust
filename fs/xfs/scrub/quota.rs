//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/quota.h
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
// Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
extern "C" {
    pub fn xchk_quota_to_dqtype(sc: *mut xfs_scrub) -> xfs_dqtype_t;
}
// dquot iteration code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_dqiter {
    pub sc: *mut xfs_scrub,
// Quota file that we're walking.
    pub quota_ip: *mut xfs_inode,
// Cached data fork mapping for the dquot.
    pub bmap: xfs_bmbt_irec,
// The next dquot to scan.
    pub id: u64,
// Quota type (user/group/project).
    pub dqtype: xfs_dqtype_t,
// Data fork sequence number to detect stale mappings.
    pub if_seq: c_uint,
}

extern "C" {
    pub fn xchk_dquot_iter(cursor: *mut xchk_dqiter, dqpp: *mut xfs_dquot) -> c_int;
}
