//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_stats.h
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
// Copyright (c) 2000,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

//
// The btree stats arrays have fixed offsets for the different stats. We
// store the base index in the btree cursor via XFS_STATS_CALC_INDEX() and
// that allows us to use fixed offsets into the stats array for each btree
// stat. These index offsets are defined in the order they will be emitted
// in the stats files, so it is possible to add new btree stat types by
// appending to the enum list below.
//
// XFS global statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __xfsstats {
    pub xs_allocx: u32,
    pub xs_allocb: u32,
    pub xs_freex: u32,
    pub xs_freeb: u32,
    pub xs_abt_lookup: u32,
    pub xs_abt_compare: u32,
    pub xs_abt_insrec: u32,
    pub xs_abt_delrec: u32,
    pub xs_blk_mapr: u32,
    pub xs_blk_mapw: u32,
    pub xs_blk_unmap: u32,
    pub xs_add_exlist: u32,
    pub xs_del_exlist: u32,
    pub xs_look_exlist: u32,
    pub xs_cmp_exlist: u32,
    pub xs_bmbt_lookup: u32,
    pub xs_bmbt_compare: u32,
    pub xs_bmbt_insrec: u32,
    pub xs_bmbt_delrec: u32,
    pub xs_dir_lookup: u32,
    pub xs_dir_create: u32,
    pub xs_dir_remove: u32,
    pub xs_dir_getdents: u32,
    pub xs_trans_sync: u32,
    pub xs_trans_async: u32,
    pub xs_trans_empty: u32,
    pub xs_ig_attempts: u32,
    pub xs_ig_found: u32,
    pub xs_ig_frecycle: u32,
    pub xs_ig_missed: u32,
    pub xs_ig_dup: u32,
    pub xs_ig_reclaims: u32,
    pub xs_ig_attrchg: u32,
    pub xs_log_writes: u32,
    pub xs_log_blocks: u32,
    pub xs_log_noiclogs: u32,
    pub xs_log_force: u32,
    pub xs_log_force_sleep: u32,
    pub xs_try_logspace: u32,
    pub xs_sleep_logspace: u32,
    pub xs_push_ail: u32,
    pub xs_push_ail_success: u32,
    pub xs_push_ail_pushbuf: u32,
    pub xs_push_ail_pinned: u32,
    pub xs_push_ail_locked: u32,
    pub xs_push_ail_flushing: u32,
    pub xs_push_ail_restarts: u32,
    pub xs_push_ail_flush: u32,
    pub xs_xstrat_quick: u32,
    pub xs_xstrat_split: u32,
    pub xs_write_calls: u32,
    pub xs_read_calls: u32,
    pub xs_attr_get: u32,
    pub xs_attr_set: u32,
    pub xs_attr_remove: u32,
    pub xs_attr_list: u32,
    pub xs_iflush_count: u32,
    pub xs_icluster_flushcnt: u32,
    pub xs_icluster_flushinode: u32,
    pub xs_inodes_active: u32,
    pub __unused_vn_alloc: u32,
    pub __unused_vn_get: u32,
    pub __unused_vn_hold: u32,
    pub xs_inode_destroy: u32,
    pub /: *mut *mut uint32_t xs_inode_destroy2; / same as xs_inode_destroy,
    pub xs_inode_mark_reclaimable: u32,
    pub __unused_vn_free: u32,
    pub xb_get: u32,
    pub xb_create: u32,
    pub xb_get_locked: u32,
    pub xb_get_locked_waited: u32,
    pub xb_busy_locked: u32,
    pub xb_miss_locked: u32,
    pub xb_page_retries: u32,
    pub xb_page_found: u32,
    pub xb_get_read: u32,
// Version 2 btree counters
    pub xs_abtb_2: [u32; __XBTS_MAX],
    pub xs_abtc_2: [u32; __XBTS_MAX],
    pub xs_bmbt_2: [u32; __XBTS_MAX],
    pub xs_ibt_2: [u32; __XBTS_MAX],
    pub xs_fibt_2: [u32; __XBTS_MAX],
    pub xs_rmap_2: [u32; __XBTS_MAX],
    pub xs_refcbt_2: [u32; __XBTS_MAX],
    pub xs_rmap_mem_2: [u32; __XBTS_MAX],
    pub xs_rcbag_2: [u32; __XBTS_MAX],
    pub xs_rtrmap_2: [u32; __XBTS_MAX],
    pub xs_rtrmap_mem_2: [u32; __XBTS_MAX],
    pub xs_rtrefcbt_2: [u32; __XBTS_MAX],
    pub xs_qm_dqreclaims: u32,
    pub xs_qm_dqreclaim_misses: u32,
    pub xs_qm_dquot_dups: u32,
    pub xs_qm_dqcachemisses: u32,
    pub xs_qm_dqcachehits: u32,
    pub xs_qm_dqwants: u32,
    pub xs_qm_dquot: u32,
    pub xs_qm_dquot_unused: u32,
// Zone GC counters
    pub xs_gc_read_calls: u32,
    pub xs_gc_write_calls: u32,
    pub xs_gc_zone_reset_calls: u32,
// Metafile counters
    pub xs_inodes_meta: u32,
// Extra precision counters
    pub xs_xstrat_bytes: u64,
    pub xs_write_bytes: u64,
    pub xs_read_bytes: u64,
    pub xs_defer_relog: u64,
    pub xs_gc_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfsstats {
    pub s: __xfsstats,
    pub a: [u32; xfsstats_offset(xs_qm_dquot)],
}

//
// simple wrapper for getting the array index of s struct member offset
//

extern "C" {
    pub fn xfs_stats_format(stats: *mut xfsstats __percpu, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn xfs_stats_clearall(stats: *mut xfsstats __percpu);
}

extern "C" {
    pub fn xfs_init_procfs() -> c_int;
}
extern "C" {
    pub fn xfs_cleanup_procfs();
}

