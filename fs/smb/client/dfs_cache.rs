//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/dfs_cache.h
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
// DFS referral cache routines
//
// Copyright (c) 2018-2019 Paulo Alcantara <palcantara@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfs_cache_tgt_list {
    pub tl_numtgts: c_int,
    pub tl_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfs_cache_tgt_iterator {
    pub it_name: *mut c_char,
    pub it_path_consumed: c_int,
    pub it_list: list_head,
}

extern "C" {
    pub fn dfs_cache_init() -> c_int;
}
extern "C" {
    pub fn dfs_cache_destroy();
}
extern "C" {
    pub fn dfs_cache_remount_fs(cifs_sb: *mut cifs_sb_info) -> c_int;
}
extern "C" {
    pub fn dfs_cache_refresh(work: *mut work_struct);
}
extern "C" {
    pub fn list_next_entry(_arg: it, _arg: it_list) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &dfs_cache_ttl) -> return;
}
