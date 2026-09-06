//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/mballoc.h
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
// fs/ext4/mballoc.h
//
// Written by: Alex Tomas <alex@clusterfs.com>
//

//
// mb_debug() dynamic printk msgs could be used to debug mballoc code.
//

//
// How long mballoc can look for a best extent (in found extents)
//
pub const MB_DEFAULT_MAX_TO_SCAN: c_int = 200;
//
// How long mballoc must look for a best extent
//
pub const MB_DEFAULT_MIN_TO_SCAN: c_int = 10;
//
// with 's_mb_stats' allocator will collect stats that will be
// shown at umount. The collecting costs though!
//
pub const MB_DEFAULT_STATS: c_int = 0;
//
// files smaller than MB_DEFAULT_STREAM_THRESHOLD are served
// by the stream allocator, which purpose is to pack requests
// as close each to other as possible to produce smooth I/O traffic
// We use locality group prealloc space for stream request.
// We can tune the same via /proc/fs/ext4/<partition>/stream_req
//

//
// for which requests use 2^N search using buddies
//
pub const MB_DEFAULT_ORDER2_REQS: c_int = 2;
//
// default group prealloc size 512 blocks
//
pub const MB_DEFAULT_GROUP_PREALLOC: c_int = 512;
//
// Number of groups to search linearly before performing group scanning
// optimization.
//
pub const MB_DEFAULT_LINEAR_LIMIT: c_int = 4;
//
// Minimum number of groups that should be present in the file system to perform
// group scanning optimizations.
//
pub const MB_DEFAULT_LINEAR_SCAN_THRESHOLD: c_int = 16;
//
// The maximum order upto which CR_BEST_AVAIL_LEN can trim a particular
// allocation request. Example, if we have an order 7 request and max trim order
// of 3, we can trim this request upto order 4.
//
pub const MB_DEFAULT_BEST_AVAIL_TRIM_ORDER: c_int = 3;
//
// Number of valid buddy orders
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_free_data {
// this links the free block information from sb_info
    pub efd_list: list_head,
// this links the free block information from group_info
    pub efd_node: rb_node,
// group which free block extent belongs
    pub efd_group: ext4_group_t,
// free block extent
    pub efd_start_cluster: ext4_grpblk_t,
    pub efd_count: ext4_grpblk_t,
// transaction which freed this extent
    pub efd_tid: tid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_prealloc_space {
    pub /: *mut *mut rb_node inode_node; / for inode PA rbtree,
    pub /: *mut *mut list_head lg_list; / for lg PAs,
    pub pa_node: },
    pub pa_group_list: list_head,
    pub pa_tmp_list: list_head,
    pub pa_rcu: rcu_head,
    pub u: },
    pub pa_lock: spinlock_t,
    pub pa_count: core::sync::atomic::AtomicI32,
    pub pa_deleted: unsigned,
    pub /: *mut *mut ext4_fsblk_t pa_pstart; / phys. block,
    pub /: *mut *mut ext4_lblk_t pa_lstart; / log. block,
    pub /: *mut *mut ext4_grpblk_t pa_len; / len of preallocated chunk,
    pub /: *mut *mut ext4_grpblk_t pa_free; / how many blocks are free,
    pub /: *mut *mut unsigned short pa_type; / pa type. inode or group,
    pub /: *mut *mut *mut rwlock_t inode_lock; / locks the rbtree holding this PA,
    pub /: *mut *mut *mut spinlock_t lg_lock; / locks the lg list holding this PA,
    pub pa_node_lock: },
    pub /: *mut *mut *mut inode pa_inode; / used to get the inode during group discard,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_free_extent {
    pub fe_logical: ext4_lblk_t,
    pub /: *mut *mut ext4_grpblk_t fe_start; / In cluster units,
    pub fe_group: ext4_group_t,
    pub /: *mut *mut ext4_grpblk_t fe_len; / In cluster units,
}

//
// Locality group:
// we try to group all related changes together
// so that writeback can flush/allocate them together as well
// Size of lg_prealloc_list hash is determined by MB_DEFAULT_GROUP_PREALLOC
// (512). We store prealloc space into the hash based on the pa_free blocks
// order value.ie, fls(pa_free)-1;
//
pub const PREALLOC_TB_SIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_locality_group {
// for allocator
// to serialize allocates
    pub lg_mutex: mutex,
// list of preallocations
    pub lg_prealloc_list: [list_head; PREALLOC_TB_SIZE],
    pub lg_prealloc_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_allocation_context {
    pub ac_inode: *mut inode,
    pub ac_sb: *mut super_block,
// original request
    pub ac_o_ex: ext4_free_extent,
// goal request (normalized ac_o_ex)
    pub ac_g_ex: ext4_free_extent,
// the best found extent
    pub ac_b_ex: ext4_free_extent,
// copy of the best found extent taken before preallocation efforts
    pub ac_f_ex: ext4_free_extent,
//
// goal len can change in CR_BEST_AVAIL_LEN, so save the original len.
// This is used while adjusting the PA window and for accounting.
//
    pub ac_orig_goal_len: ext4_grpblk_t,
    pub ac_prefetch_grp: ext4_group_t,
    pub ac_prefetch_ios: c_uint,
    pub ac_prefetch_nr: c_uint,
    pub ac_first_err: c_int,
    pub /: *mut *mut __u32 ac_flags; / allocation hints,
    pub ac_groups_scanned: __u16,
    pub ac_found: __u16,
    pub ac_cX_found: [__u16; EXT4_MB_NUM_CRS],
    pub ac_tail: __u16,
    pub ac_buddy: __u16,
    pub ac_status: __u8,
    pub ac_criteria: __u8,
    pub and: *mut *mut __u8 ac_2order; / if request is to allocate 2^N blocks,
// N > 0, the field stores N, otherwise 0
    pub /: *mut *mut __u8 ac_op; / operation, for history only,
    pub ac_e4b: *mut ext4_buddy,
    pub ac_bitmap_folio: *mut folio,
    pub ac_buddy_folio: *mut folio,
    pub ac_pa: *mut ext4_prealloc_space,
    pub ac_lg: *mut ext4_locality_group,
}

pub const AC_STATUS_CONTINUE: c_int = 1;
pub const AC_STATUS_FOUND: c_int = 2;
pub const AC_STATUS_BREAK: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_buddy {
    pub bd_buddy_folio: *mut folio,
    pub bd_buddy: *mut c_void,
    pub bd_bitmap_folio: *mut folio,
    pub bd_bitmap: *mut c_void,
    pub bd_info: *mut ext4_group_info,
    pub bd_sb: *mut super_block,
    pub bd_blkbits: __u16,
    pub bd_group: ext4_group_t,
}

// Use loff_t to avoid end exceeding ext4_lblk_t max.

extern "C" {
    pub fn mb_clear_bits_test(bm: *mut c_void, cur: c_int, len: c_int);
}
extern "C" {
    pub fn mb_find_next_zero_bit_test(addr: *mut c_void, max: c_int, start: c_int) -> c_int;
}
extern "C" {
    pub fn mb_find_next_bit_test(addr: *mut c_void, max: c_int, start: c_int) -> c_int;
}
extern "C" {
    pub fn mb_clear_bit_test(bit: c_int, addr: *mut c_void);
}
extern "C" {
    pub fn mb_test_bit_test(bit: c_int, addr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ext4_mb_unload_buddy_test(e4b: *mut ext4_buddy);
}

