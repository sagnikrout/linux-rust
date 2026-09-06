//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_inode_util.h
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
// Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
extern "C" {
    pub fn xfs_flags2diflags(ip: *mut xfs_inode, xflags: c_uint) -> u16;
}
extern "C" {
    pub fn xfs_flags2diflags2(ip: *mut xfs_inode, xflags: c_uint) -> u64;
}
extern "C" {
    pub fn xfs_dic2xflags(ip: *mut xfs_inode) -> u32;
}
extern "C" {
    pub fn xfs_ip2xflags(ip: *mut xfs_inode) -> u32;
}
extern "C" {
    pub fn xfs_get_initial_prid(dp: *mut xfs_inode) -> prid_t;
}
//
// File creation context.
//
// Due to our only partial reliance on the VFS to propagate uid and gid values
// according to accepted Unix behaviors, callers must initialize idmap to the
// correct idmapping structure to get the correct inheritance behaviors when
// XFS_MOUNT_GRPID is set.
//
// To create files detached from the directory tree (e.g. quota inodes), set
// idmap to NULL.  To create a tree root, set pip to NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_icreate_args {
    pub idmap: *mut mnt_idmap,
    pub /: *mut *mut *mut xfs_inode pip; / parent inode or null,
    pub rdev: dev_t,
    pub mode: umode_t,

    pub flags: u16,
}

//
// Flags for xfs_trans_ichgtime().
//
pub const XFS_ICHGTIME_MOD: c_uint = 0x1	/* data fork modification timestamp */;
pub const XFS_ICHGTIME_CHG: c_uint = 0x2	/* inode field change timestamp */;
pub const XFS_ICHGTIME_CREATE: c_uint = 0x4	/* inode create timestamp */;
pub const XFS_ICHGTIME_ACCESS: c_uint = 0x8	/* last access timestamp */;
extern "C" {
    pub fn xfs_trans_ichgtime(tp: *mut xfs_trans, ip: *mut xfs_inode, flags: c_int);
}
extern "C" {
    pub fn xfs_iunlink(tp: *mut xfs_trans, ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_droplink(tp: *mut xfs_trans, ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_bumplink(tp: *mut xfs_trans, ip: *mut xfs_inode);
}
