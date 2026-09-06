//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_ag.h
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
// Copyright (c) 2018 Red Hat, Inc.
// All rights reserved.
//
pub const __LIBXFS_AG_H: c_int = 1;

//
// Per-ag infrastructure
//
// per-AG block reservation data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_ag_resv {
// number of blocks originally reserved here
    pub ar_orig_reserved: xfs_extlen_t,
// number of blocks reserved here
    pub ar_reserved: xfs_extlen_t,
// number of blocks originally asked for
    pub ar_asked: xfs_extlen_t,
}

//
// Per-ag incore structure, copies of information in agf and agi, to improve the
// performance of allocation group selection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_perag {
    pub pag_group: xfs_group,
    pub pag_opstate: c_ulong,
    pub /: *mut *mut uint8_t pagf_bno_level; / # of levels in bno btree,
    pub /: *mut *mut uint8_t pagf_cnt_level; / # of levels in cnt btree,
    pub /: *mut *mut uint8_t pagf_rmap_level;/ # of levels in rmap btree,
    pub /: *mut *mut uint32_t pagf_flcount; / count of blocks in freelist,
    pub /: *mut *mut xfs_extlen_t pagf_freeblks; / total free blocks,
    pub /: *mut *mut xfs_extlen_t pagf_longest; / longest free space,
    pub /: *mut *mut uint32_t pagf_btreeblks; / # of blocks held in AGF btrees,
    pub /: *mut *mut xfs_agino_t pagi_freecount; / number of free inodes,
    pub /: *mut *mut xfs_agino_t pagi_count; / number of allocated inodes,
//
// Inode allocation search lookup optimisation.
// If the pagino matches, the search for new inodes
// doesn't need to search the near ones again straight away
//
    pub pagl_pagino: xfs_agino_t,
    pub pagl_leftrec: xfs_agino_t,
    pub pagl_rightrec: xfs_agino_t,
    pub /: *mut *mut uint8_t pagf_refcount_level; / recount btree height,
// Blocks reserved for all kinds of metadata.
    pub pag_meta_resv: xfs_ag_resv,
// Blocks reserved for the reverse mapping btree.
    pub pag_rmapbt_resv: xfs_ag_resv,
// Precalculated geometry info
    pub agino_min: xfs_agino_t,
    pub agino_max: xfs_agino_t,

// -- kernel only structures below this line --

//
// Alternate btree heights so that online repair won't trip the write
// verifiers while rebuilding the AG btrees.
//
    pub pagf_repair_bno_level: u8,
    pub pagf_repair_cnt_level: u8,
    pub pagf_repair_refcount_level: u8,
    pub pagf_repair_rmap_level: u8,

    pub /: *mut *mut atomic_t pagf_fstrms; / # of filestreams active in this AG,
    pub /: *mut *mut spinlock_t pag_ici_lock; / incore inode cache lock,
    pub /: *mut *mut radix_tree_root pag_ici_root; / incore inode cache root,
    pub /: *mut *mut int pag_ici_reclaimable; / reclaimable inodes,
    pub /: *mut *mut unsigned long pag_ici_reclaim_cursor; / reclaim restart point,
// background prealloc block trimming
    pub pag_blockgc_work: delayed_work,

}

extern "C" {
    pub fn container_of(_arg: xg, xfs_perag: struct, _arg: pag_group) -> return;
}
//
// Per-AG operational state. These are atomic flag bits.
//
pub const XFS_AGSTATE_AGF_INIT: c_int = 0;
pub const XFS_AGSTATE_AGI_INIT: c_int = 1;
pub const XFS_AGSTATE_PREFERS_METADATA: c_int = 2;
pub const XFS_AGSTATE_ALLOWS_INODES: c_int = 3;
pub const XFS_AGSTATE_AGFL_NEEDS_RESET: c_int = 4;

extern "C" {
    pub fn xfs_initialize_perag_data(mp: *mut xfs_mount, agno: xfs_agnumber_t) -> c_int;
}
extern "C" {
    pub fn xfs_update_last_ag_size(mp: *mut xfs_mount, prev_agcount: xfs_agnumber_t) -> c_int;
}
// Passive AG references
extern "C" {
    pub fn to_perag(_arg: xfs_group_get(mp, _arg: agno, _arg: XG_TYPE_AG)) -> return;
}
extern "C" {
    pub fn to_perag(_arg: xfs_group_hold(pag_group(pag))) -> return;
}
// Active AG references
extern "C" {
    pub fn to_perag(_arg: xfs_group_grab(mp, _arg: agno, _arg: XG_TYPE_AG)) -> return;
}
extern "C" {
    pub fn xfs_perag_next_range(_arg: mp, _arg: pag, _arg: start_agno, 1: mp->m_sb.sb_agcount -) -> return;
}
extern "C" {
    pub fn xfs_perag_next_from(_arg: mp, _arg: pag, _arg: 0) -> return;
}
//
// Per-ag geometry infomation and validation
//
extern "C" {
    pub fn xfs_ag_block_count(mp: *mut xfs_mount, agno: xfs_agnumber_t) -> xfs_agblock_t;
}
extern "C" {
    pub fn xfs_verify_gbno(_arg: pag_group(pag), _arg: agbno) -> return;
}
extern "C" {
    pub fn xfs_verify_gbext(_arg: pag_group(pag), _arg: agbno, _arg: len) -> return;
}
//
// Verify that an AG inode number pointer neither points outside the AG
// nor points at static metadata.
//
// Verify that an AG inode number pointer neither points outside the AG
// nor points at static metadata, or is NULLAGINO.
//
extern "C" {
    pub fn xfs_verify_agino(_arg: pag, _arg: agino) -> return;
}
// agno = pag_agno(pag) + 1;
// agno = restart_agno;
//
// Iterate all AGs from start_agno through wrap_agno, then restart_agno through
// (start_agno - 1).
//

//
// Iterate all AGs from start_agno through wrap_agno, then 0 through
// (start_agno - 1).
//

//
// Iterate all AGs from start_agno through to the end of the filesystem, then 0
// through (start_agno - 1).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aghdr_init_data {
// per ag data
    pub /: *mut *mut xfs_agblock_t agno; / ag to init,
    pub /: *mut *mut xfs_extlen_t agsize; / new AG size,
    pub /: *mut *mut list_head buffer_list; / buffer writeback list,
    pub /: *mut *mut xfs_rfsblock_t nfree; / cumulative new free space,
// per header data
    pub /: *mut *mut xfs_daddr_t daddr; / header location,
    pub /: *mut *mut size_t numblks; / size of header,
    pub /: *const *const *const xfs_btree_ops bc_ops; / btree ops,
}

extern "C" {
    pub fn xfs_ag_init_headers(mp: *mut xfs_mount, id: *mut aghdr_init_data) -> c_int;
}
extern "C" {
    pub fn xfs_ag_get_geometry(pag: *mut xfs_perag, ageo: *mut xfs_ag_geometry) -> c_int;
}
extern "C" {
    pub fn XFS_AGB_TO_FSB(_arg: pag_mount(pag), _arg: pag_agno(pag), _arg: agbno) -> return;
}
extern "C" {
    pub fn XFS_AGB_TO_DADDR(_arg: pag_mount(pag), _arg: pag_agno(pag), _arg: agbno) -> return;
}
extern "C" {
    pub fn XFS_AGINO_TO_INO(_arg: pag_mount(pag), _arg: pag_agno(pag), _arg: agino) -> return;
}
