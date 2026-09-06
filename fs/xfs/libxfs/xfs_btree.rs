//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_btree.h
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
// Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Generic key, ptr and record wrapper structures.
//
// These are disk format structures, and are converted where necessary
// by the btree specific code that needs to interpret them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union xfs_btree_ptr {
    pub /: *mut *mut __be32 s; / short form ptr,
    pub /: *mut *mut __be64 l; / long form ptr,
}

//
// The in-core btree key.  Overlapping btrees actually store two keys
// per pointer, so we reserve enough memory to hold both.  The __*bigkey
// items should never be accessed directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union xfs_btree_key {
    pub bmbt: xfs_bmbt_key,
    pub /: *mut *mut xfs_bmdr_key_t bmbr; / bmbt root block,
    pub alloc: xfs_alloc_key_t,
    pub inobt: xfs_inobt_key,
    pub rmap: xfs_rmap_key,
    pub __rmap_bigkey: [xfs_rmap_key; 2],
    pub refc: xfs_refcount_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xfs_btree_rec {
    pub bmbt: xfs_bmbt_rec,
    pub /: *mut *mut xfs_bmdr_rec_t bmbr; / bmbt root block,
    pub alloc: xfs_alloc_rec,
    pub inobt: xfs_inobt_rec,
    pub rmap: xfs_rmap_rec,
    pub refc: xfs_refcount_rec,
}

//
// This nonsense is to make -wlint happy.
//

extern "C" {
    pub fn xfs_btree_magic(mp: *mut xfs_mount, ops: *const xfs_btree_ops) -> u32;
}
//
// For logging record fields.
//

pub const XFS_BB_NUM_BITS: c_int = 5;

pub const XFS_BB_NUM_BITS_CRC: c_int = 9;

//
// Generic stats interface
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xbtree_key_contig {
    XBTREE_KEY_GAP = 0,
    XBTREE_KEY_CONTIGUOUS,
    XBTREE_KEY_OVERLAP,
}

//
// Decide if these two numeric btree key fields are contiguous, overlapping,
// or if there's a gap between them.  @x should be the field from the high
// key and @y should be the field from the low key.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_btree_type {
    XFS_BTREE_TYPE_AG,
    XFS_BTREE_TYPE_INODE,
    XFS_BTREE_TYPE_MEM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_btree_ops {
    pub name: *const c_char,
// Type of btree - AG-rooted or inode-rooted
    pub type: xfs_btree_type,
// XFS_BTGEO_* flags that determine the geometry of the btree
    pub geom_flags: c_uint,
// size of the key, pointer, and record structures
    pub key_len: usize,
    pub ptr_len: usize,
    pub rec_len: usize,
// LRU refcount to set on each btree buffer created
    pub lru_refs: c_uint,
// offset of btree stats array
    pub statoff: c_uint,
// sick mask for health reporting (not for bmap btrees)
    pub sick_mask: c_uint,
// cursor operations
    pub ): *mut *mut *mut xfs_btree_cur (dup_cursor)(xfs_btree_cur,
    pub dst): *mut xfs_btree_cur,
// update btree root pointer
    pub level_change): *const *const xfs_btree_ptr nptr, int,
// block allocation / freeing
    pub stat): *mut c_int,
    pub bp): *mut *mut *mut int (free_block)(struct xfs_btree_cur cur, struct xfs_buf,
// records in block/level
    pub level): *mut *mut *mut int (get_minrecs)(struct xfs_btree_cur cur, int,
    pub level): *mut *mut *mut int (get_maxrecs)(struct xfs_btree_cur cur, int,
// records on disk.  Matter for the root in inode case.
    pub level): *mut *mut *mut int (get_dmaxrecs)(struct xfs_btree_cur cur, int,
// init values of btree structures
    pub rec): *const xfs_btree_rec,
    pub rec): *mut xfs_btree_rec,
    pub ptr): *mut xfs_btree_ptr,
    pub rec): *const xfs_btree_rec,
//
// Compare key value and cursor value -- positive if key > cur,
// negative if key < cur, and zero if equal.
//
    pub key): *const xfs_btree_key,
//
// Compare key1 and key2 -- positive if key1 > key2, negative if
// key1 < key2, and zero if equal.  If the @mask parameter is non NULL,
// each key field to be used in the comparison must contain a nonzero
// value.
//
    pub mask): *const xfs_btree_key,
    pub buf_ops: *const xfs_buf_ops,
// check that k1 is lower than k2
    pub k2): *const xfs_btree_key,
// check that r1 is lower than r2
    pub r2): *const xfs_btree_rec,
//
// Are these two btree keys immediately adjacent?
//
// Given two btree keys @key1 and @key2, decide if it is impossible for
// there to be a third btree key K satisfying the relationship
// @key1 < K < @key2.  To determine if two btree records are
// immediately adjacent, @key1 should be the high key of the first
// record and @key2 should be the low key of the second record.
// If the @mask parameter is non NULL, each key field to be used in the
// comparison must contain a nonzero value.
//
    pub mask): *const xfs_btree_key,
//
// Reallocate the space for if_broot to fit the number of records.
// Move the records and pointers in if_broot to fit the new size.  When
// shrinking this will eliminate holes between the records and pointers
// created by the caller.  When growing this will create holes to be
// filled in by the caller.
//
// The caller must not request to add more records than would fit in
// the on-disk inode root.  If the if_broot is currently NULL, then if
// we are adding records, one will be allocated.  The caller must also
// not request that the number of records go below zero, although it
// can go to zero.
//
    pub new_numrecs): c_uint,
}

// btree geometry flags

#[repr(C)]
#[derive(Copy, Clone)]
pub union xfs_btree_irec {
    pub a: xfs_alloc_rec_incore,
    pub b: xfs_bmbt_irec,
    pub i: xfs_inobt_rec_incore,
    pub r: xfs_rmap_irec,
    pub rc: xfs_refcount_irec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_btree_level {
// buffer pointer
    pub bp: *mut xfs_buf,
// key/record number
    pub ptr: u16,
// readahead info

    pub ra: u16,
}

//
// Btree cursor structure.
// This collects all information needed by the btree code in one place.
//
// per-type information
// per-format private data
// Must be at the end of the struct!
//
// Compute the size of a btree cursor that can handle a btree of a given
// height.  The bc_levels array handles node and leaf blocks, so its size
// is exactly nlevels.
//
extern "C" {
    pub fn struct_size_t(xfs_btree_cur: struct, _arg: bc_levels, _arg: nlevels) -> return;
}
// cursor state flags
//
// The root of this btree is a fakeroot structure so that we can stage a btree
// rebuild without leaving it accessible via primary metadata.  The ops struct
// is dynamically allocated and must be freed when the cursor is deleted.
//

// We are converting a delalloc reservation (only for bmbt btrees)

// For extent swap, ignore owner check in verifier (only for bmbt btrees)

// Cursor is active (only for allocbt btrees)

pub const XFS_BTREE_NOERROR: c_int = 0;
pub const XFS_BTREE_ERROR: c_int = 1;
//
// Convert from buffer to btree block header.
//

//
// Check that block header is ok.
//
// Delete the btree cursor.
//
// Duplicate the btree cursor.
// Allocate a new one, copy the record, re-get the buffers.
//
// Compute first and last byte offsets for the fields given.
// Interprets the offsets table, which contains struct field offsets.
//
// Initialise a new btree block header
//
// Common btree core entry points.
//
extern "C" {
    pub fn xfs_btree_increment(: *mut xfs_btree_cur, _arg: c_int, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn xfs_btree_decrement(: *mut xfs_btree_cur, _arg: c_int, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn xfs_btree_lookup(: *mut xfs_btree_cur, _arg: xfs_lookup_t, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn xfs_btree_update(: *mut xfs_btree_cur, : *mut xfs_btree_rec) -> c_int;
}
extern "C" {
    pub fn xfs_btree_new_iroot(: *mut xfs_btree_cur, : *mut c_int, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn xfs_btree_insert(: *mut xfs_btree_cur, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn xfs_btree_delete(: *mut xfs_btree_cur, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn xfs_btree_get_rec(: *mut xfs_btree_cur, : *mut xfs_btree_rec, : *mut c_int) -> c_int;
}
//
// btree block CRC helpers
//
extern "C" {
    pub fn xfs_btree_fsblock_calc_crc(: *mut xfs_buf);
}
extern "C" {
    pub fn xfs_btree_fsblock_verify_crc(: *mut xfs_buf) -> bool;
}
extern "C" {
    pub fn xfs_btree_agblock_calc_crc(: *mut xfs_buf);
}
extern "C" {
    pub fn xfs_btree_agblock_verify_crc(: *mut xfs_buf) -> bool;
}
//
// Internal btree helpers also used by xfs_bmap.c.
//
extern "C" {
    pub fn xfs_btree_log_block(: *mut xfs_btree_cur, : *mut xfs_buf, _arg: u32);
}
extern "C" {
    pub fn xfs_btree_log_recs(: *mut xfs_btree_cur, : *mut xfs_buf, _arg: c_int, _arg: c_int);
}
//
// Helpers.
//
extern "C" {
    pub fn be16_to_cpu(_arg: block->bb_numrecs) -> return;
}
extern "C" {
    pub fn be16_to_cpu(_arg: block->bb_level) -> return;
}
//
// Min and max functions for extlen, agblock, fileoff, and filblks types.
//

extern "C" {
    pub fn xfs_btree_agblock_v5hdr_verify(bp: *mut xfs_buf) -> xfs_failaddr_t;
}
//
// Return codes for the query range iterator function are 0 to continue
// iterating, and non-zero to stop iterating.  Any non-zero value will be
// passed up to the _query_range caller.  The special value -ECANCELED can be
// used to stop iteration, because _query_range never generates that error
// code on its own.
//
// Visit record blocks.

// Visit leaf blocks.

// Visit all blocks.

extern "C" {
    pub fn xfs_btree_count_blocks(cur: *mut xfs_btree_cur, blocks: *mut xfs_filblks_t) -> c_int;
}
extern "C" {
    pub fn xfs_btree_has_more_records(cur: *mut xfs_btree_cur) -> bool;
}
// Key comparison helpers
// Masked key comparison helpers
// Does this cursor point to the last block in the given level?
// BMBT allocations can come through from non-transactional context.
extern "C" {
    pub fn xfs_btree_init_cur_caches() -> int __init;
}
extern "C" {
    pub fn xfs_btree_destroy_cur_caches();
}
extern "C" {
    pub fn xfs_btree_goto_left_edge(cur: *mut xfs_btree_cur) -> c_int;
}
// Does this level of the cursor point to the inode root (and not a block)?
