//! Automatically rewritten from C Header to Rust Module
//! Source: fs/bfs/bfs.h
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
// fs/bfs/bfs.h
// Copyright (C) 1999-2018 Tigran Aivazian <aivazian.tigran@gmail.com>
//

// In theory BFS supports up to 512 inodes, numbered from 2 (for /) up to 513 inclusive.
pub const BFS_MAX_LASTI: c_int = 513;
//
// BFS file system in-core superblock info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfs_sb_info {
    pub si_blocks: c_ulong,
    pub si_freeb: c_ulong,
    pub si_freei: c_ulong,
    pub si_lf_eblk: c_ulong,
    pub si_lasti: c_ulong,
    pub BFS_MAX_LASTI+1): DECLARE_BITMAP(si_imap,,
    pub bfs_lock: mutex,
}

//
// BFS file system in-core inode info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfs_inode_info {
    pub /: *mut *mut unsigned long i_dsk_ino; / inode number from the disk, can be 0,
    pub i_sblock: c_ulong,
    pub i_eblock: c_ulong,
    pub i_metadata_bhs: mapping_metadata_bhs,
    pub vfs_inode: inode,
}

extern "C" {
    pub fn container_of(_arg: inode, bfs_inode_info: struct, _arg: vfs_inode) -> return;
}

// inode.c
extern "C" {
    pub fn bfs_dump_imap(: *const c_char, : *mut super_block);
}
// file.c
// dir.c
