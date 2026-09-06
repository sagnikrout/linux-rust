//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/dir.h
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
// dir.h
//
// Function prototypes
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dx_hinfo {
    pub major_hash: u32,
    pub minor_hash: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dir_lookup_result {
    pub leaf: *mut *mut *mut buffer_head dl_leaf_bh; / Unindexed,
// block
    pub in: *mut *mut *mut ocfs2_dir_entry dl_entry; / Target dirent,
// unindexed leaf
    pub indexed: *mut *mut *mut buffer_head dl_dx_root_bh; / Root of,
// tree
    pub /: *mut *mut *mut buffer_head dl_dx_leaf_bh; / Indexed leaf block,
    pub in: *mut *mut *mut ocfs2_dx_entry dl_dx_entry; / Target dx_entry,
// indexed leaf
    pub /: *mut *mut ocfs2_dx_hinfo dl_hinfo; / Name hash results,
    pub in: *mut *mut *mut buffer_head dl_prev_leaf_bh;/ Previous entry,
// dir free space
// list. NULL if
// previous entry is
// dx root block.
}

extern "C" {
    pub fn ocfs2_free_dir_lookup_result(res: *mut ocfs2_dir_lookup_result);
}
extern "C" {
    pub fn ocfs2_empty_dir(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ocfs2_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
}
extern "C" {
    pub fn ocfs2_dir_foreach(inode: *mut inode, ctx: *mut dir_context) -> c_int;
}
extern "C" {
    pub fn ocfs2_dx_dir_truncate(dir: *mut inode, di_bh: *mut buffer_head) -> c_int;
}
