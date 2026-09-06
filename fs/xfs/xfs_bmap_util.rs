//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_bmap_util.h
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
// Copyright (c) 2000-2006 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Kernel only BMAP related definitions and functions

extern "C" {
    pub fn xfs_bmap_rtalloc(ap: *mut xfs_bmalloca) -> c_int;
}

//
// Attempts to allocate RT extents when RT is disable indicates corruption and
// should trigger a shutdown.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgetbmap {
    pub /: *mut *mut __s64 bmv_offset; / file offset of segment in blocks,
    pub /: *mut *mut __s64 bmv_block; / starting block (64-bit daddr_t),
    pub /: *mut *mut __s64 bmv_length; / length of segment, blocks,
    pub /: *mut *mut __s32 bmv_oflags; / output flags,
}

// functions in xfs_bmap.c that are only needed by xfs_bmap_util.c
extern "C" {
    pub fn xfs_bmap_adjacent(ap: *mut xfs_bmalloca) -> bool;
}
// preallocation and hole punch interface
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_alloc_file_space_mode {
    XFS_ALLOC_FILE_SPACE_PREALLOC,
    XFS_ALLOC_FILE_SPACE_WRITE_ZEROES,
}

// EOF block manipulation functions
extern "C" {
    pub fn xfs_can_free_eofblocks(ip: *mut xfs_inode) -> bool;
}
extern "C" {
    pub fn xfs_free_eofblocks(ip: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_fsb_to_db(ip: *mut xfs_inode, fsb: xfs_fsblock_t) -> xfs_daddr_t;
}
extern "C" {
    pub fn xfs_bmap_count_leaves(ifp: *mut xfs_ifork, count: *mut xfs_filblks_t) -> xfs_extnum_t;
}
