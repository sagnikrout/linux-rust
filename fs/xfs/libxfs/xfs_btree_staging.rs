//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_btree_staging.h
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
// Copyright (C) 2020 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//
// Fake root for an AG-rooted btree.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xbtree_afakeroot {
// AG block number of the new btree root.
    pub af_root: xfs_agblock_t,
// Height of the new btree.
    pub af_levels: c_uint,
// Number of blocks used by the btree.
    pub af_blocks: c_uint,
}

// Cursor interactions with fake roots for AG-rooted btrees.
// Fake root for an inode-rooted btree.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xbtree_ifakeroot {
// Fake inode fork.
    pub if_fork: *mut xfs_ifork,
// Number of blocks used by the btree.
    pub if_blocks: i64,
// Height of the new btree.
    pub if_levels: c_uint,
// Number of bytes available for this fork in the inode.
    pub if_fork_size: c_uint,
}

// Cursor interactions with fake roots for inode-rooted btrees.
// Bulk loading of staged btrees.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_btree_bload {
//
// This function will be called to load @nr_wanted records into the
// btree.  The implementation does this by setting the cursor's bc_rec
// field in in-core format and using init_rec_from_cur to set the
// records in the btree block.  Records must be returned in sort order.
// The function must return the number of records loaded or the usual
// negative errno.
//
    pub get_records: xfs_btree_bload_get_records_fn,
//
// This function will be called nr_blocks times to obtain a pointer
// to a new btree block on disk.  Callers must preallocate all space
// for the new btree before calling xfs_btree_bload, and this function
// is what claims that reservation.
//
    pub claim_block: xfs_btree_bload_claim_block_fn,
//
// This function should return the size of the in-core btree root
// block.  It is only necessary for XFS_BTREE_TYPE_INODE btrees.
//
    pub iroot_size: xfs_btree_bload_iroot_size_fn,
//
// The caller should set this to the number of records that will be
// stored in the new btree.
//
    pub nr_records: u64,
//
// Number of free records to leave in each leaf block.  If the caller
// sets this to -1, the slack value will be calculated to be halfway
// between maxrecs and minrecs.  This typically leaves the block 75%
// full.  Note that slack values are not enforced on inode root blocks.
//
    pub leaf_slack: c_int,
//
// Number of free key/ptrs pairs to leave in each node block.  This
// field has the same semantics as leaf_slack.
//
    pub node_slack: c_int,
//
// The xfs_btree_bload_compute_geometry function will set this to the
// number of btree blocks needed to store nr_records records.
//
    pub nr_blocks: u64,
//
// The xfs_btree_bload_compute_geometry function will set this to the
// height of the new btree.
//
    pub btree_height: c_uint,
//
// Flush the new btree block buffer list to disk after this many blocks
// have been formatted.  Zero prohibits writing any buffers until all
// blocks have been formatted.
//
    pub max_dirty: u16,
// Number of dirty buffers.
    pub nr_dirty: u16,
}
