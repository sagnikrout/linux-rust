//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_bmap_btree.h
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
// Copyright (c) 2000,2002-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Maximum number of bmap btree levels.
//

//
// Prototypes for xfs_bmap.c to call.
//
extern "C" {
    pub fn xfs_bmbt_disk_set_all(r: *mut xfs_bmbt_rec, s: *mut xfs_bmbt_irec);
}
extern "C" {
    pub fn xfs_bmbt_disk_get_blockcount(r: *const xfs_bmbt_rec) -> xfs_filblks_t;
}
extern "C" {
    pub fn xfs_bmbt_disk_get_startoff(r: *const xfs_bmbt_rec) -> xfs_fileoff_t;
}
extern "C" {
    pub fn xfs_bmbt_get_maxrecs(: *mut xfs_btree_cur, level: c_int) -> c_int;
}
extern "C" {
    pub fn xfs_bmdr_maxrecs(blocklen: c_int, leaf: c_int) -> c_int;
}
extern "C" {
    pub fn xfs_bmbt_maxlevels_ondisk() -> c_uint;
}
extern "C" {
    pub fn xfs_bmbt_init_cur_cache() -> int __init;
}
extern "C" {
    pub fn xfs_bmbt_destroy_cur_cache();
}
//
// Btree block header size depends on a superblock flag.
//
// Addresses of key, pointers, and records within an incore bmbt block.
// Addresses of key, pointers, and records within an ondisk bmbt block.
//
// Address of pointers within the incore btree root.
//
// These are to be used when we know the size of the block and
// we don't have a cursor.
//
extern "C" {
    pub fn xfs_bmbt_ptr_addr(_arg: mp, _arg: bb, _arg: i, _arg: xfs_bmbt_maxrecs(mp, _arg: sz, _arg: false)) -> return;
}
//
// Compute the space required for the incore btree root containing the given
// number of records.
//
// Compute the space required for the incore btree root given the ondisk
// btree root block.
//
extern "C" {
    pub fn xfs_bmap_broot_space_calc(_arg: mp, _arg: be16_to_cpu(bb->bb_numrecs)) -> return;
}
// Compute the space required for the ondisk root block.
//
// Compute the space required for the ondisk root block given an incore root
// block.
//
extern "C" {
    pub fn xfs_bmdr_space_calc(_arg: be16_to_cpu(bb->bb_numrecs)) -> return;
}
