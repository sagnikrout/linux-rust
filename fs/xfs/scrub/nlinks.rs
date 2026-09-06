//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/nlinks.h
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
// Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// Live link count control structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_nlink_ctrs {
    pub sc: *mut xfs_scrub,
// Shadow link count data and its mutex.
    pub nlinks: *mut xfarray,
    pub lock: mutex,
//
// The collection step uses a separate iscan context from the compare
// step because the collection iscan coordinates live updates to the
// observation data while this scanner is running.  The compare iscan
// is secondary and can be reinitialized as needed.
//
    pub collect_iscan: xchk_iscan,
    pub compare_iscan: xchk_iscan,
//
// Hook into directory updates so that we can receive live updates
// from other writer threads.
//
    pub dhook: xfs_dir_hook,
// Orphanage reparenting request.
    pub adoption: xrep_adoption,
// Directory entry name, plus the trailing null.
    pub xname: xfs_name,
    pub namebuf: [c_char; MAXNAMELEN],
}

//
// In-core link counts for a given inode in the filesystem.
//
// For an empty rootdir, the directory entries and the field to which they are
// accounted are as follows:
//
// Root directory:
//
// . points to self		(root.child)
// .. points to self		(root.parent)
// f1 points to a child file	(f1.parent)
// d1 points to a child dir	(d1.parent, root.child)
//
// Subdirectory d1:
//
// . points to self		(d1.child)
// .. points to root dir	(root.backref)
// f2 points to child file	(f2.parent)
// f3 points to root.f1		(f1.parent)
//
// root.nlink == 3 (root.dot, root.dotdot, root.d1)
// d1.nlink == 2 (root.d1, d1.dot)
// f1.nlink == 2 (root.f1, d1.f3)
// f2.nlink == 1 (d1.f2)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_nlink {
// Count of forward links from parent directories to this file.
    pub parents: xfs_nlink_t,
//
// Count of back links to this parent directory from child
// subdirectories.
//
    pub backrefs: xfs_nlink_t,
//
// Count of forward links from this directory to all child files and
// the number of dot entries.  Should be zero for non-directories.
//
    pub children: xfs_nlink_t,
// Record state flags
    pub flags: c_uint,
}

//
// This incore link count has been written at least once.  We never want to
// store an xchk_nlink that looks uninitialized.
//

// Already checked this link count record.

// Already made a repair with this link count record.

// Compute total link count, using large enough variables to detect overflow.
// Add one link count for the dot entry of any linked directory.
