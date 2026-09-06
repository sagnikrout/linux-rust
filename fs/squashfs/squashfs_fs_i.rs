//! Automatically rewritten from C Header to Rust Module
//! Source: fs/squashfs/squashfs_fs_i.h
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

// Macro flag: #define SQUASHFS_FS_I
//
// Squashfs
//
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
// Phillip Lougher <phillip@squashfs.org.uk>
//
// squashfs_fs_i.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_inode_info {
    pub start: u64,
    pub offset: c_int,
    pub xattr: u64,
    pub xattr_size: c_uint,
    pub xattr_count: c_int,
    pub parent: c_int,
    pub fragment_block: u64,
    pub fragment_size: c_int,
    pub fragment_offset: c_int,
    pub block_list_start: u64,
}

extern "C" {
    pub fn container_of(_arg: inode, squashfs_inode_info: struct, _arg: vfs_inode) -> return;
}
