//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_ialloc.h
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
// Copyright (c) 2000,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Move inodes in clusters of this size
pub const XFS_INODE_BIG_CLUSTER_SIZE: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_icluster {
    pub /: *mut *mut bool deleted; / record is deleted,
    pub /: *mut *mut xfs_ino_t first_ino; / first inode number,
    pub for: *mut *mut uint64_t alloc; / inode phys. allocation bitmap,
// sparse chunks
}

//
// Make an inode pointer out of the buffer/offset.
//
extern "C" {
    pub fn xfs_buf_offset(_arg: b, (mp)->m_sb.sb_inodelog: o <<) -> return;
}
//
// Allocate an inode on disk.  Mode is used to tell whether the new inode will
// need space, and whether it is a directory.
//
// Return the location of the inode in imap, for mapping it into a buffer.
//
// Log specified fields for the ag hdr (inode section)
//

//
// Lookup a record by ino in the btree given by cur.
//
// Get the data from the pointed-to record.
//
extern "C" {
    pub fn xfs_inobt_rec_freecount(irec: *const xfs_inobt_rec_incore) -> u8;
}
//
// Inode chunk initialisation routine
//
extern "C" {
    pub fn xfs_ialloc_cluster_alignment(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_ialloc_setup_geometry(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_ialloc_calc_rootino(mp: *mut xfs_mount, sunit: c_int) -> xfs_ino_t;
}
