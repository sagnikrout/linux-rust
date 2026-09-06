//! Automatically rewritten from C Header to Rust Module
//! Source: fs/f2fs/acl.h
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
// fs/f2fs/acl.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Portions of this code from linux/fs/ext2/acl.h
//
// Copyright (C) 2001-2003 Andreas Gruenbacher, <agruen@suse.de>
//

pub const F2FS_ACL_VERSION: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_acl_entry {
    pub e_tag: __le16,
    pub e_perm: __le16,
    pub e_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_acl_entry_short {
    pub e_tag: __le16,
    pub e_perm: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_acl_header {
    pub a_version: __le32,
}

