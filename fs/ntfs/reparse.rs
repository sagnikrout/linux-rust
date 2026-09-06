//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/reparse.h
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
// Copyright (c) 2008-2021 Jean-Pierre Andre
// Copyright (c) 2025 LG Electronics Co., Ltd.
//
extern "C" {
    pub fn ntfs_parse_reparse(ni: *mut ntfs_inode, mode: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn ntfs_reparse_tag_dt_types(vol: *mut ntfs_volume, mref: c_ulong) -> c_uint;
}
extern "C" {
    pub fn ntfs_reparse_set_wsl_not_symlink(ni: *mut ntfs_inode, mode: mode_t) -> c_int;
}
extern "C" {
    pub fn ntfs_delete_reparse_index(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_remove_ntfs_reparse_data(ni: *mut ntfs_inode) -> c_int;
}
