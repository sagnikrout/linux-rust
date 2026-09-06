//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/acl.h
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
// acl.h
//
// Copyright (C) 2004, 2008 Oracle.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_acl_entry {
    pub e_tag: __le16,
    pub e_perm: __le16,
    pub e_id: __le32,
}

extern "C" {
    pub fn ocfs2_acl_chmod(: *mut inode, : *mut buffer_head) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_acl_state {
    pub default_acl: *mut posix_acl,
    pub acl: *mut posix_acl,
    pub mode: umode_t,
}

extern "C" {
    pub fn ocfs2_acl_init_release(state: *mut ocfs2_acl_state);
}
