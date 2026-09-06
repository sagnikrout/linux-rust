//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_types.h
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
// Copyright (c) 2000-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// New verifiers will return the instruction address of the failing check.
// NULL means everything is ok.
//
pub type xfs_failaddr_t = *mut c_void;
//
// Null values for the types.
//

//
// Minimum and maximum blocksize and sectorsize.
// The blocksize upper limit is pretty much arbitrary.
// The sectorsize upper limit is due to sizeof(sb_sectsize).
// CRC enable filesystems use 512 byte inodes, meaning 512 byte block sizes
// cannot be used.
//

//
// Inode fork identifiers.
//

//
// Min numbers of data/attr fork btree root pointers.
//
pub const MINDBTPTRS: c_int = 3;
pub const MINABTPTRS: c_int = 2;
//
// MAXNAMELEN is the length (including the terminating null) of
// the longest permissible file (component) name.
//
pub const MAXNAMELEN: c_int = 256;
//
// This enum is used in string mapping in xfs_trace.h; please keep the
// TRACE_DEFINE_ENUMs for it up to date.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_name {
    pub name: *const c_uchar,
    pub len: c_int,
    pub type: c_int,
}

//
// uid_t and gid_t are hard-coded to 32 bits in the inode.
// Hence, an 'id' in a dquot is 32 bits..
//
pub type xfs_dqid_t = u32;
//
// Constants for bit manipulations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_iext_cursor {
    pub leaf: *mut xfs_iext_leaf,
    pub pos: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_refc_domain {
    XFS_REFC_DOMAIN_SHARED = 0,
    XFS_REFC_DOMAIN_COW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_refcount_irec {
    pub /: *mut *mut xfs_agblock_t rc_startblock; / starting block number,
    pub /: *mut *mut xfs_extlen_t rc_blockcount; / count of free blocks,
    pub /: *mut *mut xfs_nlink_t rc_refcount; / number of inodes linked here,
    pub /: *mut *mut xfs_refc_domain rc_domain; / shared or cow staging extent?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rmap_irec {
    pub /: *mut *mut xfs_agblock_t rm_startblock; / extent start block,
    pub /: *mut *mut xfs_extlen_t rm_blockcount; / extent length,
    pub /: *mut *mut uint64_t rm_owner; / extent owner,
    pub /: *mut *mut uint64_t rm_offset; / offset within the owner,
    pub /: *mut *mut unsigned int rm_flags; / state flags,
}

// per-AG block reservation types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_ag_resv_type {
    XFS_AG_RESV_NONE = 0,
    XFS_AG_RESV_AGFL,
    XFS_AG_RESV_METADATA,
    XFS_AG_RESV_RMAPBT,

//
// Don't increase fdblocks when freeing extent.  This is a pony for
// the bnobt repair functions to re-free the free space without
// altering fdblocks.  If you think you need this you're wrong.
//
    XFS_AG_RESV_IGNORE,

//
// This allocation activity is being done on behalf of a metadata file.
// These files maintain their own permanent space reservations and are
// required to adjust fdblocks using the xfs_metafile_resv_* helpers.
//
    XFS_AG_RESV_METAFILE,
}

// Results of scanning a btree keyspace to check occupancy.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xbtree_recpacking {
// None of the keyspace maps to records.
    XBTREE_RECPACKING_EMPTY = 0,

// Some, but not all, of the keyspace maps to records.
    XBTREE_RECPACKING_SPARSE,

// The entire keyspace maps to records.
    XBTREE_RECPACKING_FULL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_group_type {
    XG_TYPE_AG,
    XG_TYPE_RTG,
    XG_TYPE_MAX,
    } __packed;

    { XG_TYPE_AG,	"ag" }, \
    { XG_TYPE_RTG,	"rtg" }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_free_counter {
//
// Number of free blocks on the data device.
//
    XC_FREE_BLOCKS,

//
// Number of free RT extents on the RT device.
//
    XC_FREE_RTEXTENTS,

//
// Number of available for use RT extents.
//
// This counter only exists for zoned RT device and indicates the number
// of RT extents that can be directly used by writes.  XC_FREE_RTEXTENTS
// also includes blocks that have been written previously and freed, but
// sit in a rtgroup that still needs a zone reset.
//
    XC_FREE_RTAVAILABLE,
    XC_FREE_NR,
}

//
// Type verifier functions
//
extern "C" {
    pub fn xfs_verify_fsbno(mp: *mut xfs_mount, fsbno: xfs_fsblock_t) -> bool;
}
extern "C" {
    pub fn xfs_verify_ino(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool;
}
extern "C" {
    pub fn xfs_is_sb_inum(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool;
}
extern "C" {
    pub fn xfs_verify_dir_ino(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool;
}
extern "C" {
    pub fn xfs_verify_rtbno(mp: *mut xfs_mount, rtbno: xfs_rtblock_t) -> bool;
}
extern "C" {
    pub fn xfs_verify_icount(mp: *mut xfs_mount, icount: c_ulonglong) -> bool;
}
extern "C" {
    pub fn xfs_verify_dablk(mp: *mut xfs_mount, off: xfs_fileoff_t) -> bool;
}
extern "C" {
    pub fn xfs_verify_fileoff(mp: *mut xfs_mount, off: xfs_fileoff_t) -> bool;
}
