//! Automatically rewritten from C Header to Rust Module
//! Source: fs/f2fs/node.h
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
// fs/f2fs/node.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// start node id of a node block dedicated to the given node id

// node block offset on the NAT area dedicated to the given start node id

// # of pages to perform synchronous readahead before building free nids
pub const FREE_NID_PAGES: c_int = 8;

// size of free nid batch when shrinking
pub const SHRINK_NID_BATCH_SIZE: c_int = 8;

// maximum readahead size for node during getting data blocks
pub const MAX_RA_NODE: c_int = 128;
// control the memory footprint threshold (10MB per 1GB ram)
pub const DEF_RAM_THRESHOLD: c_int = 1;
// control dirty nats ratio threshold (default: 10% over max nid count)
pub const DEF_DIRTY_NAT_RATIO_THRESHOLD: c_int = 10;
// control total # of nats
pub const DEF_NAT_CACHE_THRESHOLD: c_int = 100000;
// control total # of node writes used for roll-forward recovery
pub const DEF_RF_NODE_BLOCKS: c_int = 0;
// vector size for gang look-up from nat cache that consists of radix tree
pub const NAT_VEC_SIZE: c_int = 32;
// return value for read_node_page
pub const LOCKED_PAGE: c_int = 1;
// check pinned file's alignment status of physical blocks
pub const FILE_NOT_ALIGNED: c_int = 1;
// For flag in struct node_info
//
// For node information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_info {
    pub /: *mut *mut nid_t nid; / node id,
    pub /: *mut *mut nid_t ino; / inode number of the node's owner,
    pub /: *mut *mut block_t blk_addr; / block address of the node,
    pub /: *mut *mut unsigned char version; / version of the node,
    pub /: *mut *mut unsigned char flag; / for node information bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nat_entry {
    pub /: *mut *mut list_head list; / for clean or dirty nat list,
    pub /: *mut *mut node_info ni; / in-memory node information,
}

// should not copy flag here
// these states can be set only after checkpoint was done
// nat_cnt[] is heuristic accounting sampled locklessly here.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_type {
    FREE_NIDS,	/* indicates the free nid list */
    NAT_ENTRIES,	/* indicates the cached nat entry */
    DIRTY_DENTS,	/* indicates dirty dentry pages */
    INO_ENTRIES,	/* indicates inode entries */
    READ_EXTENT_CACHE,	/* indicates read extent cache */
    AGE_EXTENT_CACHE,	/* indicates age extent cache */
    DISCARD_CACHE,	/* indicates memory of cached discard cmds */
    COMPRESS_PAGE,	/* indicates memory of cached compressed pages */
    BASE_CHECK,	/* check kernel status */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nat_entry_set {
    pub /: *mut *mut list_head set_list; / link with other nat sets,
    pub /: *mut *mut list_head entry_list; / link with dirty nat entries,
    pub number*/: *mut *mut nid_t set; / set,
    pub /: *mut *mut unsigned int entry_cnt; / the # of nat entries in set,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct free_nid {
    pub /: *mut *mut list_head list; / for free node id list,
    pub /: *mut *mut nid_t nid; / node id,
    pub /: *mut *mut int state; / in use or not: FREE_NID or PREALLOC_NID,
}

// nid = fnid->nid;
//
// inline functions
//

//
// block_off = segment_off * 512 + off_in_segment
// OLD = (segment_off * 512) * 2 + off_in_segment
// NEW = 2 * (segment_off * 512 + off_in_segment) - off_in_segment
//

extern "C" {
    pub fn le32_to_cpu(_arg: rn->footer.ino) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: rn->footer.nid) -> return;
}
extern "C" {
    pub fn le64_to_cpu(_arg: rn->footer.cp_ver) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: rn->footer.next_blkaddr) -> return;
}
// should remain old flag bits such as COLD_BIT_SHIFT
// Don't care crc part, if fsck.f2fs sets it.
//
// f2fs assigns the following node offsets described as (num).
// N = NIDS_PER_BLOCK
//
// Inode block (0)
// |- direct node (1)
// |- direct node (2)
// |- indirect node (3)
// |            `- direct node (4 => 4 + N - 1)
// |- indirect node (4 + N)
// |            `- direct node (5 + N => 5 + 2N - 1)
// `- double indirect node (5 + 2N)
// `- indirect node (6 + 2N)
// `- direct node
// ......
// `- indirect node ((6 + 2N) + x(N + 1))
// `- direct node
// ......
// `- indirect node ((6 + 2N) + (N - 1)(N + 1))
// `- direct node
//
extern "C" {
    pub fn folio_mark_dirty(_arg: folio) -> return;
}
extern "C" {
    pub fn le32_to_cpu(NODE_DIR1_BLOCK]: rn->i.i_nid[off -) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: rn->in.nid[off]) -> return;
}
//
// Coldness identification:
// - Mark cold files in f2fs_inode_info
// - Mark cold node blocks in their node footer
// - Mark cold data pages in page cache
//
extern "C" {
    pub fn le32_to_cpu(BIT(type: rn->footer.flag) &) -> return;
}

