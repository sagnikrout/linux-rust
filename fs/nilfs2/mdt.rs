//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/mdt.h
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
// NILFS meta data file prototype and definitions
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Ryusuke Konishi.
//

//
// struct nilfs_shadow_map - shadow mapping of meta data file
// @bmap_store: shadow copy of bmap state
// @inode: holder of page caches used in shadow mapping
// @frozen_buffers: list of frozen buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_shadow_map {
    pub bmap_store: nilfs_bmap_store,
    pub inode: *mut inode,
    pub frozen_buffers: list_head,
}

//
// struct nilfs_mdt_info - on-memory private data of meta data files
// @mi_sem: reader/writer semaphore for meta data operations
// @mi_bgl: per-blockgroup locking
// @mi_entry_size: size of an entry
// @mi_first_entry_offset: offset to the first entry
// @mi_entries_per_block: number of entries in a block
// @mi_palloc_cache: persistent object allocator cache
// @mi_shadow: shadow of bmap and page caches
// @mi_blocks_per_group: number of blocks in a group
// @mi_blocks_per_desc_block: number of blocks per descriptor block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_mdt_info {
    pub mi_sem: rw_semaphore,
    pub mi_bgl: *mut blockgroup_lock,
    pub mi_entry_size: c_uint,
    pub mi_first_entry_offset: c_uint,
    pub mi_entries_per_block: c_ulong,
    pub mi_palloc_cache: *mut nilfs_palloc_cache,
    pub mi_shadow: *mut nilfs_shadow_map,
    pub mi_blocks_per_group: c_ulong,
    pub mi_blocks_per_desc_block: c_ulong,
}

// Default GFP flags using highmem

extern "C" {
    pub fn nilfs_mdt_delete_block(: *mut inode, long: unsigned) -> c_int;
}
extern "C" {
    pub fn nilfs_mdt_forget_block(: *mut inode, long: unsigned) -> c_int;
}
extern "C" {
    pub fn nilfs_mdt_fetch_dirty(: *mut inode) -> c_int;
}
extern "C" {
    pub fn nilfs_mdt_init(inode: *mut inode, gfp_mask: gfp_t, objsz: usize) -> c_int;
}
extern "C" {
    pub fn nilfs_mdt_clear(inode: *mut inode);
}
extern "C" {
    pub fn nilfs_mdt_destroy(inode: *mut inode);
}
extern "C" {
    pub fn nilfs_mdt_set_entry_size(: *mut inode, int: unsigned, int: unsigned);
}
extern "C" {
    pub fn nilfs_mdt_save_to_shadow_map(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn nilfs_mdt_restore_from_shadow_map(inode: *mut inode);
}
extern "C" {
    pub fn nilfs_mdt_clear_shadow_map(inode: *mut inode);
}
extern "C" {
    pub fn nilfs_mdt_freeze_buffer(inode: *mut inode, bh: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn bgl_lock_ptr(_arg: NILFS_MDT(inode)->mi_bgl, _arg: block_group) -> return;
}
