//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/segbuf.h
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
// NILFS Segment buffer prototypes and definitions
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Ryusuke Konishi.
//

//
// struct nilfs_segsum_info - On-memory segment summary
// @flags: Flags
// @nfinfo: Number of file information structures
// @nblocks: Number of blocks included in the partial segment
// @nsumblk: Number of summary blocks
// @sumbytes: Byte count of segment summary
// @nfileblk: Total number of file blocks
// @seg_seq: Segment sequence number
// @cno: Checkpoint number
// @ctime: Creation time
// @next: Block number of the next full segment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_segsum_info {
    pub flags: c_uint,
    pub nfinfo: c_ulong,
    pub nblocks: c_ulong,
    pub nsumblk: c_ulong,
    pub sumbytes: c_ulong,
    pub nfileblk: c_ulong,
    pub seg_seq: u64,
    pub cno: __u64,
    pub ctime: time64_t,
    pub next: sector_t,
}

//
// struct nilfs_segment_buffer - Segment buffer
// @sb_super: back pointer to a superblock struct
// @sb_list: List head to chain this structure
// @sb_sum: On-memory segment summary
// @sb_segnum: Index number of the full segment
// @sb_nextnum: Index number of the next full segment
// @sb_fseg_start: Start block number of the full segment
// @sb_fseg_end: End block number of the full segment
// @sb_pseg_start: Disk block number of partial segment
// @sb_rest_blocks: Number of residual blocks in the current segment
// @sb_segsum_buffers: List of buffers for segment summaries
// @sb_payload_buffers: List of buffers for segment payload
// @sb_super_root: Pointer to buffer storing a super root block (if exists)
// @sb_nbio: Number of flying bio requests
// @sb_err: I/O error status
// @sb_bio_event: Completion event of log writing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_segment_buffer {
    pub sb_super: *mut super_block,
    pub sb_list: list_head,
// Segment information
    pub sb_sum: nilfs_segsum_info,
    pub sb_segnum: __u64,
    pub sb_nextnum: __u64,
    pub sb_fseg_end: sector_t sb_fseg_start,,
    pub sb_pseg_start: sector_t,
    pub sb_rest_blocks: c_uint,
// Buffers
    pub sb_segsum_buffers: list_head,
    pub /: *mut *mut list_head sb_payload_buffers; / including super root,
    pub sb_super_root: *mut buffer_head,
// io status
    pub sb_nbio: c_int,
    pub sb_err: core::sync::atomic::AtomicI32,
    pub sb_bio_event: completion,
}

extern "C" {
    pub fn nilfs_segbuf_free(: *mut nilfs_segment_buffer);
}
extern "C" {
    pub fn nilfs_segbuf_extend_segsum(: *mut nilfs_segment_buffer) -> c_int;
}
extern "C" {
    pub fn nilfs_segbuf_fill_in_segsum(: *mut nilfs_segment_buffer);
}
extern "C" {
    pub fn nilfs_clear_logs(logs: *mut list_head);
}
extern "C" {
    pub fn nilfs_write_logs(logs: *mut list_head, nilfs: *mut the_nilfs) -> c_int;
}
extern "C" {
    pub fn nilfs_wait_on_logs(logs: *mut list_head) -> c_int;
}
extern "C" {
    pub fn nilfs_add_checksums_on_logs(logs: *mut list_head, seed: u32);
}
