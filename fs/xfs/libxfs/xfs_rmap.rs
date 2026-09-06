//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_rmap.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2016 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//

// Reverse mapping functions.
// owner = oinfo->oi_owner;
// offset = oinfo->oi_offset;
// flags = r;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_rmap_intent_type {
    XFS_RMAP_MAP,
    XFS_RMAP_MAP_SHARED,
    XFS_RMAP_UNMAP,
    XFS_RMAP_UNMAP_SHARED,
    XFS_RMAP_CONVERT,
    XFS_RMAP_CONVERT_SHARED,
    XFS_RMAP_ALLOC,
    XFS_RMAP_FREE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rmap_intent {
    pub ri_list: list_head,
    pub ri_type: xfs_rmap_intent_type,
    pub ri_whichfork: c_int,
    pub ri_owner: u64,
    pub ri_bmap: xfs_bmbt_irec,
    pub ri_group: *mut xfs_group,
    pub ri_realtime: bool,
}

// functions for updating the rmapbt based on bmbt map/unmap operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rmap_matches {
// Number of owner matches.
    pub matches: c_ulonglong,
// Number of non-owner matches.
    pub non_owner_matches: c_ulonglong,
// Number of non-owner matches that conflict with the owner matches.
    pub bad_non_owner_matches: c_ulonglong,
}

extern "C" {
    pub fn xfs_rmap_map_raw(cur: *mut xfs_btree_cur, rmap: *mut xfs_rmap_irec) -> c_int;
}
extern "C" {
    pub fn xfs_rmap_intent_init_cache() -> int __init;
}
extern "C" {
    pub fn xfs_rmap_intent_destroy_cache();
}
//
// Parameters for tracking reverse mapping changes.  The hook function arg
// parameter is enum xfs_rmap_intent_type, and the rest is below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rmap_update_params {
    pub startblock: xfs_agblock_t,
    pub blockcount: xfs_extlen_t,
    pub oinfo: xfs_owner_info,
    pub unwritten: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rmap_hook {
    pub rmap_hook: xfs_hook,
}

extern "C" {
    pub fn xfs_rmap_hook_disable();
}
extern "C" {
    pub fn xfs_rmap_hook_enable();
}
extern "C" {
    pub fn xfs_rmap_hook_add(xg: *mut xfs_group, hook: *mut xfs_rmap_hook) -> c_int;
}
extern "C" {
    pub fn xfs_rmap_hook_del(xg: *mut xfs_group, hook: *mut xfs_rmap_hook);
}
extern "C" {
    pub fn xfs_rmap_hook_setup(hook: *mut xfs_rmap_hook, mod_fn: notifier_fn_t);
}

