//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_export.h
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
// Copyright (c) 2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Common defines for code related to exporting XFS filesystems over NFS.
//
// The NFS fileid goes out on the wire as an array of
// 32bit unsigned ints in host order.  There are 5 possible
// formats.
//
// (1)	fileid_type=0x00
// (no fileid data; handled by the generic code)
//
// (2)	fileid_type=0x01
// inode-num
// generation
//
// (3)	fileid_type=0x02
// inode-num
// generation
// parent-inode-num
// parent-generation
//
// (4)	fileid_type=0x81
// inode-num-lo32
// inode-num-hi32
// generation
//
// (5)	fileid_type=0x82
// inode-num-lo32
// inode-num-hi32
// generation
// parent-inode-num-lo32
// parent-inode-num-hi32
// parent-generation
//
// Note, the NFS filehandle also includes an fsid portion which
// may have an inode number in it.  That number is hardcoded to
// 32bits and there is no way for XFS to intercept it.  In
// practice this means when exporting an XFS filesystem with 64bit
// inodes you should either export the mountpoint (rather than
// a subdirectory) or use the "fsid" export option.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_fid64 {
    pub ino: u64,
    pub gen: u32,
    pub parent_ino: u64,
    pub parent_gen: u32,
    pub __attribute__((packed)): },
// This flag goes on the wire.  Don't play with it.
pub const XFS_FILEID_TYPE_64FLAG: c_uint = 0x80	/* NFS fileid has 64bit inodes */;
    pub gen): *mut *mut *mut inode xfs_nfs_get_inode(super_block sb, u64 ino, u32,
