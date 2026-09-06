//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/ordered-data.h
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
// Copyright (C) 2007 Oracle.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ordered_sum {
//
// Logical start address and length for of the blocks covered by
// the sums array.
//
    pub logical: u64,
    pub len: u32,
    pub list: list_head,
// last field is a variable length array of csums
    pub sums: [u8; ],
}

//
// Bits for btrfs_ordered_extent::flags.
//
// BTRFS_ORDERED_IO_DONE is set when all of the blocks are written.
// It is used to make sure metadata is inserted into the tree only once
// per extent.
//
// BTRFS_ORDERED_COMPLETE is set when the extent is removed from the
// rbtree, just before waking any waiters.  It is used to indicate the
// IO is done and any metadata is inserted into the tree.
//
// Extra status bits for ordered extents
// Set when all the pages are written.
// Set when removed from the tree.
// We had an io error when writing this out.
// Set when we have to truncate an extent.
// Used during fsync to track already logged extents.
// We have already logged all the csums of the ordered extent.
// We wait for this extent to complete in the current transaction.
//
// Different types for ordered extents, one and only one of these types
// need to be set when creating ordered extent.
//
// REGULAR:	For regular non-compressed COW write
// NOCOW:	For NOCOW write into existing non-hole extent
// PREALLOC:	For NOCOW write into preallocated extent
// COMPRESSED:	For compressed COW write
//
// Extra bit for encoded write, must be set with COMPRESSED.
//
// Extra bit for direct io, can only be set for
// REGULAR/NOCOW/PREALLOC. Must not be set for COMPRESSED nor ENCODED.
//
// One and only one flag can be set.

// BTRFS_ORDERED_* flags that specify the type of the extent.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ordered_extent {
// logical offset in the file
    pub file_offset: u64,
//
// These fields directly correspond to the same fields in
// btrfs_file_extent_item.
//
    pub num_bytes: u64,
    pub ram_bytes: u64,
    pub disk_bytenr: u64,
    pub disk_num_bytes: u64,
    pub offset: u64,
// number of bytes that still need writing
    pub bytes_left: u64,
//
// If we get truncated we need to adjust the file extent we enter for
// this ordered extent so that we do not expose stale data.
//
    pub truncated_len: u64,
// flags (described above)
    pub flags: c_ulong,
// compression algorithm
    pub compress_type: c_int,
// Qgroup reserved space
    pub qgroup_rsv: c_int,
// reference count
    pub refs: refcount_t,
// the inode we belong to
    pub inode: *mut btrfs_inode,
// list of checksums for insertion when the extent io is done
    pub csum_list: list_head,
// used for fast fsyncs
    pub log_list: list_head,
// used to wait for the BTRFS_ORDERED_COMPLETE bit
    pub wait: wait_queue_head_t,
// our friendly rbtree entry
    pub rb_node: rb_node,
// a per root list of all the pending ordered extents
    pub root_extent_list: list_head,
    pub work: btrfs_work,
    pub completion: completion,
    pub flush_work: btrfs_work,
    pub work_list: list_head,
    pub bioc_list: list_head,
}

extern "C" {
    pub fn btrfs_finish_one_ordered(ordered_extent: *mut btrfs_ordered_extent) -> c_int;
}
extern "C" {
    pub fn btrfs_finish_ordered_io(ordered_extent: *mut btrfs_ordered_extent) -> c_int;
}
extern "C" {
    pub fn btrfs_put_ordered_extent(entry: *mut btrfs_ordered_extent);
}
extern "C" {
    pub fn btrfs_remove_ordered_extent(entry: *mut btrfs_ordered_extent);
}
//
// This represents details about the target file extent item of a write operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_file_extent {
    pub disk_bytenr: u64,
    pub disk_num_bytes: u64,
    pub num_bytes: u64,
    pub ram_bytes: u64,
    pub offset: u64,
    pub compression: u8,
}

extern "C" {
    pub fn btrfs_start_ordered_extent_nowriteback(_arg: entry, _arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn btrfs_wait_ordered_range(inode: *mut btrfs_inode, start: u64, len: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_mark_ordered_extent_error(ordered: *mut btrfs_ordered_extent);
}
extern "C" {
    pub fn ordered_data_init() -> int __init;
}
extern "C" {
    pub fn ordered_data_exit() -> void __cold;
}
