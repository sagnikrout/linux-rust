//! Automatically rewritten from C Header to Rust Module
//! Source: fs/9p/fid.h
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
// V9FS FID Management
//
// Copyright (C) 2005 by Eric Van Hensbergen <ericvh@gmail.com>
//

extern "C" {
    pub fn v9fs_fid_lookup(_arg: dentry->d_parent) -> return;
}
extern "C" {
    pub fn v9fs_fid_add(dentry: *mut dentry, fid: *mut p9_fid);
}
extern "C" {
    pub fn v9fs_open_fid_add(inode: *mut inode, fid: *mut p9_fid);
}
extern "C" {
    pub fn IS_ERR(p9_client_walk(fid: fid) ? fid :, _arg: 0, _arg: NULL, _arg: 1) -> return;
}
//
// v9fs_fid_addmodes - add cache flags to fid mode (for client use only)
// @fid: fid to augment
// @s_flags: session info mount flags
// @s_cache: session info cache flags
// @f_flags: unix open flags
//
// make sure mode reflects flags of underlying mounts
// also qid.version == 0 reflects a synthetic or legacy file system
// NOTE: these are set after open so only reflect 9p client not
// underlying file system on server.
//
