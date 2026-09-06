//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/uptodate.h
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
// uptodate.h
//
// Cluster uptodate tracking
//
// Copyright (C) 2002, 2004, 2005 Oracle.  All rights reserved.
//
// The caching code relies on locking provided by the user of
// struct ocfs2_caching_info.  These operations connect that up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_caching_operations {
//
// A u64 representing the owning structure.  Usually this
// is the block number (i_blkno or whatnot).  This is used so
// that caching log messages can identify the owning structure.
//
    pub ci): *mut *mut u64 (co_owner)(struct ocfs2_caching_info,
// The superblock is needed during I/O.
    pub ci): *mut *mut *mut super_block (co_get_super)(ocfs2_caching_info,
//
// Lock and unlock the caching data.  These will not sleep, and
// should probably be spinlocks.
//
    pub ci): *mut *mut void (co_cache_lock)(struct ocfs2_caching_info,
    pub ci): *mut *mut void (co_cache_unlock)(struct ocfs2_caching_info,
//
// Lock and unlock for disk I/O.  These will sleep, and should
// be mutexes.
//
    pub ci): *mut *mut void (co_io_lock)(struct ocfs2_caching_info,
    pub ci): *mut *mut void (co_io_unlock)(struct ocfs2_caching_info,
}

extern "C" {
    pub fn init_ocfs2_uptodate_cache() -> int __init;
}
extern "C" {
    pub fn exit_ocfs2_uptodate_cache();
}
extern "C" {
    pub fn ocfs2_metadata_cache_purge(ci: *mut ocfs2_caching_info);
}
extern "C" {
    pub fn ocfs2_metadata_cache_exit(ci: *mut ocfs2_caching_info);
}
extern "C" {
    pub fn ocfs2_metadata_cache_owner(ci: *mut ocfs2_caching_info) -> u64;
}
extern "C" {
    pub fn ocfs2_metadata_cache_io_lock(ci: *mut ocfs2_caching_info);
}
extern "C" {
    pub fn ocfs2_metadata_cache_io_unlock(ci: *mut ocfs2_caching_info);
}
