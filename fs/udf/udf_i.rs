//! Automatically rewritten from C Header to Rust Module
//! Source: fs/udf/udf_i.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_position {
    pub bh: *mut buffer_head,
    pub offset: u32,
    pub block: kernel_lb_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_ext_cache {
// Extent position
    pub epos: extent_position,
// Start logical offset in bytes
    pub lstart: loff_t,
}

//
// The i_data_sem and i_mutex serve for protection of allocation information
// of a regular files and symlinks. This includes all extents belonging to
// the file/symlink, a fact whether data are in-inode or in external data
// blocks, preallocation, goal block information... When extents are read,
// i_mutex or i_data_sem must be held (for reading is enough in case of
// i_data_sem). When extents are changed, i_data_sem must be held for writing
// and also i_mutex must be held.
//
// For directories i_mutex is used for all the necessary protection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_inode_info {
    pub i_crtime: timespec64,
// Physical address of inode
    pub i_location: kernel_lb_addr,
    pub i_unique: __u64,
    pub i_lenEAttr: __u32,
    pub i_lenAlloc: __u32,
    pub i_lenExtents: __u64,
    pub i_next_alloc_block: __u32,
    pub i_next_alloc_goal: __u32,
    pub i_checkpoint: __u32,
    pub i_extraPerms: __u32,
    pub 3: unsigned i_alloc_type :,
    pub /: *mut *mut unsigned i_efe : 1; / extendedFileEntry,
    pub /: *mut *mut unsigned i_use : 1; / unallocSpaceEntry,
    pub 1: unsigned i_strat4096 :,
    pub 1: unsigned i_streamdir :,
    pub /: *mut *mut unsigned i_hidden : 1; / hidden system inode,
    pub 24: unsigned reserved :,
    pub i_data: *mut __u8,
    pub i_locStreamdir: kernel_lb_addr,
    pub i_lenStreams: __u64,
    pub i_data_sem: rw_semaphore,
    pub i_metadata_bhs: mapping_metadata_bhs,
    pub cached_extent: udf_ext_cache,
// Spinlock for protecting extent cache
    pub i_extent_cache_lock: spinlock_t,
    pub vfs_inode: inode,
}

extern "C" {
    pub fn container_of(_arg: inode, udf_inode_info: struct, _arg: vfs_inode) -> return;
}
