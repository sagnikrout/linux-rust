//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/repair.h
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
// Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//

// Repair helpers
extern "C" {
    pub fn xrep_attempt(sc: *mut xfs_scrub, run: *mut xchk_stats_run) -> c_int;
}
extern "C" {
    pub fn xrep_will_attempt(sc: *mut xfs_scrub) -> bool;
}
extern "C" {
    pub fn xrep_failure(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xrep_roll_ag_trans(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_roll_trans(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_defer_finish(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_calc_ag_resblks(sc: *mut xfs_scrub) -> xfs_extlen_t;
}
extern "C" {
    pub fn xrep_fix_freelist(sc: *mut xfs_scrub, alloc_flags: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrep_find_ag_btree {
// in: rmap owner of the btree we're looking for
    pub rmap_owner: u64,
// in: buffer ops
    pub buf_ops: *const xfs_buf_ops,
// in: maximum btree height
    pub maxlevels: c_uint,
// out: the highest btree block found and the tree height
    pub root: xfs_agblock_t,
    pub height: c_uint,
}

extern "C" {
    pub fn xrep_force_quotacheck(sc: *mut xfs_scrub, type: xfs_dqtype_t);
}
extern "C" {
    pub fn xrep_ino_dqattach(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xrep_setup_xfbtree(sc: *mut xfs_scrub, descr: *const c_char) -> c_int;
}
extern "C" {
    pub fn xrep_reset_perag_resv(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_bmap(sc: *mut xfs_scrub, whichfork: c_int, allow_unwritten: bool) -> c_int;
}
extern "C" {
    pub fn xrep_metadata_inode_forks(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_ag_rmapbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_ag_refcountbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_xattr(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_directory(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_parent(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_nlinks(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_symlink(sc: *mut xfs_scrub, resblks: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn xrep_setup_dirtree(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_rtrmapbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_rtrefcountbt(sc: *mut xfs_scrub) -> c_int;
}
// Repair setup functions
extern "C" {
    pub fn xrep_setup_ag_allocbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_setup_inode(sc: *mut xfs_scrub, imap: *const xfs_imap) -> c_int;
}
extern "C" {
    pub fn xrep_ag_btcur_init(sc: *mut xfs_scrub, sa: *mut xchk_ag);
}

extern "C" {
    pub fn xrep_rtgroup_btcur_init(sc: *mut xfs_scrub, sr: *mut xchk_rt);
}
extern "C" {
    pub fn xrep_calc_rtgroup_resblks(sc: *mut xfs_scrub) -> xfs_extlen_t;
}

// Metadata revalidators
extern "C" {
    pub fn xrep_revalidate_allocbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_revalidate_iallocbt(sc: *mut xfs_scrub) -> c_int;
}
// Metadata repairers
extern "C" {
    pub fn xrep_probe(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_superblock(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_agf(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_agfl(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_agi(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_allocbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_iallocbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_rmapbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_refcountbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_inode(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_bmap_data(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_bmap_attr(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_bmap_cow(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_nlinks(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_fscounters(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_xattr(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_directory(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_parent(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_symlink(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_dirtree(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_metapath(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xrep_rtbitmap(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_rtsummary(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_rgsuperblock(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_rtrmapbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_rtrefcountbt(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xrep_quota(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_quotacheck(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xrep_reinit_pagf(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_reinit_pagi(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xrep_buf_verify_struct(bp: *mut xfs_buf, ops: *const xfs_buf_ops) -> bool;
}
extern "C" {
    pub fn xrep_inode_set_nblocks(sc: *mut xfs_scrub, new_blocks: i64);
}
extern "C" {
    pub fn xrep_reset_metafile_resv(sc: *mut xfs_scrub) -> c_int;
}

//
// When online repair is not built into the kernel, we still want to attempt
// the repair so that the stub xrep_attempt below will return EOPNOTSUPP.
//

// repair setup functions for no-repair

