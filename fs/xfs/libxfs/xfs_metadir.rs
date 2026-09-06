//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_metadir.h
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
// Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// Cleanup widget for metadata inode creation and deletion.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_metadir_update {
// Parent directory
    pub dp: *mut xfs_inode,
// Path to metadata file
    pub path: *const c_char,
// Parent pointer update context
    pub ppargs: *mut xfs_parent_args,
// Child metadata file
    pub ip: *mut xfs_inode,
    pub tp: *mut xfs_trans,
    pub metafile_type: xfs_metafile_type,
    pub dp_locked:1: c_uint,
    pub ip_locked:1: c_uint,
}

extern "C" {
    pub fn int(upd: *mut *mut xfs_metadir_createfn)(struct xfs_metadir_update, priv: *mut c_void) -> typedef;
}
extern "C" {
    pub fn xfs_metadir_start_link(upd: *mut xfs_metadir_update) -> c_int;
}
extern "C" {
    pub fn xfs_metadir_link(upd: *mut xfs_metadir_update) -> c_int;
}
extern "C" {
    pub fn xfs_metadir_commit(upd: *mut xfs_metadir_update) -> c_int;
}
