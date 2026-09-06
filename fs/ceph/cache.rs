//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ceph/cache.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Ceph cache definitions.
//
// Copyright (C) 2013 by Adfin Solutions, Inc. All Rights Reserved.
// Written by Milosz Tanski (milosz@adfin.com)
//

extern "C" {
    pub fn ceph_fscache_register_fs(fsc: *mut *mut ceph_fs_client, fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn ceph_fscache_unregister_fs(fsc: *mut *mut ceph_fs_client);
}
extern "C" {
    pub fn ceph_fscache_register_inode_cookie(inode: *mut inode);
}
extern "C" {
    pub fn ceph_fscache_unregister_inode_cookie(ci: *mut *mut ceph_inode_info);
}
extern "C" {
    pub fn ceph_fscache_use_cookie(inode: *mut inode, will_modify: bool);
}
extern "C" {
    pub fn ceph_fscache_unuse_cookie(inode: *mut inode, update: bool);
}
extern "C" {
    pub fn ceph_fscache_update(inode: *mut inode);
}
extern "C" {
    pub fn ceph_fscache_invalidate(inode: *mut inode, dio_write: bool);
}
extern "C" {
    pub fn netfs_i_cookie(_arg: &ci->netfs) -> return;
}
extern "C" {
    pub fn netfs_unpin_writeback(_arg: inode, _arg: wbc) -> return;
}

extern "C" {
    pub fn fscache_cookie_enabled(_arg: ceph_fscache_cookie(ceph_inode(inode))) -> return;
}

