//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_extent_busy.h
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
// Copyright (c) 2010 David Chinner.
// Copyright (c) 2011 Christoph Hellwig.
// All Rights Reserved.
//
// Busy block/extent entry.  Indexed by a rbtree in the group to mark blocks
// that have been freed but whose transactions aren't committed to disk yet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_extent_busy {
    pub /: *mut *mut rb_node rb_node; / group by-bno indexed search tree,
    pub /: *mut *mut list_head list; / transaction busy extent list,
    pub group: *mut xfs_group,
    pub bno: xfs_agblock_t,
    pub length: xfs_extlen_t,
    pub flags: c_uint,
pub const XFS_EXTENT_BUSY_DISCARDED: c_uint = 0x01	/* undergoing a discard op. */;
pub const XFS_EXTENT_BUSY_SKIP_DISCARD: c_uint = 0x02	/* do not discard */;
}

//
// List used to track groups of related busy extents all the way through
// to discard completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_busy_extents {
    pub extent_list: list_head,
    pub endio_work: work_struct,
//
// Owner is the object containing the struct xfs_busy_extents to free
// once the busy extents have been processed. If only the
// xfs_busy_extents object needs freeing, then point this at itself.
//
    pub owner: *mut c_void,
}

extern "C" {
    pub fn xfs_extent_busy_clear(list: *mut list_head, do_discard: bool);
}
extern "C" {
    pub fn xfs_extent_busy_wait_all(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_extent_busy_list_empty(xg: *mut xfs_group, busy_gen: *mut c_uint) -> bool;
}
//
// Zoned RTGs don't need to track busy extents, as the actual block freeing only
// happens by a zone reset, which forces out all transactions that touched the
// to be reset zone first.
//

