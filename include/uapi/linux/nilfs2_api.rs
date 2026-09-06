//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nilfs2_api.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// nilfs2_api.h - NILFS2 user space API
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation; either version 2.1 of the License, or
// (at your option) any later version.
//

//
// struct nilfs_cpinfo - checkpoint information
// @ci_flags: flags
// @ci_pad: padding
// @ci_cno: checkpoint number
// @ci_create: creation timestamp
// @ci_nblk_inc: number of blocks incremented by this checkpoint
// @ci_inodes_count: inodes count
// @ci_blocks_count: blocks count
// @ci_next: next checkpoint number in snapshot list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_cpinfo {
    pub ci_flags: __u32,
    pub ci_pad: __u32,
    pub ci_cno: __u64,
    pub ci_create: __u64,
    pub ci_nblk_inc: __u64,
    pub ci_inodes_count: __u64,
    pub ci_blocks_count: __u64,
    pub ci_next: __u64,
}

// checkpoint flags

//
// struct nilfs_suinfo - segment usage information
// @sui_lastmod: timestamp of last modification
// @sui_nblocks: number of written blocks in segment
// @sui_flags: segment usage flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_suinfo {
    pub sui_lastmod: __u64,
    pub sui_nblocks: __u32,
    pub sui_flags: __u32,
}

// segment usage flags

//
// struct nilfs_suinfo_update - segment usage information update
// @sup_segnum: segment number
// @sup_flags: flags for which fields are active in sup_sui
// @sup_reserved: reserved necessary for alignment
// @sup_sui: segment usage information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_suinfo_update {
    pub sup_segnum: __u64,
    pub sup_flags: __u32,
    pub sup_reserved: __u32,
    pub sup_sui: nilfs_suinfo,
}

//
// struct nilfs_cpmode - change checkpoint mode structure
// @cm_cno: checkpoint number
// @cm_mode: mode of checkpoint
// @cm_pad: padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_cpmode {
    pub cm_cno: __u64,
    pub cm_mode: __u32,
    pub cm_pad: __u32,
}

//
// struct nilfs_argv - argument vector
// @v_base: pointer on data array from userspace
// @v_nmembs: number of members in data array
// @v_size: size of data array in bytes
// @v_flags: flags
// @v_index: start number of target data items
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_argv {
    pub v_base: __u64,
    pub /: *mut *mut __u32 v_nmembs; / number of members,
    pub /: *mut *mut __u16 v_size; / size of members,
    pub v_flags: __u16,
    pub v_index: __u64,
}

//
// struct nilfs_period - period of checkpoint numbers
// @p_start: start checkpoint number (inclusive)
// @p_end: end checkpoint number (exclusive)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_period {
    pub p_start: __u64,
    pub p_end: __u64,
}

//
// struct nilfs_cpstat - checkpoint statistics
// @cs_cno: checkpoint number
// @cs_ncps: number of checkpoints
// @cs_nsss: number of snapshots
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_cpstat {
    pub cs_cno: __u64,
    pub cs_ncps: __u64,
    pub cs_nsss: __u64,
}

//
// struct nilfs_sustat - segment usage statistics
// @ss_nsegs: number of segments
// @ss_ncleansegs: number of clean segments
// @ss_ndirtysegs: number of dirty segments
// @ss_ctime: creation time of the last segment
// @ss_nongc_ctime: creation time of the last segment not for GC
// @ss_prot_seq: least sequence number of segments which must not be reclaimed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_sustat {
    pub ss_nsegs: __u64,
    pub ss_ncleansegs: __u64,
    pub ss_ndirtysegs: __u64,
    pub ss_ctime: __u64,
    pub ss_nongc_ctime: __u64,
    pub ss_prot_seq: __u64,
}

//
// struct nilfs_vinfo - virtual block number information
// @vi_vblocknr: virtual block number
// @vi_start: start checkpoint number (inclusive)
// @vi_end: end checkpoint number (exclusive)
// @vi_blocknr: disk block number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_vinfo {
    pub vi_vblocknr: __u64,
    pub vi_start: __u64,
    pub vi_end: __u64,
    pub vi_blocknr: __u64,
}

//
// struct nilfs_vdesc - descriptor of virtual block number
// @vd_ino: inode number
// @vd_cno: checkpoint number
// @vd_vblocknr: virtual block number
// @vd_period: period of checkpoint numbers
// @vd_blocknr: disk block number
// @vd_offset: logical block offset inside a file
// @vd_flags: flags (data or node block)
// @vd_pad: padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_vdesc {
    pub vd_ino: __u64,
    pub vd_cno: __u64,
    pub vd_vblocknr: __u64,
    pub vd_period: nilfs_period,
    pub vd_blocknr: __u64,
    pub vd_offset: __u64,
    pub vd_flags: __u32,
    pub vd_pad: __u32,
}

//
// struct nilfs_bdesc - descriptor of disk block number
// @bd_ino: inode number
// @bd_oblocknr: disk block address (for skipping dead blocks)
// @bd_blocknr: disk block address
// @bd_offset: logical block offset inside a file
// @bd_level: level in the b-tree organization
// @bd_pad: padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_bdesc {
    pub bd_ino: __u64,
    pub bd_oblocknr: __u64,
    pub bd_blocknr: __u64,
    pub bd_offset: __u64,
    pub bd_level: __u32,
    pub bd_pad: __u32,
}

