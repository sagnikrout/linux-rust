//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/xattr.h
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
// xattr.h
//
// Copyright (C) 2004, 2008 Oracle.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_xattr_type {
    OCFS2_XATTR_INDEX_USER = 1,
    OCFS2_XATTR_INDEX_POSIX_ACL_ACCESS,
    OCFS2_XATTR_INDEX_POSIX_ACL_DEFAULT,
    OCFS2_XATTR_INDEX_TRUSTED,
    OCFS2_XATTR_INDEX_SECURITY,
    OCFS2_XATTR_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_security_xattr_info {
    pub enable: c_int,
    pub name: *const c_char,
    pub value: *mut c_void,
    pub value_len: usize,
}

extern "C" {
    pub fn ocfs2_listxattr(: *mut dentry, : *mut c_char, _arg: usize) -> isize;
}
extern "C" {
    pub fn ocfs2_xattr_remove(: *mut inode, : *mut buffer_head) -> c_int;
}
//
// xattrs can live inside an inode, as part of an external xattr block,
// or inside an xattr bucket, which is the leaf of a tree rooted in an
// xattr block.  Some of the xattr calls, especially the value setting
// functions, want to treat each of these locations as equal.  Let's wrap
// them in a structure that we can pass around instead of raw buffer_heads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_xattr_value_buf {
    pub vb_bh: *mut buffer_head,
    pub vb_access: ocfs2_journal_access_func,
    pub vb_xv: *mut ocfs2_xattr_value_root,
}
