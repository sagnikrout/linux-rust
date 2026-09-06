//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_icache.h
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
// Copyright (c) 2000-2006 Silicon Graphics, Inc.
// All Rights Reserved.
//
pub const XFS_SYNC_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_icwalk {
    pub icw_flags: __u32,
    pub icw_uid: kuid_t,
    pub icw_gid: kgid_t,
    pub icw_prid: prid_t,
    pub icw_min_file_size: __u64,
    pub icw_scan_limit: c_long,
}

// Flags that reflect xfs_fs_eofblocks functionality.

//
// Flags for xfs_iget()
//

// don't read from disk or reinit

// Return -EAGAIN immediately if the inode is unavailable.

// recovery needs direct inode allocation capability
extern "C" {
    pub fn xfs_inode_alloc(mp: *mut xfs_mount, ino: xfs_ino_t) -> *mut xfs_inode;
}
extern "C" {
    pub fn xfs_inode_free(ip: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_reclaim_worker(work: *mut work_struct);
}
extern "C" {
    pub fn xfs_reclaim_inodes(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_reclaim_inodes_count(mp: *mut xfs_mount) -> c_long;
}
extern "C" {
    pub fn xfs_reclaim_inodes_nr(mp: *mut xfs_mount, nr_to_scan: c_ulong) -> c_long;
}
extern "C" {
    pub fn xfs_inode_mark_reclaimable(ip: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_blockgc_free_quota(ip: *mut xfs_inode, iwalk_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn xfs_blockgc_free_space(mp: *mut xfs_mount, icm: *mut xfs_icwalk) -> c_int;
}
extern "C" {
    pub fn xfs_blockgc_flush_all(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_inode_set_eofblocks_tag(ip: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_inode_clear_eofblocks_tag(ip: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_inode_set_cowblocks_tag(ip: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_inode_clear_cowblocks_tag(ip: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_blockgc_worker(work: *mut work_struct);
}
extern "C" {
    pub fn xfs_blockgc_stop(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_blockgc_start(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_inodegc_worker(work: *mut work_struct);
}
extern "C" {
    pub fn xfs_inodegc_push(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_inodegc_flush(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_inodegc_stop(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_inodegc_start(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_inodegc_register_shrinker(mp: *mut xfs_mount) -> c_int;
}
