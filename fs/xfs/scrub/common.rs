//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/common.h
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
extern "C" {
    pub fn xchk_trans_alloc(sc: *mut xfs_scrub, resblks: c_uint) -> c_int;
}
extern "C" {
    pub fn xchk_trans_alloc_empty(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xchk_trans_cancel(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xchk_ino_set_preen(sc: *mut xfs_scrub, ino: xfs_ino_t);
}
extern "C" {
    pub fn xchk_set_corrupt(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xchk_ino_set_corrupt(sc: *mut xfs_scrub, ino: xfs_ino_t);
}

extern "C" {
    pub fn xchk_ino_set_warning(sc: *mut xfs_scrub, ino: xfs_ino_t);
}
extern "C" {
    pub fn xchk_set_incomplete(sc: *mut xfs_scrub);
}
extern "C" {
    pub fn xchk_checkpoint_log(mp: *mut xfs_mount) -> c_int;
}
// Are we set up for a cross-referencing check?
// Setup functions
extern "C" {
    pub fn xchk_setup_agheader(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_fs(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_rt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_ag_allocbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_ag_iallocbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_ag_rmapbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_ag_refcountbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_inode(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_inode_bmap(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_inode_bmap_data(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_directory(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_xattr(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_symlink(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_parent(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_dirtree(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_metapath(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xchk_setup_rtbitmap(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_rtsummary(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_rgsuperblock(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_rtrmapbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_rtrefcountbt(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xchk_ino_dqattach(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_quota(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_quotacheck(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xchk_setup_fscounters(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_nlinks(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_ag_free(sc: *mut xfs_scrub, sa: *mut xchk_ag);
}
extern "C" {
    pub fn xchk_perag_drain_and_lock(sc: *mut xfs_scrub) -> c_int;
}
//
// Grab all AG resources, treating the inability to grab the perag structure as
// a fs corruption.  This is intended for callers checking an ondisk reference
// to a given AG, which means that the AG must still exist.
//

// All the locks we need to check an rtgroup.

extern "C" {
    pub fn xchk_rtgroup_unlock(sr: *mut xchk_rt);
}
extern "C" {
    pub fn xchk_rtgroup_btcur_free(sr: *mut xchk_rt);
}
extern "C" {
    pub fn xchk_rtgroup_free(sc: *mut xfs_scrub, sr: *mut xchk_rt);
}

extern "C" {
    pub fn xchk_ag_btcur_free(sa: *mut xchk_ag);
}
extern "C" {
    pub fn xchk_ag_btcur_init(sc: *mut xfs_scrub, sa: *mut xchk_ag);
}
extern "C" {
    pub fn xchk_setup_ag_btree(sc: *mut xfs_scrub, force_log: bool) -> c_int;
}
extern "C" {
    pub fn xchk_iget_for_scrubbing(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_setup_inode_contents(sc: *mut xfs_scrub, resblks: c_uint) -> c_int;
}
extern "C" {
    pub fn xchk_install_live_inode(sc: *mut xfs_scrub, ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xchk_ilock(sc: *mut xfs_scrub, ilock_flags: c_uint);
}
extern "C" {
    pub fn xchk_ilock_nowait(sc: *mut xfs_scrub, ilock_flags: c_uint) -> bool;
}
extern "C" {
    pub fn xchk_iunlock(sc: *mut xfs_scrub, ilock_flags: c_uint);
}
extern "C" {
    pub fn xchk_buffer_recheck(sc: *mut xfs_scrub, bp: *mut xfs_buf);
}
//
// Grab the inode at @inum.  The caller must have created a scrub transaction
// so that we can confirm the inumber by walking the inobt and not deadlock on
// a loop in the inobt.
//
extern "C" {
    pub fn xchk_iget(sc: *mut xfs_scrub, inum: xfs_ino_t, ipp: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xchk_irele(sc: *mut xfs_scrub, ip: *mut xfs_inode);
}
extern "C" {
    pub fn xchk_install_handle_inode(sc: *mut xfs_scrub, ip: *mut xfs_inode) -> c_int;
}
//
// Safe version of (untrusted) xchk_iget that uses an empty transaction to
// avoid deadlocking on loops in the inobt.  This should only be used in a
// scrub or repair setup routine, and only prior to grabbing a transaction.
//
// Don't bother cross-referencing if we already found corruption or cross
// referencing discrepancies.
//
extern "C" {
    pub fn xchk_dir_looks_zapped(dp: *mut xfs_inode) -> bool;
}
extern "C" {
    pub fn xchk_pptr_looks_zapped(ip: *mut xfs_inode) -> bool;
}
// Decide if a repair is required.
//
// "Should we prepare for a repair?"
//
// Return true if the caller permits us to repair metadata and we're not
// setting up for a post-repair evaluation.
//
extern "C" {
    pub fn xchk_metadata_inode_forks(sc: *mut xfs_scrub) -> c_int;
}
//
// Setting up a hook to wait for intents to drain is costly -- we have to take
// the CPU hotplug lock and force an i-cache flush on all CPUs once to set it
// up, and again to tear it down.  These costs add up quickly, so we only want
// to enable the drain waiter if the drain actually detected a conflict with
// running intent chains.
//
extern "C" {
    pub fn xchk_fsgates_enable(sc: *mut xfs_scrub, scrub_fshooks: c_uint);
}
extern "C" {
    pub fn xchk_inode_is_dirtree_root(ip: *const xfs_inode) -> bool;
}
extern "C" {
    pub fn xchk_inode_is_sb_rooted(ip: *const xfs_inode) -> bool;
}
extern "C" {
    pub fn xchk_inode_rootdir_inum(ip: *const xfs_inode) -> xfs_ino_t;
}
