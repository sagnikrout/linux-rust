//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_alloc.h
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
extern "C" {
    pub fn xfs_agfl_size(mp: *mut xfs_mount) -> c_uint;
}
//
// Flags for xfs_alloc_fix_freelist.
//

//
// Argument structure for xfs_alloc routines.
// This is turned into a structure to avoid having 20 arguments passed
// down several levels of the stack.
//
// Defines for datatype
//

// freespace limit calculations
extern "C" {
    pub fn xfs_alloc_set_aside(mp: *mut xfs_mount) -> c_uint;
}
extern "C" {
    pub fn xfs_alloc_ag_max_usable(mp: *mut xfs_mount) -> c_uint;
}
//
// Compute and fill in value of m_alloc_maxlevels.
//
// Log the given fields from the agf structure.
//
// Allocate an extent anywhere in the specific AG given. If there is no
// space matching the requirements in that AG, then the allocation will fail.
//
extern "C" {
    pub fn xfs_alloc_vextent_this_ag(args: *mut xfs_alloc_arg, agno: xfs_agnumber_t) -> c_int;
}
//
// Allocate an extent as close to the target as possible. If there are not
// viable candidates in the AG, then fail the allocation.
//
// Allocate an extent exactly at the target given. If this is not possible
// then the allocation fails.
//
// Best effort full filesystem allocation scan.
//
// Locality aware allocation will be attempted in the initial AG, but on failure
// non-localised attempts will be made. The AGs are constrained by previous
// allocations in the current transaction. Two passes will be made - the first
// non-blocking, the second blocking.
//
// Iterate from the AG indicated from args->fsbno through to the end of the
// filesystem attempting blocking allocation. This is for use in last
// resort allocation attempts when everything else has failed.
//
// Free an extent.
//
extern "C" {
    pub fn __xfs_free_extent(_arg: tp, _arg: pag, _arg: agbno, _arg: len, _arg: oinfo, _arg: type, _arg: false) -> return;
}
extern "C" {
    pub fn xfs_alloc_fix_freelist(args: *mut xfs_alloc_arg, alloc_flags: u32) -> c_int;
}
extern "C" {
    pub fn xfs_prealloc_blocks(mp: *mut xfs_mount) -> xfs_extlen_t;
}
// Don't issue a discard for the blocks freed.

// Free blocks on the realtime device.

//
// List of extents to be free "later".
// The list is kept sorted on xbf_startblock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_extent_free_item {
    pub xefi_list: list_head,
    pub xefi_owner: u64,
    pub /: *mut *mut xfs_fsblock_t xefi_startblock;/ starting fs block number,
    pub /: *mut *mut xfs_extlen_t xefi_blockcount;/ number of blocks in extent,
    pub xefi_group: *mut xfs_group,
    pub xefi_flags: c_uint,
    pub xefi_agresv: xfs_ag_resv_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_alloc_autoreap {
    pub dfp: *mut xfs_defer_pending,
}

extern "C" {
    pub fn xfs_extfree_intent_init_cache() -> int __init;
}
extern "C" {
    pub fn xfs_extfree_intent_destroy_cache();
}
