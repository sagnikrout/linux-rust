//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/inode.h
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
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//

extern "C" {
    pub fn gfs2_release_folio(folio: *mut folio, gfp_mask: gfp_t) -> bool;
}
extern "C" {
    pub fn gfs2_set_aops(inode: *mut inode);
}
extern "C" {
    pub fn S_ISDIR(_arg: ip->i_inode.i_mode) -> return;
}
extern "C" {
    pub fn gfs2_setup_inode(inode: *mut inode);
}
extern "C" {
    pub fn gfs2_dinode_dealloc(ip: *mut gfs2_inode) -> c_int;
}
extern "C" {
    pub fn gfs2_dinode_out(ip: *const gfs2_inode, buf: *mut c_void);
}
extern "C" {
    pub fn gfs2_open_common(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn gfs2_seek_data(file: *mut file, offset: loff_t) -> loff_t;
}
extern "C" {
    pub fn gfs2_seek_hole(file: *mut file, offset: loff_t) -> loff_t;
}
extern "C" {
    pub fn gfs2_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn gfs2_set_inode_flags(inode: *mut inode);
}

