//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/extents_status.h
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
// fs/ext4/extents_status.h
//
// Written by Yongqiang Yang <xiaoqiangnk@gmail.com>
// Modified by
// Allison Henderson <achender@linux.vnet.ibm.com>
// Zheng Liu <wenqing.lz@taobao.com>
//
// Turn on ES_DEBUG__ to get lots of info about extent status operations.
//

//
// With ES_AGGRESSIVE_TEST defined, the result of es caching will be
// checked with old map_block's result.
//
// Macro flag: #define ES_AGGRESSIVE_TEST__
//
// These flags live in the high bits of extent_status.es_pblk
//

//
// Besides EXTENT_STATUS_REFERENCED, all these extent type masks
// are exclusive, only one type can be set at a time.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_status {
    pub rb_node: rb_node,
    pub /: *mut *mut ext4_lblk_t es_lblk; / first logical block extent covers,
    pub /: *mut *mut ext4_lblk_t es_len; / length of extent in block,
    pub /: *mut *mut ext4_fsblk_t es_pblk; / first physical block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_es_tree {
    pub root: rb_root,
    pub /: *mut *mut *mut extent_status cache_es; / recently accessed extent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_es_stats {
    pub es_stats_shrunk: c_ulong,
    pub es_stats_cache_hits: percpu_counter,
    pub es_stats_cache_misses: percpu_counter,
    pub es_stats_scan_time: u64,
    pub es_stats_max_scan_time: u64,
    pub es_stats_all_cnt: percpu_counter,
    pub es_stats_shk_cnt: percpu_counter,
}

//
// Pending cluster reservations for bigalloc file systems
//
// A cluster with a pending reservation is a logical cluster shared by at
// least one extent in the extents status tree with delayed and unwritten
// status and at least one other written or unwritten extent.  The
// reservation is said to be pending because a cluster reservation would
// have to be taken in the event all blocks in the cluster shared with
// written or unwritten extents were deleted while the delayed and
// unwritten blocks remained.
//
// The set of pending cluster reservations is an auxiliary data structure
// used with the extents status tree to implement reserved cluster/block
// accounting for bigalloc file systems.  The set is kept in memory and
// records all pending cluster reservations.
//
// Its primary function is to avoid the need to read extents from the
// disk when invalidating pages as a result of a truncate, punch hole, or
// collapse range operation.  Page invalidation requires a decrease in the
// reserved cluster count if it results in the removal of all delayed
// and unwritten extents (blocks) from a cluster that is not shared with a
// written or unwritten extent, and no decrease otherwise.  Determining
// whether the cluster is shared can be done by searching for a pending
// reservation on it.
//
// Secondarily, it provides a potentially faster method for determining
// whether the reserved cluster count should be increased when a physical
// cluster is deallocated as a result of a truncate, punch hole, or
// collapse range operation.  The necessary information is also present
// in the extents status tree, but might be more rapidly accessed in
// the pending reservation set in many cases due to smaller size.
//
// The pending cluster reservation set is implemented as a red-black tree
// with the goal of minimizing per page search time overhead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_reservation {
    pub rb_node: rb_node,
    pub lclu: ext4_lblk_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_pending_tree {
    pub root: rb_root,
}

extern "C" {
    pub fn ext4_init_es() -> int __init;
}
extern "C" {
    pub fn ext4_exit_es();
}
extern "C" {
    pub fn ext4_es_init_tree(tree: *mut ext4_es_tree);
}
extern "C" {
    pub fn ext4_es_register_shrinker(sbi: *mut ext4_sb_info) -> c_int;
}
extern "C" {
    pub fn ext4_es_unregister_shrinker(sbi: *mut ext4_sb_info);
}
extern "C" {
    pub fn ext4_seq_es_shrinker_info_show(seq: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ext4_init_pending() -> int __init;
}
extern "C" {
    pub fn ext4_exit_pending();
}
extern "C" {
    pub fn ext4_init_pending_tree(tree: *mut ext4_pending_tree);
}
extern "C" {
    pub fn ext4_remove_pending(inode: *mut inode, lblk: ext4_lblk_t);
}
extern "C" {
    pub fn ext4_is_pending(inode: *mut inode, lblk: ext4_lblk_t) -> bool;
}
extern "C" {
    pub fn ext4_clear_inode_es(inode: *mut inode);
}
