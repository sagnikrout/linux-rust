//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_shared.h
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
// Copyright (c) 2013 Red Hat, Inc.
// All Rights Reserved.
//
// Definitions shared between kernel and userspace that don't fit into any other
// header file that is shared with userspace.
//
// Buffer verifier operations are widely used, including userspace tools
//
// btree ops

// log size calculation functions
extern "C" {
    pub fn xfs_log_calc_unit_res(mp: *mut xfs_mount, unit_bytes: c_int) -> c_int;
}
extern "C" {
    pub fn xfs_log_calc_minimum_size(: *mut xfs_mount) -> c_int;
}
//
// Values for t_flags.
//
// Transaction needs to be logged

// Superblock is dirty and needs to be logged

// Transaction took a permanent log reservation

// Synchronous transaction commit needed

// Transaction can use reserve block pool

// Transaction should avoid VFS level superblock write accounting

// Transaction has freed blocks returned to it's reservation

// Transaction contains an intent done log item

//
// LOWMODE is used by the allocator to activate the lowspace algorithm - when
// free space is running low the extent allocator may choose to allocate an
// extent from an AG without leaving sufficient space for a btree split when
// inserting the new extent. In this case the allocator will enable the
// lowspace algorithm which is supposed to allow further allocations (such as
// btree splits and newroots) to allocate from sequential AGs. In order to
// avoid locking AGs out of order the lowspace algorithm will start searching
// for free space from AG 0. If the correct transaction reservations have been
// made then this algorithm will eventually find all the space it needs.
//

// Transaction has locked the rtbitmap and rtsum inodes

//
// Field values for xfs_trans_mod_sb.
//
pub const XFS_TRANS_SB_ICOUNT: c_uint = 0x00000001;
pub const XFS_TRANS_SB_IFREE: c_uint = 0x00000002;
pub const XFS_TRANS_SB_FDBLOCKS: c_uint = 0x00000004;
pub const XFS_TRANS_SB_RES_FDBLOCKS: c_uint = 0x00000008;
pub const XFS_TRANS_SB_FREXTENTS: c_uint = 0x00000010;
pub const XFS_TRANS_SB_RES_FREXTENTS: c_uint = 0x00000020;
pub const XFS_TRANS_SB_DBLOCKS: c_uint = 0x00000040;
pub const XFS_TRANS_SB_AGCOUNT: c_uint = 0x00000080;
pub const XFS_TRANS_SB_IMAXPCT: c_uint = 0x00000100;
pub const XFS_TRANS_SB_REXTSIZE: c_uint = 0x00000200;
pub const XFS_TRANS_SB_RBMBLOCKS: c_uint = 0x00000400;
pub const XFS_TRANS_SB_RBLOCKS: c_uint = 0x00000800;
pub const XFS_TRANS_SB_REXTENTS: c_uint = 0x00001000;
pub const XFS_TRANS_SB_REXTSLOG: c_uint = 0x00002000;
pub const XFS_TRANS_SB_RGCOUNT: c_uint = 0x00004000;
//
// Here we centralize the specification of XFS meta-data buffer reference count
// values.  This determines how hard the buffer cache tries to hold onto the
// buffer.
//
pub const XFS_AGF_REF: c_int = 4;
pub const XFS_AGI_REF: c_int = 4;
pub const XFS_AGFL_REF: c_int = 3;
pub const XFS_INO_BTREE_REF: c_int = 3;
pub const XFS_ALLOC_BTREE_REF: c_int = 2;
pub const XFS_BMAP_BTREE_REF: c_int = 2;
pub const XFS_RMAP_BTREE_REF: c_int = 2;
pub const XFS_DIR_BTREE_REF: c_int = 2;
pub const XFS_INO_REF: c_int = 2;
pub const XFS_ATTR_BTREE_REF: c_int = 1;
pub const XFS_DQUOT_REF: c_int = 1;
pub const XFS_REFC_BTREE_REF: c_int = 1;
pub const XFS_SSB_REF: c_int = 0;
// Computed inode geometry for the filesystem.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_ino_geometry {
// Maximum inode count in this filesystem.
    pub maxicount: u64,
// Actual inode cluster buffer size, in bytes.
    pub inode_cluster_size: c_uint,
//
// Desired inode cluster buffer size, in bytes.  This value is not
// rounded up to at least one filesystem block, which is necessary for
// the sole purpose of validating sb_spino_align.  Runtime code must
// only ever use inode_cluster_size.
//
    pub inode_cluster_size_raw: c_uint,
// Inode cluster sizes, adjusted to be at least 1 fsb.
    pub inodes_per_cluster: c_uint,
    pub blocks_per_cluster: c_uint,
// Inode cluster alignment.
    pub cluster_align: c_uint,
    pub cluster_align_inodes: c_uint,
    pub /: *mut *mut unsigned int inoalign_mask; / mask sb_inoalignmt if used,
    pub /: *mut *mut unsigned int inobt_mxr[2]; / max inobt btree records,
    pub /: *mut *mut unsigned int inobt_mnr[2]; / min inobt btree records,
    pub /: *mut *mut unsigned int inobt_maxlevels; / max inobt btree levels.,
// Size of inode allocations under normal operation.
    pub ialloc_inos: c_uint,
    pub ialloc_blks: c_uint,
// Minimum inode blocks for a sparse allocation.
    pub ialloc_min_blks: c_uint,
// stripe unit inode alignment
    pub ialloc_align: c_uint,
    pub /: *mut *mut unsigned int agino_log; / #bits for agino in inum,
// precomputed default inode attribute fork offset
    pub attr_fork_offset: c_uint,
// precomputed value for di_flags2
    pub new_diflags2: u64,
// minimum folio order of a page cache allocation
    pub min_folio_order: c_uint,
}
