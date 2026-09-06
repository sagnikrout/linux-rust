//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/iostat.h
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
// linux/fs/nfs/iostat.h
//
// Declarations for NFS client per-mount statistics
//
// Copyright (C) 2005, 2006 Chuck Lever <cel@netapp.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_iostats {
    pub bytes: [c_ulonglong; __NFSIOS_BYTESMAX],
    pub events: [c_ulong; __NFSIOS_COUNTSMAX],
    pub ____cacheline_aligned: },
    pub stat): nfs_inc_server_stats(NFS_SERVER(inode),,
    pub addend): this_cpu_add(server->io_stats->bytes[stat],,
    pub addend): nfs_add_server_stats(NFS_SERVER(inode), stat,,
//
// This specialized allocator has to be a macro for its allocations to be
// accounted separately (to have a separate alloc_tag).
//

