//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/orphanage.h
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
    pub fn xrep_orphanage_create(sc: *mut xfs_scrub) -> c_int;
}
//
// If we're doing a repair, ensure that the orphanage exists and attach it to
// the scrub context.
//
// If the orphanage can't be found or isn't a directory, we'll
// keep going, but we won't be able to attach the file to the
// orphanage if we can't find the parent.
//
extern "C" {
    pub fn xrep_orphanage_iolock_two(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_orphanage_ilock(sc: *mut xfs_scrub, ilock_flags: c_uint);
}
extern "C" {
    pub fn xrep_orphanage_iunlock(sc: *mut xfs_scrub, ilock_flags: c_uint);
}
extern "C" {
    pub fn xrep_orphanage_rele(sc: *mut xfs_scrub);
}
// Information about a request to add a file to the orphanage.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrep_adoption {
    pub sc: *mut xfs_scrub,
// Name used for the adoption.
    pub xname: *mut xfs_name,
// Parent pointer context tracking
    pub ppargs: xfs_parent_args,
// Block reservations for orphanage and child (if directory).
    pub orphanage_blkres: c_uint,
    pub child_blkres: c_uint,
//
// Does the caller want us to bump the child link count?  This is not
// needed when reattaching files that have become disconnected but have
// nlink > 1.  It is necessary when changing the directory tree
// structure.
//
    pub bump_child_nlink:1: bool,
}

extern "C" {
    pub fn xrep_orphanage_can_adopt(sc: *mut xfs_scrub) -> bool;
}
extern "C" {
    pub fn xrep_adoption_move(adopt: *mut xrep_adoption) -> c_int;
}
extern "C" {
    pub fn xrep_adoption_trans_roll(adopt: *mut xrep_adoption) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrep_adoption {

