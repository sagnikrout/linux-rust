//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/dabtree.h
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
// Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// dir/attr btree
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_da_btree {
    pub dargs: xfs_da_args,
    pub hashes: [xfs_dahash_t; XFS_DA_NODE_MAXDEPTH],
    pub maxrecs: [c_int; XFS_DA_NODE_MAXDEPTH],
    pub state: *mut xfs_da_state,
    pub sc: *mut xfs_scrub,
    pub private: *mut c_void,
//
// Lowest and highest directory block address in which we expect
// to find dir/attr btree node blocks.  For a directory this
// (presumably) means between LEAF_OFFSET and FREE_OFFSET; for
// attributes there is no limit.
//
    pub lowest: xfs_dablk_t,
    pub highest: xfs_dablk_t,
    pub tree_level: c_int,
}

extern "C" {
    pub fn int(ds: *mut *mut xchk_da_btree_rec_fn)(struct xchk_da_btree, level: c_int) -> typedef;
}
// Check for da btree operation errors.
extern "C" {
    pub fn xchk_da_process_error(ds: *mut xchk_da_btree, level: c_int, error: *mut c_int) -> bool;
}
// Check for da btree corruption.
extern "C" {
    pub fn xchk_da_set_corrupt(ds: *mut xchk_da_btree, level: c_int);
}
extern "C" {
    pub fn xchk_da_set_preen(ds: *mut xchk_da_btree, level: c_int);
}
extern "C" {
    pub fn xchk_da_set_preen(ds: *mut xchk_da_btree, level: c_int);
}
extern "C" {
    pub fn xchk_da_btree_hash(ds: *mut xchk_da_btree, level: c_int, hashp: *mut __be32) -> c_int;
}
