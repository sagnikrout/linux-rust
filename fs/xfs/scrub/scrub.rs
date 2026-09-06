//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/scrub.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_relax {
    pub next_resched: c_ulong,
    pub resched_nr: c_uint,
    pub killable: bool,
}

// Yield to the scheduler at most 10x per second.

//
// Relax during a scrub operation and exit if there's a fatal signal pending.
//
// If preemption is disabled, we need to yield to the scheduler every now and
// then so that we don't run afoul of the soft lockup watchdog or RCU stall
// detector.  cond_resched calls are somewhat expensive (~5ns) so we want to
// ratelimit this to 10x per second.  Amortize the cost of the other checks by
// only doing it once every 100 calls.
//
// Amortize the cost of scheduling and checking signals.
//
// Standard flags for allocating memory within scrub.  NOFS context is
// configured by the process allocation scope.  Scrub and repair must be able
// to back out gracefully if there isn't enough memory.  Force-cast to avoid
// complaints from static checkers.
//

//
// For opening files by handle for fsck operations, we don't trust the inumber
// or the allocation state; therefore, perform an untrusted lookup.  We don't
// want these inodes to pollute the cache, so mark them for immediate removal.
//

// Type info and names for the scrub types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xchk_type {
    ST_NONE = 1,	/* disabled */
    ST_PERAG,	/* per-AG metadata */
    ST_FS,		/* per-FS metadata */
    ST_INODE,	/* per-inode metadata */
    ST_GENERIC,	/* determined by the scrubber */
    ST_RTGROUP,	/* rtgroup metadata */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_meta_ops {
// Acquire whatever resources are needed for the operation.
    pub sc): *mut *mut int (setup)(struct xfs_scrub,
// Examine metadata for errors.
    pub ): *mut *mut int (scrub)(struct xfs_scrub,
// Repair or optimize the metadata.
    pub ): *mut *mut int (repair)(struct xfs_scrub,
//
// Re-scrub the metadata we repaired, in case there's extra work that
// we need to do to check our repair work.  If this is NULL, we'll use
// the ->scrub function pointer, assuming that the regular scrub is
// sufficient.
//
    pub sc): *mut *mut int (repair_eval)(struct xfs_scrub,
// Decide if we even have this piece of metadata.
    pub ): *const *const bool (has)(struct xfs_mount,
// type describing required/allowed inputs
    pub type: xchk_type,
}

// Buffer pointers and btree cursors for an entire AG.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_ag {
    pub pag: *mut xfs_perag,
// AG btree roots
    pub agf_bp: *mut xfs_buf,
    pub agi_bp: *mut xfs_buf,
// AG btrees
    pub bno_cur: *mut xfs_btree_cur,
    pub cnt_cur: *mut xfs_btree_cur,
    pub ino_cur: *mut xfs_btree_cur,
    pub fino_cur: *mut xfs_btree_cur,
    pub rmap_cur: *mut xfs_btree_cur,
    pub refc_cur: *mut xfs_btree_cur,
}

// Inode lock state for the RT volume.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_rt {
// incore rtgroup, if applicable
    pub rtg: *mut xfs_rtgroup,
// XFS_RTGLOCK_* lock state if locked
    pub rtlock_flags: c_uint,
// rtgroup btrees
    pub rmap_cur: *mut xfs_btree_cur,
    pub refc_cur: *mut xfs_btree_cur,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_scrub {
// General scrub state.
    pub mp: *mut xfs_mount,
    pub sm: *mut xfs_scrub_metadata,
    pub ops: *const xchk_meta_ops,
    pub tp: *mut xfs_trans,
// File that scrub was called with.
    pub file: *mut file,
//
// File that is undergoing the scrub operation.  This can differ from
// the file that scrub was called with if we're checking file-based fs
// metadata (e.g. rt bitmaps) or if we're doing a scrub-by-handle for
// something that can't be opened directly (e.g. symlinks).
//
    pub ip: *mut xfs_inode,
// Kernel memory buffer used by scrubbers; freed at teardown.
    pub buf: *mut c_void,
//
// Clean up resources owned by whatever is in the buffer.  Cleanup can
// be deferred with this hook as a means for scrub functions to pass
// data to repair functions.  This function must not free the buffer
// itself.
//
    pub buf): *mut *mut void (buf_cleanup)(void,
// xfile used by the scrubbers; freed at teardown.
    pub xfile: *mut xfile,
// buffer target for in-memory btrees; also freed at teardown.
    pub xmbtp: *mut xfs_buftarg,
// Lock flags for @ip.
    pub ilock_flags: c_uint,
// The orphanage, for stashing files that have lost their parent.
    pub orphanage_ilock_flags: c_uint,
    pub orphanage: *mut xfs_inode,
// A temporary file on this filesystem, for staging new metadata.
    pub tempip: *mut xfs_inode,
    pub temp_ilock_flags: c_uint,
// See the XCHK/XREP state flags below.
    pub flags: c_uint,
//
// The XFS_SICK_* flags that correspond to the metadata being scrubbed
// or repaired.  We will use this mask to update the in-core fs health
// status with whatever we find.
//
    pub sick_mask: c_uint,
//
// Clear these XFS_SICK_* flags but only if the scan is ok.  Useful for
// removing ZAPPED flags after a repair.
//
    pub healthy_mask: c_uint,
// next time we want to cond_resched()
    pub relax: xchk_relax,
// State tracking for single-AG operations.
    pub sa: xchk_ag,
// State tracking for realtime operations.
    pub sr: xchk_rt,
}

// XCHK state flags grow up from zero, XREP state flags grown down from 2^31

//
// The XCHK_FSGATES* flags reflect functionality in the main filesystem that
// are only enabled for this particular online fsck.  When not in use, the
// features are gated off via dynamic code patching, which is why the state
// must be enabled during scrub setup and can only be torn down afterwards.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_scrub_subord {
    pub sc: xfs_scrub,
    pub parent_sc: *mut xfs_scrub,
    pub old_smtype: c_uint,
    pub old_smflags: c_uint,
}

extern "C" {
    pub fn xchk_scrub_free_subord(sub: *mut xfs_scrub_subord);
}
//
// We /could/ terminate a scrub/repair operation early.  If we're not
// in a good place to continue (fatal signal, etc.) then bail out.
// Note that we're careful not to make any judgements about *error.
//
// error = -EINTR;
// Metadata scrubbers
extern "C" {
    pub fn xchk_tester(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_superblock(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_agf(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_agfl(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_agi(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_allocbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_iallocbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_rmapbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_refcountbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_inode(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_bmap_data(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_bmap_attr(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_bmap_cow(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_directory(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_xattr(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_symlink(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_parent(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_dirtree(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_metapath(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xchk_rtbitmap(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_rtsummary(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_rgsuperblock(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_rtrmapbt(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_rtrefcountbt(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xchk_quota(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_quotacheck(sc: *mut xfs_scrub) -> c_int;
}

extern "C" {
    pub fn xchk_fscounters(sc: *mut xfs_scrub) -> c_int;
}
extern "C" {
    pub fn xchk_nlinks(sc: *mut xfs_scrub) -> c_int;
}
// cross-referencing helpers

