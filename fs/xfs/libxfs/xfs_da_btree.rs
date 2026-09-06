//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_da_btree.h
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
// Copyright (c) 2000,2002,2005 Silicon Graphics, Inc.
// Copyright (c) 2013 Red Hat, Inc.
// All Rights Reserved.
//
// Directory/attribute geometry information. There will be one of these for each
// data fork type, and it will be passed around via the xfs_da_args. Global
// structures will be attached to the xfs_mount.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_da_geometry {
    pub /: *mut *mut unsigned int blksize; / da block size in bytes,
    pub /: *mut *mut unsigned int fsbcount; / da block size in filesystem blocks,
    pub /: *mut *mut uint8_t fsblog; / log2 of _filesystem_ block size,
    pub /: *mut *mut uint8_t blklog; / log2 of da block size,
    pub /: *mut *mut unsigned int node_hdr_size; / danode header size in bytes,
    pub /: *mut *mut unsigned int node_ents; / # of entries in a danode,
    pub /: *mut *mut unsigned int magicpct; / 37% of block size in bytes,
    pub /: *mut *mut xfs_dablk_t datablk; / blockno of dir data v2,
    pub /: *mut *mut unsigned int leaf_hdr_size; / dir2 leaf header size,
    pub /: *mut *mut unsigned int leaf_max_ents; / # of entries in dir2 leaf,
    pub /: *mut *mut xfs_dablk_t leafblk; / blockno of leaf data v2,
    pub /: *mut *mut unsigned int free_hdr_size; / dir2 free header size,
    pub /: *mut *mut unsigned int free_max_bests; / # of bests entries in dir2 free,
    pub /: *mut *mut xfs_dablk_t freeblk; / blockno of free data v2,
    pub /: *mut *mut xfs_extnum_t max_extents; / Max. extents in corresponding fork,
    pub data_first_offset: xfs_dir2_data_aoff_t,
    pub data_entry_offset: usize,
}

// ========================================================================
// Btree searching and modification structure definitions.
// ========================================================================
//
// Search comparison results
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_dacmp {
    XFS_CMP_DIFFERENT,	/* names are completely different */
    XFS_CMP_EXACT,		/* names are exactly the same */
    XFS_CMP_CASE		/* names are same but differ in case */
}

//
// Structure to ease passing around component names.
//
// Operation flags:
//

//
// Storage for holding state during Btree searches and split/join ops.
//
// Only need space for 5 intermediate nodes.  With a minimum of 62-way
// fanout to the Btree, we can support over 900 million directory blocks,
// which is slightly more than enough.
//
// for dirv2 extrablk is data
//
// In-core version of the node header to abstract the differences in the v2 and
// v3 disk format of the headers. Callers need to convert to/from disk format as
// appropriate.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_da3_icnode_hdr {
    pub forw: u32,
    pub back: u32,
    pub magic: u16,
    pub count: u16,
    pub level: u16,
//
// Pointer to the on-disk format entries, which are behind the
// variable size (v4 vs v5) header in the on-disk block.
//
    pub btree: *mut xfs_da_node_entry,
}

//
// Utility macros to aid in logging changed structure fields.
//

// ========================================================================
// Function prototypes.
// ========================================================================
//
// Routines used for growing the Btree.
//
extern "C" {
    pub fn xfs_da3_split(state: *mut xfs_da_state_t) -> c_int;
}
//
// Routines used for shrinking the Btree.
//
extern "C" {
    pub fn xfs_da3_join(state: *mut xfs_da_state_t) -> c_int;
}
//
// Routines used for finding things in the Btree.
//
extern "C" {
    pub fn xfs_da3_node_lookup_int(state: *mut xfs_da_state_t, result: *mut c_int) -> c_int;
}
//
// Utility routines.
//
// Utility routines.
//

extern "C" {
    pub fn xfs_da_grow_inode(args: *mut xfs_da_args_t, new_blkno: *mut xfs_dablk_t) -> c_int;
}
extern "C" {
    pub fn xfs_da_hashname(name_string: *const u8, name_length: c_int) -> c_uint;
}
extern "C" {
    pub fn xfs_da_state_free(state: *mut xfs_da_state_t);
}
extern "C" {
    pub fn xfs_da_state_reset(state: *mut xfs_da_state, args: *mut xfs_da_args);
}
extern "C" {
    pub fn xfs_da3_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
}
extern "C" {
    pub fn xfs_da3_node_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
}
