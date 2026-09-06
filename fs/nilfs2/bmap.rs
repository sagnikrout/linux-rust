//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/bmap.h
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
// NILFS block mapping.
//
// Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Koji Sato.
//

pub const NILFS_BMAP_INVALID_PTR: c_int = 0;

//
// union nilfs_bmap_ptr_req - request for bmap ptr
// @bpr_ptr: bmap pointer
// @bpr_req: request for persistent allocator
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nilfs_bmap_ptr_req {
    pub bpr_ptr: __u64,
    pub bpr_req: nilfs_palloc_req,
}

//
// struct nilfs_bmap_stats - bmap statistics
// @bs_nblocks: number of blocks created or deleted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_bmap_stats {
    pub bs_nblocks: c_uint,
}

//
// struct nilfs_bmap_operations - bmap operation table
// @bop_lookup:               single block search operation
// @bop_lookup_contig:        consecutive block search operation
// @bop_insert:               block insertion operation
// @bop_delete:               block delete operation
// @bop_clear:                block mapping resource release operation
// @bop_propagate:            operation to propagate dirty state towards the
// mapping root
// @bop_lookup_dirty_buffers: operation to collect dirty block buffers
// @bop_assign:               disk block address assignment operation
// @bop_mark:                 operation to mark in-use blocks as dirty for
// relocation by GC
// @bop_seek_key:             find valid block key operation
// @bop_last_key:             find last valid block key operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_bmap_operations {
    pub ): *const *const *const int (bop_lookup)(struct nilfs_bmap , __u64, int, __u64,
    pub int): unsigned,
    pub __u64): *mut *mut *mut int (bop_insert)(struct nilfs_bmap , __u64,,
    pub deform): *mut *mut *mut int (bop_delete)(struct nilfs_bmap bmap, __u64 key, bool,
    pub ): *mut *mut void (bop_clear)(struct nilfs_bmap,
    pub ): *mut *mut *mut int (bop_propagate)(struct nilfs_bmap , struct buffer_head,
    pub ): *mut list_head,
    pub ): *mut nilfs_binfo,
    pub int): *mut *mut *mut int (bop_mark)(struct nilfs_bmap , __u64,,
    pub ): *const *const *const int (bop_seek_key)(struct nilfs_bmap , __u64, __u64,
    pub ): *const *const *const int (bop_last_key)(struct nilfs_bmap , __u64,
// private: internal use only
    pub __u64): *const *const *const int (bop_check_insert)(struct nilfs_bmap ,,
    pub __u64): *mut *mut *mut int (bop_check_delete)(struct nilfs_bmap ,,
    pub int): *mut *mut *mut *mut *mut int (bop_gather_data)(struct nilfs_bmap , __u64 , __u64 ,,
}

//
// struct nilfs_bmap - bmap structure
// @b_u: raw data
// @b_sem: semaphore
// @b_inode: owner of bmap
// @b_ops: bmap operation table
// @b_last_allocated_key: last allocated key for data block
// @b_last_allocated_ptr: last allocated ptr for data block
// @b_ptr_type: pointer type
// @b_state: state
// @b_nchildren_per_block: maximum number of child nodes for non-root nodes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_bmap {
    pub u_flags: __u8,
    pub sizeof(__le64)]: __le64 u_data[NILFS_BMAP_SIZE /,
    pub b_u: },
    pub b_sem: rw_semaphore,
    pub b_inode: *mut inode,
    pub b_ops: *const nilfs_bmap_operations,
    pub b_last_allocated_key: __u64,
    pub b_last_allocated_ptr: __u64,
    pub b_ptr_type: c_int,
    pub b_state: c_int,
    pub b_nchildren_per_block: __u16,
}

// pointer type

// virtual block number (single
// version)
//

// virtual block number (has multiple
// versions)
//

// state
pub const NILFS_BMAP_DIRTY: c_uint = 0x00000001;
//
// struct nilfs_bmap_store - shadow copy of bmap state
// @data: cached raw block mapping of on-disk inode
// @last_allocated_key: cached value of last allocated key for data block
// @last_allocated_ptr: cached value of last allocated ptr for data block
// @state: cached value of state field of bmap structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_bmap_store {
    pub sizeof(__le64)]: __le64 data[NILFS_BMAP_SIZE /,
    pub last_allocated_key: __u64,
    pub last_allocated_ptr: __u64,
    pub state: c_int,
}

extern "C" {
    pub fn nilfs_bmap_test_and_clear_dirty(: *mut nilfs_bmap) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_read(: *mut nilfs_bmap, : *mut nilfs_inode) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_write(: *mut nilfs_bmap, : *mut nilfs_inode);
}
extern "C" {
    pub fn nilfs_bmap_lookup_contig(: *mut nilfs_bmap, _arg: __u64, : *mut __u64, int: unsigned) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_insert(bmap: *mut nilfs_bmap, key: __u64, rec: c_ulong) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_delete(bmap: *mut nilfs_bmap, key: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_seek_key(bmap: *mut nilfs_bmap, start: __u64, keyp: *mut __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_last_key(bmap: *mut nilfs_bmap, keyp: *mut __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_truncate(bmap: *mut nilfs_bmap, key: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_clear(: *mut nilfs_bmap);
}
extern "C" {
    pub fn nilfs_bmap_propagate(: *mut nilfs_bmap, : *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_lookup_dirty_buffers(: *mut nilfs_bmap, : *mut list_head);
}
extern "C" {
    pub fn nilfs_bmap_lookup_at_level(: *mut nilfs_bmap, _arg: __u64, _arg: c_int, : *mut __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_mark(: *mut nilfs_bmap, _arg: __u64, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nilfs_bmap_init_gc(: *mut nilfs_bmap);
}
extern "C" {
    pub fn nilfs_bmap_save(: *const nilfs_bmap, : *mut nilfs_bmap_store);
}
extern "C" {
    pub fn nilfs_bmap_restore(: *mut nilfs_bmap, : *const nilfs_bmap_store);
}
extern "C" {
    pub fn nilfs_bmap_lookup_at_level(_arg: bmap, _arg: key, _arg: 1, _arg: ptr) -> return;
}
//
// Internal use only
//
extern "C" {
    pub fn nilfs_dat_prepare_alloc(_arg: dat, _arg: &req->bpr_req) -> return;
}
// ignore target ptr
extern "C" {
    pub fn nilfs_bmap_find_target_seq(: *const nilfs_bmap, _arg: __u64) -> __u64;
}
extern "C" {
    pub fn nilfs_bmap_find_target_in_group(: *const nilfs_bmap) -> __u64;
}
// Assume that bmap semaphore is locked.
pub const NILFS_BMAP_LARGE: c_uint = 0x1;

