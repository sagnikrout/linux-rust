//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_bmap.h
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
// Copyright (c) 2000-2006 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Argument structure for xfs_bmap_alloc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_bmalloca {
    pub /: *mut *mut *mut xfs_trans tp; / transaction pointer,
    pub /: *mut *mut *mut xfs_inode ip; / incore inode pointer,
    pub /: *mut *mut xfs_bmbt_irec prev; / extent before the new one,
    pub /: *mut *mut xfs_bmbt_irec got; / extent after, or delayed,
    pub /: *mut *mut xfs_fileoff_t offset; / offset in file filling in,
    pub /: *mut *mut xfs_extlen_t length; / i/o length asked/allocated,
    pub /: *mut *mut xfs_fsblock_t blkno; / starting block of new extent,
    pub /: *mut *mut *mut xfs_btree_cur cur; / btree cursor,
    pub /: *mut *mut xfs_iext_cursor icur; / incore extent cursor,
    pub /: *mut *mut int nallocs;/ number of extents alloc'd,
    pub /: *mut *mut int logflags;/ flags for transaction logging,
    pub /: *mut *mut xfs_extlen_t total; / total blocks needed for xaction,
    pub /: *mut *mut xfs_extlen_t minlen; / minimum allocation size (blocks),
    pub /: *mut *mut xfs_extlen_t minleft; / amount must be left after alloc,
    pub /: *mut *mut bool eof; / set if allocating past last extent,
    pub /: *mut *mut bool wasdel; / replacing a delayed allocation,
    pub /: *mut *mut bool aeof; / allocated space at eof,
    pub /: *mut *mut bool conv; / overwriting unwritten extents,
    pub /: *mut *mut int datatype;/ data type being allocated,
    pub flags: u32,
}

pub const XFS_BMAP_MAX_NMAP: c_int = 4;
//
// Flags for xfs_bmapi_
//

//
// unwritten extent conversion - this needs write cache flushing and no additional
// allocation alignments. When specified with XFS_BMAPI_PREALLOC it converts
// from written to unwritten, otherwise convert from unwritten to written.
//

//
// allocate zeroed extents - this requires all newly allocated user data extents
// to be initialised to zero. It will be ignored if XFS_BMAPI_METADATA is set.
// Use in conjunction with XFS_BMAPI_CONVERT to convert unwritten extents found
// during the allocation range to zeroed written extents.
//

//
// Map the inode offset to the block given in ap->firstblock.  Primarily
// used for reflink.  The range must be in a hole, and this flag cannot be
// turned on with PREALLOC or CONVERT, and cannot be used on the attr fork.
//
// For bunmapi, this flag unmaps the range without adjusting quota, reducing
// refcount, or freeing the blocks.
//

// Map something in the CoW fork.

// Skip online discard of freed extents

// Do not update the rmap btree.  Used for reconstructing bmbt from rmapbt.

// Try to align allocations to the extent size hint

extern "C" {
    pub fn xfs_bmap_alloc_account(ap: *mut xfs_bmalloca);
}
//
// Special values for xfs_bmbt_irec_t br_startblock field.
//

//
// Flags for xfs_bmap_add_extent*.
//

// Return true if the extent is an allocated extent, written or not.
//
// Return true if the extent is a real, allocated extent, or false if it is  a
// delayed allocation, and unwritten extent or a hole.
//
// Check the mapping for obviously garbage allocations that could trash the
// filesystem immediately.
//

extern "C" {
    pub fn xfs_bmap_compute_attr_offset(mp: *mut xfs_mount) -> c_uint;
}
extern "C" {
    pub fn xfs_bmap_compute_maxlevels(mp: *mut xfs_mount, whichfork: c_int);
}
extern "C" {
    pub fn xfs_default_attroffset(ip: *mut xfs_inode) -> c_uint;
}
extern "C" {
    pub fn xfs_bmap_worst_indlen(ip: *mut xfs_inode, len: xfs_filblks_t) -> xfs_filblks_t;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_bmap_intent_type {
    XFS_BMAP_MAP = 1,
    XFS_BMAP_UNMAP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_bmap_intent {
    pub bi_list: list_head,
    pub bi_type: xfs_bmap_intent_type,
    pub bi_whichfork: c_int,
    pub bi_owner: *mut xfs_inode,
    pub bi_group: *mut xfs_group,
    pub bi_bmap: xfs_bmbt_irec,
}

extern "C" {
    pub fn xfs_bmap_finish_one(tp: *mut xfs_trans, bi: *mut xfs_bmap_intent) -> c_int;
}
extern "C" {
    pub fn xfs_bmap_intent_init_cache() -> int __init;
}
extern "C" {
    pub fn xfs_bmap_intent_destroy_cache();
}
extern "C" {
    pub fn xfs_get_extsz_hint(ip: *mut xfs_inode) -> xfs_extlen_t;
}
extern "C" {
    pub fn xfs_get_cowextsz_hint(ip: *mut xfs_inode) -> xfs_extlen_t;
}
