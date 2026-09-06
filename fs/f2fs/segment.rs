//! Automatically rewritten from C Header to Rust Module
//! Source: fs/f2fs/segment.h
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
// fs/f2fs/segment.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// constant macro

// L: Logical segment # in volume, R: Relative segment # in main area

//
// In the victim_sel_policy->alloc_mode, there are three block allocation modes.
// LFS writes data sequentially with cleaning operations.
// SSR (Slack Space Recycle) reuses obsolete space without cleaning operations.
// AT_SSR (Age Threshold based Slack Space Recycle) merges fragments into
// fragmented segment which has similar aging degree.
//
// In the victim_sel_policy->gc_mode, there are three gc, aka cleaning, modes.
// GC_CB is based on cost-benefit algorithm.
// GC_GREEDY is based on greedy algorithm.
// GC_AT is based on age-threshold algorithm.
//
// BG_GC means the background cleaning job.
// FG_GC means the on-demand cleaning job.
//
// for a function parameter to select a victim segment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct victim_sel_policy {
    pub /: *mut *mut int alloc_mode; / LFS or SSR,
    pub /: *mut *mut int gc_mode; / GC_CB or GC_GREEDY,
    pub /: *mut *mut *mut unsigned long dirty_bitmap; / dirty segment/section bitmap,
    pub /*: *mut unsigned int max_search;,
// maximum # of segments/sections
// to search
//
    pub /: *mut *mut unsigned int offset; / last scanned bitmap offset,
    pub /: *mut *mut unsigned int ofs_unit; / bitmap search unit,
    pub /: *mut *mut unsigned int min_cost; / minimum cost,
    pub /: *mut *mut unsigned long long oldest_age; / oldest age of segments having the same min cost,
    pub /: *mut *mut unsigned int min_segno; / segment # having min. cost,
    pub section*/: *mut *mut unsigned long long age; / mtime of GCed,
    pub /: *mut *mut unsigned long long age_threshold;/ age threshold,
    pub /: *mut *mut bool one_time_gc; / one time GC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg_entry {
    pub /: *mut *mut unsigned int type:6; / segment type like CURSEG_XXX_TYPE,
    pub /: *mut *mut unsigned int valid_blocks:10; / # of valid blocks,
    pub /: *mut *mut unsigned int ckpt_valid_blocks:10; / # of valid blocks last cp,
    pub /: *mut *mut unsigned int padding:6; / padding,
    pub /: *mut *mut *mut unsigned char cur_valid_map; / validity bitmap of blocks,
//
// # of valid blocks and the validity bitmap stored in the last
// checkpoint pack. This information is used by the SSR mode.
//
    pub /: *mut *mut *mut unsigned char ckpt_valid_map; / validity bitmap of blocks last cp,
    pub discard_map: *mut c_uchar,
    pub /: *mut *mut unsigned long long mtime; / modification time of the segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_entry {
    pub /: *mut *mut unsigned int valid_blocks; / # of valid blocks in a section,
    pub /: *mut *mut unsigned int ckpt_valid_blocks; / # of valid blocks last cp in a section,
}

pub const MAX_SKIP_GC_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct revoke_entry {
    pub list: list_head,
    pub /: *mut *mut block_t old_addr; / for revoking when fail to commit,
    pub index: pgoff_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sit_info {
    pub /: *mut *mut block_t sit_base_addr; / start block address of SIT area,
    pub /: *mut *mut block_t sit_blocks; / # of blocks used by SIT area,
    pub /: *mut *mut block_t written_valid_blocks; / # of valid blocks in main area,
    pub /: *mut *mut *mut char bitmap; / all bitmaps pointer,
    pub /: *mut *mut *mut char sit_bitmap; / SIT bitmap pointer,

// bitmap of segments to be ignored by GC in case of errors
    pub invalid_segmap: *mut c_ulong,

    pub /: *mut *mut unsigned int bitmap_size; / SIT bitmap size,
    pub /: *mut *mut *mut unsigned long tmp_map; / bitmap for temporal use,
    pub /: *mut *mut *mut unsigned long dirty_sentries_bitmap; / bitmap for dirty sentries,
    pub /: *mut *mut unsigned int dirty_sentries; / # of dirty sentries,
    pub /: *mut *mut unsigned int sents_per_block; / # of SIT entries per block,
    pub /: *mut *mut rw_semaphore sentry_lock; / to protect SIT cache,
    pub /: *mut *mut *mut seg_entry sentries; / SIT segment-level cache,
    pub /: *mut *mut *mut sec_entry sec_entries; / SIT section-level cache,
// for cost-benefit algorithm in cleaning procedure
    pub /: *mut *mut unsigned long long elapsed_time; / elapsed time after mount,
    pub /: *mut *mut unsigned long long mounted_time; / mount time,
    pub /: *mut *mut unsigned long long min_mtime; / min. modification time,
    pub /: *mut *mut unsigned long long max_mtime; / max. modification time,
    pub /: *mut *mut unsigned long long dirty_min_mtime; / rerange candidates in GC_AT,
    pub /: *mut *mut unsigned long long dirty_max_mtime; / rerange candidates in GC_AT,
    pub /: *mut *mut unsigned int last_victim[MAX_GC_POLICY]; / last victim segment #,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct free_segmap_info {
    pub /: *mut *mut unsigned int start_segno; / start segment number logically,
    pub /: *mut *mut unsigned int free_segments; / # of free segments,
    pub /: *mut *mut unsigned int free_sections; / # of free sections,
    pub /: *mut *mut spinlock_t segmap_lock; / free segmap lock,
    pub /: *mut *mut *mut unsigned long free_segmap; / free segment bitmap,
    pub /: *mut *mut *mut unsigned long free_secmap; / free section bitmap,
}

// Notice: The order of dirty type is same with CURSEG_XXX in f2fs.h
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dirty_type {
    DIRTY_HOT_DATA,		/* dirty segments assigned as hot data logs */
    DIRTY_WARM_DATA,	/* dirty segments assigned as warm data logs */
    DIRTY_COLD_DATA,	/* dirty segments assigned as cold data logs */
    DIRTY_HOT_NODE,		/* dirty segments assigned as hot node logs */
    DIRTY_WARM_NODE,	/* dirty segments assigned as warm node logs */
    DIRTY_COLD_NODE,	/* dirty segments assigned as cold node logs */
    DIRTY,			/* to count # of dirty segments */
    PRE,			/* to count # of entirely obsolete segments */
    NR_DIRTY_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirty_seglist_info {
    pub dirty_segmap: [*mut c_ulong; NR_DIRTY_TYPE],
    pub dirty_secmap: *mut c_ulong,
    pub /: *mut *mut mutex seglist_lock; / lock for segment bitmaps,
    pub /: *mut *mut int nr_dirty[NR_DIRTY_TYPE]; / # of dirty segments,
    pub /: *mut *mut *mut unsigned long victim_secmap; / background GC victims,
    pub /: *mut *mut *mut unsigned long pinned_secmap; / pinned victims from foreground GC,
    pub /: *mut *mut unsigned int pinned_secmap_cnt; / count of victims which has pinned data,
    pub /: *mut *mut bool enable_pin_section; / enable pinning section,
}

// for active log information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct curseg_info {
    pub /: *mut *mut mutex curseg_mutex; / lock for consistency,
    pub /: *mut *mut *mut f2fs_summary_block sum_blk; / cached summary block,
    pub /: *mut *mut rw_semaphore journal_rwsem; / protect journal area,
    pub /: *mut *mut *mut f2fs_journal journal; / cached journal info,
    pub /: *mut *mut unsigned char alloc_type; / current allocation type,
    pub /: *mut *mut unsigned short seg_type; / segment type like CURSEG_XXX_TYPE,
    pub /: *mut *mut unsigned int segno; / current segment number,
    pub /: *mut *mut unsigned short next_blkoff; / next block offset to write,
    pub /: *mut *mut unsigned int zone; / current zone number,
    pub /: *mut *mut unsigned int next_segno; / preallocated segment,
    pub /: *mut *mut int fragment_remained_chunk; / remained block size in a chunk for block fragmentation mode,
    pub /: *mut *mut bool inited; / indicate inmem log is inited,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sit_entry_set {
    pub /: *mut *mut list_head set_list; / link with all sit sets,
    pub /: *mut *mut unsigned int start_segno; / start segno of sits in set,
    pub /: *mut *mut unsigned int entry_cnt; / the # of sit entries in set,
}

//
// inline functions
//
// In order to get # of valid blocks in a section instantly from many
// segments, f2fs manages two counting structures separately.
//

// check large section
extern "C" {
    pub fn GET_SEC_FROM_SEG(_arg: sbi, _arg: reserved_segments(sbi)) -> return;
}
extern "C" {
    pub fn CAP_BLKS_PER_SEC(get_ckpt_valid_blocks(sbi: sbi) -, _arg: segno, _arg: true) -> return;
}
// check current data/node sections in the worst case.
// total_node_blocks = (*total_node_blocks > min_free_node_blocks) ?
// total_node_blocks - min_free_node_blocks : 0;
// total_dent_blocks = (*total_dent_blocks > min_free_dent_blocks) ?
// total_dent_blocks - min_free_dent_blocks : 0;
// total_data_blocks = (*total_data_blocks > min_free_data_blocks) ?
// total_data_blocks - min_free_data_blocks : 0;
//
// call get_additional_blocks_required to calculate dirty blocks
// needing to be placed in free sections, please note that, it
// needs to account dirty data as well in lfs mode when checkpoint
// is disabled.
//
// When active_logs != 4, dentry blocks and data blocks can be
// mixed in the same logs, so check their space together.
//
// Sometimes f2fs may be better to drop out-of-place update policy.
// And, users can control the policy through sysfs entries.
// There are five policies with triggering conditions as follows.
// F2FS_IPU_FORCE - all the time,
// F2FS_IPU_SSR - if SSR mode is activated,
// F2FS_IPU_UTIL - if FS utilization is over threashold,
// F2FS_IPU_SSR_UTIL - if SSR mode is activated and FS utilization is over
// threashold,
// F2FS_IPU_FSYNC - activated in fsync path only for high performance flash
// storages. IPU will be triggered only if the # of dirty
// pages over min_fsync_blocks. (=default option)
// F2FS_IPU_ASYNC - do IPU given by asynchronous write requests.
// F2FS_IPU_NOCACHE - disable IPU bio cache.
// F2FS_IPU_HONOR_OPU_WRITE - use OPU write prior to IPU write if inode has
// FI_OPU_WRITE flag.
// F2FS_IPU_DISABLE - disable IPU. (=default option in LFS mode)
//
pub const DEF_MIN_IPU_UTIL: c_int = 70;
pub const DEF_MIN_FSYNC_BLOCKS: c_int = 8;
pub const DEF_MIN_HOT_BLOCKS: c_int = 16;

pub const F2FS_IPU_DISABLE: c_int = 0;
// Modification on enum should be synchronized with ipu_mode_names array

//
// Summary block is always treated as an invalid block
//
// check bitmap with valid block count
// check segment usage, and check boundary of a given segment number
// calculate sit block address
// system time is set to the past
//
// It is very important to gather dirty pages and write at once, so that we can
// submit a big bio without interfering other data writes.
// By default, 512 pages for directory data,
// 512 pages (2MB) * 8 for nodes, and
// 256 pages * 8 for meta are set.
//
extern "C" {
    pub fn BLKS_PER_SEG(_arg: sbi) -> return;
}
extern "C" {
    pub fn SEGS_TO_BLKS(_arg: sbi, _arg: 8) -> return;
}
//
// When writing pages, it'd better align nr_to_write for segment size.
//
