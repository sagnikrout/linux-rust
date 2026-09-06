//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/dir.h
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
// Defines for directory handling in NTFS Linux kernel driver.
//
// Copyright (c) 2002-2004 Anton Altaparmakov
//

//
// ntfs_name is used to return the file name to the caller of
// ntfs_lookup_inode_by_name() in order for the caller (namei.c::ntfs_lookup())
// to be able to deal with dcache aliasing issues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_name {
    pub mref: u64,
    pub type: u8,
    pub len: u8,
    pub name: [__le16; ],
    pub __packed: },
// The little endian Unicode string $I30 as a global constant.
    pub I30: [extern __le16; 5],
    pub res): *const *const __le16 uname, int uname_len, struct ntfs_name,
    pub ni_mrec): *mut *mut int ntfs_check_empty_dir(struct ntfs_inode ni, struct mft_record,
