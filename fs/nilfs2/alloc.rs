//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/alloc.h
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
// Persistent object (dat entry/disk inode) allocator/deallocator
//
// Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
//
// Originally written by Koji Sato.
// Two allocators were unified by Ryusuke Konishi and Amagai Yoshiji.
//

//
// nilfs_palloc_entries_per_group - get the number of entries per group
// @inode: inode of metadata file using this allocator
//
// The number of entries per group is defined by the number of bits
// that a bitmap block can maintain.
//
// Return: Number of entries per group.
//
extern "C" {
    pub fn nilfs_palloc_init_blockgroup(: *mut inode, int: unsigned) -> c_int;
}
extern "C" {
    pub fn nilfs_palloc_count_max_entries(: *mut inode, _arg: u64, : *mut u64) -> c_int;
}
//
// struct nilfs_palloc_req - persistent allocator request and reply
// @pr_entry_nr: entry number (vblocknr or inode number)
// @pr_desc_bh: buffer head of the buffer containing block group descriptors
// @pr_bitmap_bh: buffer head of the buffer containing a block group bitmap
// @pr_entry_bh: buffer head of the buffer containing translation entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_palloc_req {
    pub pr_entry_nr: __u64,
    pub pr_desc_bh: *mut buffer_head,
    pub pr_bitmap_bh: *mut buffer_head,
    pub pr_entry_bh: *mut buffer_head,
}

extern "C" {
    pub fn nilfs_palloc_abort_alloc_entry(: *mut inode, : *mut nilfs_palloc_req);
}
extern "C" {
    pub fn nilfs_palloc_commit_free_entry(: *mut inode, : *mut nilfs_palloc_req);
}
extern "C" {
    pub fn nilfs_palloc_prepare_free_entry(: *mut inode, : *mut nilfs_palloc_req) -> c_int;
}
extern "C" {
    pub fn nilfs_palloc_abort_free_entry(: *mut inode, : *mut nilfs_palloc_req);
}
extern "C" {
    pub fn nilfs_palloc_freev(: *mut inode, : *mut __u64, _arg: usize) -> c_int;
}

//
// struct nilfs_bh_assoc - block offset and buffer head association
// @blkoff: block offset
// @bh: buffer head
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_bh_assoc {
    pub blkoff: c_ulong,
    pub bh: *mut buffer_head,
}

//
// struct nilfs_palloc_cache - persistent object allocator cache
// @lock: cache protecting lock
// @prev_desc: blockgroup descriptors cache
// @prev_bitmap: blockgroup bitmap cache
// @prev_entry: translation entries cache
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_palloc_cache {
    pub lock: spinlock_t,
    pub prev_desc: nilfs_bh_assoc,
    pub prev_bitmap: nilfs_bh_assoc,
    pub prev_entry: nilfs_bh_assoc,
}

extern "C" {
    pub fn nilfs_palloc_clear_cache(inode: *mut inode);
}
extern "C" {
    pub fn nilfs_palloc_destroy_cache(inode: *mut inode);
}
