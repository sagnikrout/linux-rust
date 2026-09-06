//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_trans_space.h
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
// Components of space reservations.
//
// Worst case number of bmaps that can be held in a block.

// Worst case number of realtime rmaps that can be held in a block.

// Adding one realtime rmap could split every level to the top of the tree.

// Blocks we might need to add "b" realtime rmaps to a tree.

// Worst case number of rmaps that can be held in a block.

// Adding one rmap could split every level up to the top of the tree.

//
// Note that we historically set m_rmap_maxlevels to 9 when reflink is enabled,
// so we must preserve this behavior to avoid changing the transaction space
// reservations and minimum log size calculations for existing filesystems.
//
pub const XFS_OLD_REFLINK_RMAP_MAXLEVELS: c_int = 9;
// Blocks we might need to add "b" rmaps to a tree.

// Macro flag: #define XFS_NEXTENTADD_SPACE_RES(mp,b,w)\
// Blocks we might need to add "b" mappings & rmappings to a file.
// Macro flag: #define XFS_SWAP_RMAP_SPACE_RES(mp,b,w)\

pub const XFS_DIRENTER_MAX_SPLIT(mp,nl): c_int = 1;

//
// Space reservation values for various transactions.
//

// This macro is not used - see inline code in xfs_attr_set

extern "C" {
    pub fn xfs_create_space_res(mp: *mut xfs_mount, namelen: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_mkdir_space_res(mp: *mut xfs_mount, namelen: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_link_space_res(mp: *mut xfs_mount, namelen: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_remove_space_res(mp: *mut xfs_mount, namelen: c_uint) -> c_uint;
}
