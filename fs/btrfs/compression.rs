//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/compression.h
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
// Copyright (C) 2008 Oracle.  All rights reserved.
//

//
// We want to make sure that amount of RAM required to uncompress an extent is
// reasonable, so we limit the total size in ram of a compressed extent to
// 128k.  This is a crucial number because it also controls how easily we can
// spread reads across cpus for decompression.
//
// We also want to make sure the amount of IO required to do a random read is
// reasonably small, so we limit the size of a compressed extent to 128k.
//
// Maximum length of compressed data stored on disk

// The max size for a single worker to compress.

// Maximum size of data before compression

pub const BTRFS_ZLIB_DEFAULT_LEVEL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compressed_bio {
// starting offset in the inode for our pages
    pub start: u64,
// Number of bytes in the inode we're working on
    pub len: c_uint,
// The compression algorithm for this bio
    pub compress_type: u8,
// Whether this is a write for writeback.
    pub writeback: bool,
// For reads, this is the bio we are copying the data into.
    pub orig_bbio: *mut btrfs_bio,
// Must be last.
    pub bbio: btrfs_bio,
}

// @range_end must be exclusive.
// @cur must be inside the folio.
extern "C" {
    pub fn btrfs_alloc_compress_wsm(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_free_compress_wsm(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_init_compress() -> int __init;
}
extern "C" {
    pub fn btrfs_exit_compress() -> void __cold;
}
extern "C" {
    pub fn btrfs_compress_level_valid(type: c_uint, level: c_int) -> bool;
}
extern "C" {
    pub fn btrfs_submit_compressed_read(bbio: *mut btrfs_bio);
}
extern "C" {
    pub fn btrfs_compress_str2level(type: c_uint, str: *const c_char, level_ret: *mut c_int) -> c_int;
}
extern "C" {
    pub fn btrfs_free_compr_folio(folio: *mut folio);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct workspace_manager {
    pub idle_ws: list_head,
    pub ws_lock: spinlock_t,
// Number of free workspaces
    pub free_ws: c_int,
// Total number of allocated workspaces
    pub total_ws: core::sync::atomic::AtomicI32,
// Waiters for a free workspace
    pub ws_wait: wait_queue_head_t,
}

extern "C" {
    pub fn btrfs_put_workspace(fs_info: *mut btrfs_fs_info, type: c_int, ws: *mut list_head);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_compress_levels {
// Maximum level supported by the compression algorithm
    pub min_level: c_int,
    pub max_level: c_int,
    pub default_level: c_int,
}

// The heuristic workspaces are managed via the 0th workspace manager

extern "C" {
    pub fn btrfs_compress_type2str(type: btrfs_compression_type) -> *const c_char;
}
extern "C" {
    pub fn btrfs_compress_is_valid_type(str: *const c_char, len: usize) -> bool;
}
extern "C" {
    pub fn btrfs_compress_heuristic(inode: *mut btrfs_inode, start: u64, end: u64) -> c_int;
}
extern "C" {
    pub fn zlib_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int;
}
extern "C" {
    pub fn zlib_decompress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int;
}
extern "C" {
    pub fn zlib_free_workspace(ws: *mut list_head);
}
extern "C" {
    pub fn lzo_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int;
}
extern "C" {
    pub fn lzo_decompress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int;
}
extern "C" {
    pub fn lzo_free_workspace(ws: *mut list_head);
}
extern "C" {
    pub fn zstd_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int;
}
extern "C" {
    pub fn zstd_decompress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int;
}
extern "C" {
    pub fn zstd_alloc_workspace_manager(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn zstd_free_workspace_manager(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn zstd_free_workspace(ws: *mut list_head);
}
extern "C" {
    pub fn zstd_put_workspace(fs_info: *mut btrfs_fs_info, ws: *mut list_head);
}
