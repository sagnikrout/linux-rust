//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_trans_resv.h
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
// structure for maintaining pre-calculated transaction reservations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_trans_res {
    pub /: *mut *mut uint tr_logres; / log space unit in bytes per log ticket,
    pub /: *mut *mut int tr_logcount; / number of log operations per log ticket,
    pub indicating: *mut *mut int tr_logflags; / log flags, currently only used for,
// a reservation request is permanent or not
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_trans_resv {
    pub /: *mut *mut xfs_trans_res tr_write; / extent alloc trans,
    pub /: *mut *mut xfs_trans_res tr_itruncate; / truncate trans,
    pub /: *mut *mut xfs_trans_res tr_rename; / rename trans,
    pub /: *mut *mut xfs_trans_res tr_link; / link trans,
    pub /: *mut *mut xfs_trans_res tr_remove; / unlink trans,
    pub /: *mut *mut xfs_trans_res tr_symlink; / symlink trans,
    pub /: *mut *mut xfs_trans_res tr_create; / create trans,
    pub /: *mut *mut xfs_trans_res tr_create_tmpfile; / create O_TMPFILE trans,
    pub /: *mut *mut xfs_trans_res tr_mkdir; / mkdir trans,
    pub /: *mut *mut xfs_trans_res tr_ifree; / inode free trans,
    pub /: *mut *mut xfs_trans_res tr_ichange; / inode update trans,
    pub /: *mut *mut xfs_trans_res tr_growdata; / fs data section grow trans,
    pub /: *mut *mut xfs_trans_res tr_addafork; / add inode attr fork trans,
    pub /: *mut *mut xfs_trans_res tr_writeid; / write setuid/setgid file,
    pub buffer: *mut *mut xfs_trans_res tr_attrinval; / attr fork,
// invalidation
    pub at: *mut *mut xfs_trans_res tr_attrsetm; / set/create an attribute,
// mount time
    pub at: *mut *mut xfs_trans_res tr_attrsetrt; / set/create an attribute,
// runtime
    pub /: *mut *mut xfs_trans_res tr_attrrm; / remove an attribute,
    pub /: *mut *mut xfs_trans_res tr_clearagi; / clear agi unlinked bucket,
    pub /: *mut *mut xfs_trans_res tr_growrtalloc; / grow realtime allocations,
    pub /: *mut *mut xfs_trans_res tr_growrtzero; / grow realtime zeroing,
    pub /: *mut *mut xfs_trans_res tr_growrtfree; / grow realtime freeing,
    pub /: *mut *mut xfs_trans_res tr_qm_setqlim; / adjust quota limits,
    pub /: *mut *mut xfs_trans_res tr_qm_dqalloc; / allocate quota on disk,
    pub /: *mut *mut xfs_trans_res tr_sb; / modify superblock,
    pub /: *mut *mut xfs_trans_res tr_fsyncts; / update timestamps on fsync,
    pub /: *mut *mut xfs_trans_res tr_atomic_ioend; / untorn write completion,
}

// shorthand way of accessing reservation structure

//
// Per-directory log reservation for any directory change.
// dir blocks: (1 btree block per level + data block + free block) * dblock size
// bmap btree: (levels + 2) * max depth * block size
// v2 directory blocks can be fragmented below the dirblksize down to the fsb
// size, so account for that in the DAENTER macros.
//

//
// Various log count values.
//
pub const XFS_DEFAULT_LOG_COUNT: c_int = 1;
pub const XFS_DEFAULT_PERM_LOG_COUNT: c_int = 2;
pub const XFS_ITRUNCATE_LOG_COUNT: c_int = 2;
pub const XFS_INACTIVE_LOG_COUNT: c_int = 2;
pub const XFS_CREATE_LOG_COUNT: c_int = 2;
pub const XFS_CREATE_TMPFILE_LOG_COUNT: c_int = 2;
pub const XFS_MKDIR_LOG_COUNT: c_int = 3;
pub const XFS_SYMLINK_LOG_COUNT: c_int = 3;
pub const XFS_REMOVE_LOG_COUNT: c_int = 2;
pub const XFS_LINK_LOG_COUNT: c_int = 2;
pub const XFS_RENAME_LOG_COUNT: c_int = 2;
pub const XFS_WRITE_LOG_COUNT: c_int = 2;
pub const XFS_ADDAFORK_LOG_COUNT: c_int = 2;
pub const XFS_ATTRINVAL_LOG_COUNT: c_int = 1;
pub const XFS_ATTRSET_LOG_COUNT: c_int = 3;
pub const XFS_ATTRRM_LOG_COUNT: c_int = 3;
//
// Original log operation counts were overestimated in the early days of
// reflink.  These are retained here purely for minimum log size calculations
// and must not be used for runtime reservations.
//
pub const XFS_ITRUNCATE_LOG_COUNT_REFLINK: c_int = 8;
pub const XFS_WRITE_LOG_COUNT_REFLINK: c_int = 8;
extern "C" {
    pub fn xfs_trans_resv_calc(mp: *mut xfs_mount, resp: *mut xfs_trans_resv);
}
extern "C" {
    pub fn xfs_allocfree_block_count(mp: *mut xfs_mount, num_ops: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_calc_itruncate_reservation_minlogsize(mp: *mut xfs_mount) -> c_uint;
}
extern "C" {
    pub fn xfs_calc_write_reservation_minlogsize(mp: *mut xfs_mount) -> c_uint;
}
extern "C" {
    pub fn xfs_calc_qm_dqalloc_reservation_minlogsize(mp: *mut xfs_mount) -> c_uint;
}
extern "C" {
    pub fn xfs_calc_max_atomic_write_fsblocks(mp: *mut xfs_mount) -> xfs_extlen_t;
}
