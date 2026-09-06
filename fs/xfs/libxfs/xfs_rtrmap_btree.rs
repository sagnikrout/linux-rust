//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_rtrmap_btree.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// rmaps only exist on crc enabled filesystems

extern "C" {
    pub fn xfs_rtrmapbt_compute_maxlevels(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_rtrmapbt_droot_maxrecs(blocklen: c_uint, leaf: bool) -> c_uint;
}
//
// Addresses of records, keys, and pointers within an incore rtrmapbt block.
//
// (note that some of these may appear unused, but they are used in userspace)
//
extern "C" {
    pub fn xfs_rtrmapbt_maxlevels_ondisk() -> c_uint;
}
extern "C" {
    pub fn xfs_rtrmapbt_init_cur_cache() -> int __init;
}
extern "C" {
    pub fn xfs_rtrmapbt_destroy_cur_cache();
}
extern "C" {
    pub fn xfs_rtrmapbt_calc_reserves(mp: *mut xfs_mount) -> xfs_filblks_t;
}
// Addresses of key, pointers, and records within an ondisk rtrmapbt block.
//
// Address of pointers within the incore btree root.
//
// These are to be used when we know the size of the block and
// we don't have a cursor.
//
// Compute the space required for the incore btree root containing the given
// number of records.
//
// Compute the space required for the incore btree root given the ondisk
// btree root block.
//
// Compute the space required for the ondisk root block.
//
// Compute the space required for the ondisk root block given an incore root
// block.
//
extern "C" {
    pub fn xfs_iformat_rtrmap(ip: *mut xfs_inode, dip: *mut xfs_dinode) -> c_int;
}
extern "C" {
    pub fn xfs_iflush_rtrmap(ip: *mut xfs_inode, dip: *mut xfs_dinode);
}
extern "C" {
    pub fn xfs_rtrmap_highest_rgbno(rtg: *mut xfs_rtgroup) -> xfs_rgblock_t;
}
