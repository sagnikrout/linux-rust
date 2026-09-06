//! Automatically rewritten from C Header to Rust Module
//! Source: fs/coda/coda_fs_i.h
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
// coda_fs_i.h
//
// Copyright (C) 1998 Carnegie Mellon University
//

//
// coda fs inode data
// c_lock protects accesses to c_flags, c_mapcount, c_cached_epoch, c_uid and
// c_cached_perm.
// vfs_inode is set only when the inode is created and never changes.
// c_fid is set when the inode is created and should be considered immutable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_inode_info {
    pub /: *mut *mut CodaFid c_fid; / Coda identifier,
    pub /: *mut *mut u_short c_flags; / flags (see below),
    pub /: *mut *mut unsigned int c_mapcount; / nr of times this inode is mapped,
    pub /: *mut *mut unsigned int c_cached_epoch; / epoch for cached permissions,
    pub /: *mut *mut kuid_t c_uid; / fsuid for cached permissions,
    pub /: *mut *mut unsigned int c_cached_perm; / cached access permissions,
    pub c_lock: spinlock_t,
    pub vfs_inode: inode,
}

//
// coda fs file private data
//
pub const CODA_MAGIC: c_uint = 0xC0DAC0DA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_file_info {
    pub /: *mut *mut int cfi_magic; / magic number,
    pub /: *mut *mut *mut file cfi_container; / container file for this cnode,
    pub /: *mut *mut unsigned int cfi_mapcount; / nr of times this file is mapped,
    pub /: *mut *mut bool cfi_access_intent; / is access intent supported,
}

// flags
pub const C_VATTR: c_uint = 0x1   /* Validity of vattr in inode */;
pub const C_FLUSH: c_uint = 0x2   /* used after a flush */;
pub const C_DYING: c_uint = 0x4   /* from venus (which died) */;
pub const C_PURGE: c_uint = 0x8;
extern "C" {
    pub fn coda_replace_fid(: *mut inode, : *mut CodaFid, : *mut CodaFid);
}
