//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_error.h
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
// Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
extern "C" {
    pub fn xfs_buf_corruption_error(bp: *mut xfs_buf, fa: xfs_failaddr_t);
}
extern "C" {
    pub fn xfs_verifier_error(bp: *mut xfs_buf, error: c_int, failaddr: xfs_failaddr_t);
}

pub const XFS_ERRLEVEL_OFF: c_int = 0;
pub const XFS_ERRLEVEL_LOW: c_int = 1;
pub const XFS_ERRLEVEL_HIGH: c_int = 5;
// Dump 128 bytes of any corrupt buffer

extern "C" {
    pub fn xfs_errortag_init(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_errortag_del(mp: *mut xfs_mount);
}

extern "C" {
    pub fn xfs_errortag_add(mp: *mut xfs_mount, error_tag: c_uint) -> c_int;
}
extern "C" {
    pub fn xfs_errortag_add_name(mp: *mut xfs_mount, tag_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn xfs_errortag_copy(dst_mp: *mut xfs_mount, src_mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_errortag_clearall(mp: *mut xfs_mount) -> c_int;
}

// Macro flag: #define xfs_errortag_del(mp)

//
// XFS panic tags -- allow a call to xfs_alert_tag() be turned into
// a panic by setting fs.xfs.panic_mask in a sysctl.
//

