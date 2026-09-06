//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/segment.h
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
// NILFS Segment constructor prototypes and definitions
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Ryusuke Konishi.
//

//
// struct nilfs_recovery_info - Recovery information
// @ri_need_recovery: Recovery status
// @ri_super_root: Block number of the last super root
// @ri_cno: Number of the last checkpoint
// @ri_lsegs_start: Region for roll-forwarding (start block number)
// @ri_lsegs_end: Region for roll-forwarding (end block number)
// @ri_lsegs_start_seq: Sequence value of the segment at ri_lsegs_start
// @ri_used_segments: List of segments to be mark active
// @ri_pseg_start: Block number of the last partial segment
// @ri_seq: Sequence number on the last partial segment
// @ri_segnum: Segment number on the last partial segment
// @ri_nextnum: Next segment number on the last partial segment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_recovery_info {
    pub ri_need_recovery: c_int,
    pub ri_super_root: sector_t,
    pub ri_cno: __u64,
    pub ri_lsegs_start: sector_t,
    pub ri_lsegs_end: sector_t,
    pub ri_lsegs_start_seq: u64,
    pub ri_used_segments: list_head,
    pub ri_pseg_start: sector_t,
    pub ri_seq: u64,
    pub ri_segnum: __u64,
    pub ri_nextnum: __u64,
}

// ri_need_recovery

//
// struct nilfs_cstage - Context of collection stage
// @scnt: Stage count, must be accessed via wrappers:
// nilfs_sc_cstage_inc(), nilfs_sc_cstage_set(), nilfs_sc_cstage_get()
// @flags: State flags
// @dirty_file_ptr: Pointer on dirty_files list, or inode of a target file
// @gc_inode_ptr: Pointer on the list of gc-inodes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_cstage {
    pub scnt: c_int,
    pub flags: c_uint,
    pub dirty_file_ptr: *mut nilfs_inode_info,
    pub gc_inode_ptr: *mut nilfs_inode_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_segsum_pointer {
    pub bh: *mut buffer_head,
    pub /: *mut *mut unsigned int offset; / offset in bytes,
}

//
// struct nilfs_sc_info - Segment constructor information
// @sc_super: Back pointer to super_block struct
// @sc_root: root object of the current filesystem tree
// @sc_nblk_inc: Block count of current generation
// @sc_dirty_files: List of files to be written
// @sc_gc_inodes: List of GC inodes having blocks to be written
// @sc_iput_queue: list of inodes for which iput should be done
// @sc_iput_work: work struct to defer iput call
// @sc_freesegs: array of segment numbers to be freed
// @sc_nfreesegs: number of segments on @sc_freesegs
// @sc_dsync_inode: inode whose data pages are written for a sync operation
// @sc_dsync_start: start byte offset of data pages
// @sc_dsync_end: end byte offset of data pages (inclusive)
// @sc_segbufs: List of segment buffers
// @sc_write_logs: List of segment buffers to hold logs under writing
// @sc_segbuf_nblocks: Number of available blocks in segment buffers.
// @sc_curseg: Current segment buffer
// @sc_stage: Collection stage
// @sc_finfo_ptr: pointer to the current finfo struct in the segment summary
// @sc_binfo_ptr: pointer to the current binfo struct in the segment summary
// @sc_blk_cnt:	Block count of a file
// @sc_datablk_cnt: Data block count of a file
// @sc_nblk_this_inc: Number of blocks included in the current logical segment
// @sc_seg_ctime: Creation time
// @sc_cno: checkpoint number of current log
// @sc_flags: Internal flags
// @sc_state_lock: spinlock for sc_state and so on
// @sc_state: Segctord state flags
// @sc_flush_request: inode bitmap of metadata files to be flushed
// @sc_wait_request: Client request queue
// @sc_wait_daemon: Daemon wait queue
// @sc_seq_request: Request counter
// @sc_seq_accepted: Accepted request count
// @sc_seq_done: Completion counter
// @sc_sync: Request of explicit sync operation
// @sc_interval: Timeout value of background construction
// @sc_mjcp_freq: Frequency of creating checkpoints
// @sc_lseg_stime: Start time of the latest logical segment
// @sc_watermark: Watermark for the number of dirty buffers
// @sc_timer: Timer for segctord
// @sc_task: current thread of segctord
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_sc_info {
    pub sc_super: *mut super_block,
    pub sc_root: *mut nilfs_root,
    pub sc_nblk_inc: c_ulong,
    pub sc_dirty_files: list_head,
    pub sc_gc_inodes: list_head,
    pub sc_iput_queue: list_head,
    pub sc_iput_work: work_struct,
    pub sc_freesegs: *mut __u64,
    pub sc_nfreesegs: usize,
    pub sc_dsync_inode: *mut nilfs_inode_info,
    pub sc_dsync_start: loff_t,
    pub sc_dsync_end: loff_t,
// Segment buffers
    pub sc_segbufs: list_head,
    pub sc_write_logs: list_head,
    pub sc_segbuf_nblocks: c_ulong,
    pub sc_curseg: *mut nilfs_segment_buffer,
    pub sc_stage: nilfs_cstage,
    pub sc_finfo_ptr: nilfs_segsum_pointer,
    pub sc_binfo_ptr: nilfs_segsum_pointer,
    pub sc_blk_cnt: c_ulong,
    pub sc_datablk_cnt: c_ulong,
    pub sc_nblk_this_inc: c_ulong,
    pub sc_seg_ctime: time64_t,
    pub sc_cno: __u64,
    pub sc_flags: c_ulong,
    pub sc_state_lock: spinlock_t,
    pub sc_state: c_ulong,
    pub sc_flush_request: c_ulong,
    pub sc_wait_request: wait_queue_head_t,
    pub sc_wait_daemon: wait_queue_head_t,
    pub sc_seq_request: __u32,
    pub sc_seq_accepted: __u32,
    pub sc_seq_done: __u32,
    pub sc_sync: c_int,
    pub sc_interval: c_ulong,
    pub sc_mjcp_freq: c_ulong,
    pub /: *mut *mut unsigned long sc_lseg_stime; / in 1/HZ seconds,
    pub sc_watermark: c_ulong,
    pub sc_timer: timer_list,
    pub sc_task: *mut task_struct,
}

// sc_flags
// Requesting immediate flush without making a
// checkpoint
//
// Next checkpoint will have update of files
// other than DAT, cpfile, sufile, or files
// moved by GC.
//
// sc_state
pub const NILFS_SEGCTOR_COMMIT: c_uint = 0x0004  /* committed transaction exists */;
//
// Constant parameters
//

// Retry count of construction when
// destroying segctord
//
// Default values of timeout, in seconds.
//

// Timeout value of dirty blocks.
// It triggers construction of a
// logical segment with a super root.
//

// Maximum frequency of super root
// creation
//
// The default threshold amount of data, in block counts.
//
pub const NILFS_SC_DEFAULT_WATERMARK: c_int = 3600;
// super.c
// segment.c
extern "C" {
    pub fn nilfs_relax_pressure_in_lock(: *mut super_block);
}
extern "C" {
    pub fn nilfs_construct_segment(: *mut super_block) -> c_int;
}
extern "C" {
    pub fn nilfs_attach_log_writer(sb: *mut super_block, root: *mut nilfs_root) -> c_int;
}
extern "C" {
    pub fn nilfs_detach_log_writer(sb: *mut super_block);
}
// recovery.c
extern "C" {
    pub fn nilfs_dispose_segment_list(: *mut list_head);
}
