//! Automatically rewritten from C Header to Rust Module
//! Source: fs/9p/v9fs_vfs.h
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
// V9FS VFS extensions.
//
// Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
// Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
//
// plan9 semantics are that created files are implicitly opened.
// But linux semantics are that you call create, then open.
// the plan9 approach is superior as it provides an atomic
// open.
// we track the create fid here. When the file is opened, if fidopen is
// non-zero, we use the fid and can skip some steps.
// there may be a better way to do this, but I don't know it.
// one BAD way is to clunk the fid on create, then open it again:
// you lose the atomicity of file open
//
// special case:
// unlink calls remove, which is an implicit clunk. So we have to track
// that kind of thing so that we don't try to clunk a dead fid.
//

// flags for v9fs_stat2inode() & v9fs_stat2inode_dotl()
pub const V9FS_STAT2INODE_KEEP_ISIZE: c_int = 1;
//
// struct v9fs_dentry - v9fs specific dentry data
// @head: List of fid associated with this dentry
// @expire_time: Lookup cache expiration time for negative dentries
// @rcu: used by kfree_rcu to schedule clean up job
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v9fs_dentry {
    pub head: hlist_head,
    pub expire_time: u64,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn v9fs_ndentry_refresh_timeout(dentry: *mut dentry);
}
extern "C" {
    pub fn v9fs_dentry_fid_remove(dentry: *mut dentry);
}
extern "C" {
    pub fn v9fs_free_inode(inode: *mut inode);
}
extern "C" {
    pub fn v9fs_set_netfs_context(inode: *mut inode);
}
extern "C" {
    pub fn v9fs_evict_inode(inode: *mut inode);
}

extern "C" {
    pub fn v9fs_dir_release(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn v9fs_file_open(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn v9fs_uflags2omode(uflags: c_int, extended: c_int) -> c_int;
}
extern "C" {
    pub fn v9fs_blank_wstat(wstat: *mut p9_wstat);
}
extern "C" {
    pub fn v9fs_refresh_inode(fid: *mut p9_fid, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn v9fs_refresh_inode_dotl(fid: *mut p9_fid, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn v9fs_open_to_dotl_flags(flags: c_int) -> c_int;
}
