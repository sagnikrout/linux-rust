//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_rtgroup.h
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
// Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
pub const __LIBXFS_RTGROUP_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_rtg_inodes {
    XFS_RTGI_BITMAP,	/* allocation bitmap */
    XFS_RTGI_SUMMARY,	/* allocation summary */
    XFS_RTGI_RMAP,		/* rmap btree inode */
    XFS_RTGI_REFCOUNT,	/* refcount btree inode */

    XFS_RTGI_MAX,
}

//
// Realtime group incore structure, similar to the per-AG structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rtgroup {
    pub rtg_group: xfs_group,
// per-rtgroup metadata inodes
    pub rtg_inodes: [*mut xfs_inode; XFS_RTGI_MAX],
// Number of blocks in this group
    pub rtg_extents: xfs_rtxnum_t,
//
// For bitmap based RT devices this points to a cache of rt summary
// level per bitmap block with the invariant that rtg_rsum_cache[bbno]
// > the maximum i for which rsum[i][bbno] != 0, or 0 if
// rsum[i][bbno] == 0 for all i.
// Reads and writes are serialized by the rsumip inode lock.
//
// For zoned RT devices this points to the open zone structure for
// a group that is open for writers, or is NULL.
//
    pub rtg_rsum_cache: *mut u8,
    pub rtg_open_zone: *mut xfs_open_zone,
}

//
// Count of outstanding GC operations for zoned XFS.  Any RTG with a
// non-zero rtg_gccount will not be picked as new GC victim.
//
// For zoned RT devices this is set on groups that have no written blocks
// and can be picked by the allocator for opening.
//

extern "C" {
    pub fn container_of(_arg: xg, xfs_rtgroup: struct, _arg: rtg_group) -> return;
}
// Passive rtgroup references
extern "C" {
    pub fn to_rtg(_arg: xfs_group_get(mp, _arg: rgno, _arg: XG_TYPE_RTG)) -> return;
}
extern "C" {
    pub fn to_rtg(_arg: xfs_group_hold(rtg_group(rtg))) -> return;
}
// Active rtgroup references
extern "C" {
    pub fn to_rtg(_arg: xfs_group_grab(mp, _arg: rgno, _arg: XG_TYPE_RTG)) -> return;
}
extern "C" {
    pub fn xfs_rtgroup_next_range(_arg: mp, _arg: rtg, _arg: 0, 1: mp->m_sb.sb_rgcount -) -> return;
}
extern "C" {
    pub fn xfs_verify_gbno(_arg: rtg_group(rtg), _arg: rgbno) -> return;
}
//
// Check that [@rgbno,@len] is a valid extent range in @rtg.
//
// Must only be used for RTG-enabled file systems.
//
extern "C" {
    pub fn xfs_verify_gbext(_arg: rtg_group(rtg), _arg: rgbno, _arg: len) -> return;
}
extern "C" {
    pub fn xfs_gbno_to_fsb(_arg: rtg_group(rtg), _arg: rgbno) -> return;
}
extern "C" {
    pub fn xfs_fsb_to_gno(_arg: mp, _arg: rtbno, _arg: XG_TYPE_RTG) -> return;
}
extern "C" {
    pub fn xfs_fsb_to_gbno(_arg: mp, _arg: rtbno, _arg: XG_TYPE_RTG) -> return;
}
// Is rtbno the start of a RT group?
// Convert an rtgroups rt extent number into an rgbno.
extern "C" {
    pub fn XFS_FSB_TO_BB(_arg: mp, rtbno: g->start_fsb +) -> return;
}

extern "C" {
    pub fn xfs_rtgroup_free(mp: *mut xfs_mount, rgno: xfs_rgnumber_t);
}
extern "C" {
    pub fn xfs_rtgroup_extents(mp: *mut xfs_mount, rgno: xfs_rgnumber_t) -> xfs_rtxnum_t;
}
// Lock the rt bitmap inode in exclusive mode

// Lock the rt bitmap inode in shared mode

// Lock the rt rmap inode in exclusive mode

// Lock the rt refcount inode in exclusive mode

extern "C" {
    pub fn xfs_rtgroup_lock(rtg: *mut xfs_rtgroup, rtglock_flags: c_uint);
}
extern "C" {
    pub fn xfs_rtgroup_unlock(rtg: *mut xfs_rtgroup, rtglock_flags: c_uint);
}
extern "C" {
    pub fn xfs_rtginode_mkdir_parent(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_rtginode_load_parent(tp: *mut xfs_trans) -> c_int;
}
extern "C" {
    pub fn xfs_rtginode_metafile_type(type: xfs_rtg_inodes) -> xfs_metafile_type;
}
extern "C" {
    pub fn xfs_rtginode_enabled(rtg: *mut xfs_rtgroup, type: xfs_rtg_inodes) -> bool;
}
extern "C" {
    pub fn xfs_rtginode_mark_sick(rtg: *mut xfs_rtgroup, type: xfs_rtg_inodes);
}
extern "C" {
    pub fn xfs_rtginode_irele(ipp: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_rtginode_irele(ipp: *mut xfs_inode);
}
extern "C" {
    pub fn kasprintf(_arg: GFP_KERNEL, _arg: "%u.%s", _arg: rgno, _arg: xfs_rtginode_name(type)) -> return;
}

extern "C" {
    pub fn xfs_groups_to_rfsbs(_arg: mp, _arg: nr_groups, _arg: XG_TYPE_RTG) -> return;
}
//
// Return the "raw" size of a group on the hardware device.  This includes the
// daddr gaps present for XFS_SB_FEAT_INCOMPAT_ZONE_GAPS file systems.
//
