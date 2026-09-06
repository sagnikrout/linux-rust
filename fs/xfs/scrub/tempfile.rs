//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/tempfile.h
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

extern "C" {
    pub fn xrep_tempfile_create(sc: *mut xfs_scrub, mode: u16) -> c_int;
}
extern "C" {
    pub fn xrep_tempfile_rele(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xrep_tempfile_adjust_directory_tree(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_tempfile_iolock_nowait(sc: *mut xfs_scrub) -> bool;
}
extern "C" {
    pub fn xrep_tempfile_iolock_polled(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_tempfile_iounlock(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xrep_tempfile_ilock(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xrep_tempfile_ilock_nowait(sc: *mut xfs_scrub) -> bool;
}
extern "C" {
    pub fn xrep_tempfile_iunlock(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xrep_tempfile_iunlock_both(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xrep_tempfile_ilock_both(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xrep_tempfile_set_isize(sc: *mut xfs_scrub, isize: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn xrep_tempfile_roll_trans(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_tempfile_copyout_local(sc: *mut xfs_scrub, whichfork: c_int);
}
extern "C" {
    pub fn xrep_is_tempfile(ip: *const xfs_inode) -> bool;
}

