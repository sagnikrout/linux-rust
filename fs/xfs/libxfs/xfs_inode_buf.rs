//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_inode_buf.h
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
// Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Inode location information.  Stored in the inode and passed to
// xfs_read_icluster() to get a buffer and dinode for a given inode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_imap {
    pub /: *mut *mut xfs_agblock_t im_agbno; / starting agbno of inode cluster,
    pub /: *mut *mut unsigned short im_boffset; / offset in inode cluster in bytes,
    pub __packed: },
    pub bpp): *mut xfs_agblock_t agbno, struct xfs_buf,
    pub dip): *mut *mut void xfs_dinode_calc_crc(struct xfs_mount mp, struct xfs_dinode,
    pub lsn): xfs_lsn_t,
    pub from): *mut *mut int xfs_inode_from_disk(struct xfs_inode ip, struct xfs_dinode,
    pub dip): *mut xfs_dinode,
    pub flags2): u64,
    pub flags): uint32_t extsize, uint16_t mode, uint16_t,
    pub flags2): u64,
    pub tv.tv_nsec: *mut *mut return xfs_unix_to_bigtime(tv.tv_sec)  NSEC_PER_SEC +,
    pub ts): xfs_timestamp_t,
    pub 3: return version ==,
    pub 2: return version == 1 || version ==,
