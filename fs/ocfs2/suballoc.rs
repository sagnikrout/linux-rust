//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/suballoc.h
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
// suballoc.h
//
// Defines sub allocator api
//
// Copyright (C) 2003, 2004 Oracle.  All rights reserved.
//
// found bits
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_alloc_context {
    pub /: *mut *mut *mut inode ac_inode; / which bitmap are we allocating from?,
    pub /: *mut *mut *mut buffer_head ac_bh; / file entry bh,
    pub /: *mut *mut u32 ac_alloc_slot; / which slot are we allocating from?,
    pub ac_bits_wanted: u32,
    pub ac_bits_given: u32,
pub const OCFS2_AC_USE_LOCAL: c_int = 1;
pub const OCFS2_AC_USE_MAIN: c_int = 2;
pub const OCFS2_AC_USE_INODE: c_int = 3;
pub const OCFS2_AC_USE_META: c_int = 4;
pub const OCFS2_AC_USE_MAIN_DISCONTIG: c_int = 5;
    pub ac_which: u32,
// these are used by the chain search
    pub ac_chain: u16,
    pub ac_disable_chain_relink: c_int,
    pub ac_group_search: *mut group_search_t,
    pub ac_last_group: u64,
    pub is: *mut *mut u64 ac_max_block; / Highest block number to allocate. 0,
    pub /: *mut *mut int ac_find_loc_only; / hack for reflink operation ordering,
    pub /: *mut *mut *mut ocfs2_suballoc_result ac_find_loc_priv; /,
    pub ac_resv: *mut ocfs2_alloc_reservation,
}

extern "C" {
    pub fn ocfs2_init_steal_slots(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_free_alloc_context(ac: *mut ocfs2_alloc_context);
}
//
// Please note that the caller must make sure that root_el is the root
// of extent tree. So for an inode, it should be &fe->id2.i_list. Otherwise
// the result may be wrong.
//
// Use this variant of ocfs2_claim_clusters to specify a maximum
// number of clusters smaller than the allocation reserved.
//
// This should work for all block group descriptors as only
// the 1st group descriptor of the cluster bitmap is
// different.
// the rest of the block groups are located at the beginning
// of their 1st cluster, so a direct translation just
// works.
extern "C" {
    pub fn ocfs2_blocks_to_clusters(_arg: osb->sb, _arg: bg_blkno) -> return;
}
// This is for local alloc ONLY. Others should use the task-specific
// apis above.
extern "C" {
    pub fn ocfs2_free_ac_resource(ac: *mut ocfs2_alloc_context);
}
// given a cluster offset, calculate which block group it belongs to
// and return that block offset.
extern "C" {
    pub fn ocfs2_which_cluster_group(inode: *mut inode, cluster: u32) -> u64;
}
//
// By default, ocfs2_read_group_descriptor() calls ocfs2_error() when it
// finds a problem.  A caller that wants to check a group descriptor
// without going readonly should read the block with ocfs2_read_block[s]()
// and then checking it with this function.  This is only resize, really.
// Everyone else should be using ocfs2_read_group_descriptor().
//
// Read a group descriptor block into *bh.  If *bh is NULL, a bh will be
// allocated.  This is a cached read.  The descriptor will be validated with
// ocfs2_validate_group_descriptor().
//
extern "C" {
    pub fn ocfs2_test_inode_bit(osb: *mut ocfs2_super, blkno: u64, res: *mut c_int) -> c_int;
}
//
// The following two interfaces are for ocfs2_create_inode_in_orphan().
//
