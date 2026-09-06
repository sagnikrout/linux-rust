//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/findparent.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrep_parent_scan_info {
    pub sc: *mut xfs_scrub,
// Inode scan cursor.
    pub iscan: xchk_iscan,
// Hook to capture directory entry updates.
    pub dhook: xfs_dir_hook,
// Lock protecting parent_ino.
    pub lock: mutex,
// Parent inode that we've found.
    pub parent_ino: xfs_ino_t,
    pub lookup_parent: bool,
}

extern "C" {
    pub fn __xrep_findparent_scan_start(_arg: sc, _arg: pscan, _arg: NULL) -> return;
}
extern "C" {
    pub fn xrep_findparent_scan(pscan: *mut xrep_parent_scan_info) -> c_int;
}
extern "C" {
    pub fn xrep_findparent_scan_teardown(pscan: *mut xrep_parent_scan_info);
}
extern "C" {
    pub fn xrep_findparent_confirm(sc: *mut xfs_scrub, parent_ino: *mut xfs_ino_t) -> c_int;
}
extern "C" {
    pub fn xrep_findparent_self_reference(sc: *mut xfs_scrub) -> xfs_ino_t;
}
extern "C" {
    pub fn xrep_findparent_from_dcache(sc: *mut xfs_scrub) -> xfs_ino_t;
}
