//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/bfs_fs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// include/linux/bfs_fs.h - BFS data structures on disk.
// Copyright (C) 1999-2018 Tigran Aivazian <aivazian.tigran@gmail.com>
//

pub const BFS_BSIZE_BITS: c_int = 9;

pub const BFS_MAGIC: c_uint = 0x1BADFACE;
pub const BFS_ROOT_INO: c_int = 2;
pub const BFS_INODES_PER_BLOCK: c_int = 8;
// SVR4 vnode type values (bfs_inode->i_vtype)

// BFS inode layout on disk
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfs_inode {
    pub i_ino: __le16,
    pub i_unused: __u16,
    pub i_sblock: __le32,
    pub i_eblock: __le32,
    pub i_eoffset: __le32,
    pub i_vtype: __le32,
    pub i_mode: __le32,
    pub i_uid: __le32,
    pub i_gid: __le32,
    pub i_nlink: __le32,
    pub i_atime: __le32,
    pub i_mtime: __le32,
    pub i_ctime: __le32,
    pub i_padding: [__u32; 4],
}

pub const BFS_NAMELEN: c_int = 14;
pub const BFS_DIRENT_SIZE: c_int = 16;
pub const BFS_DIRS_PER_BLOCK: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfs_dirent {
    pub ino: __le16,
    pub name: [c_char; BFS_NAMELEN],
}

// BFS superblock layout on disk
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfs_super_block {
    pub s_magic: __le32,
    pub s_start: __le32,
    pub s_end: __le32,
    pub s_from: __le32,
    pub s_to: __le32,
    pub s_bfrom: __s32,
    pub s_bto: __s32,
    pub s_fsname: [c_char; 6],
    pub s_volume: [c_char; 6],
    pub s_padding: [__u32; 118],
}

