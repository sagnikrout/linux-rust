//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/attrlist.h
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
// Exports for attribute list attribute handling.
//
// Copyright (c) 2004 Anton Altaparmakov
// Copyright (c) 2004 Yura Pakhuchiy
// Copyright (c) 2025 LG Electronics Co., Ltd.
//

extern "C" {
    pub fn ntfs_attrlist_need(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_attrlist_entry_add(ni: *mut ntfs_inode, attr: *mut attr_record) -> c_int;
}
extern "C" {
    pub fn ntfs_attrlist_entry_rm(ctx: *mut ntfs_attr_search_ctx) -> c_int;
}
extern "C" {
    pub fn ntfs_attrlist_update(base_ni: *mut ntfs_inode) -> c_int;
}
