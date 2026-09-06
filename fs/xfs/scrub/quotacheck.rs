//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/quotacheck.h
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
// Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// Quota counters for live quotacheck.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xqcheck_dquot {
// block usage count
    pub bcount: i64,
// inode usage count
    pub icount: i64,
// realtime block usage count
    pub rtbcount: i64,
// Record state
    pub flags: c_uint,
}

//
// This incore dquot record has been written at least once.  We never want to
// store an xqcheck_dquot that looks uninitialized.
//

// Already checked this dquot.

// Already repaired this dquot.

// Live quotacheck control structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xqcheck {
    pub sc: *mut xfs_scrub,
// Shadow dquot counter data.
    pub ucounts: *mut xfarray,
    pub gcounts: *mut xfarray,
    pub pcounts: *mut xfarray,
// Lock protecting quotacheck count observations
    pub lock: mutex,
    pub iscan: xchk_iscan,
// Hooks into the quota code.
    pub qhook: xfs_dqtrx_hook,
// Shadow quota delta tracking structure.
    pub shadow_dquot_acct: rhashtable,
}

// Return the incore counter array for a given quota type.
