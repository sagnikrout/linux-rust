//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/dfs.h
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
// Copyright (c) 2022 Paulo Alcantara <palcantara@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfs_ref {
    pub path: *mut c_char,
    pub full_path: *mut c_char,
    pub ses: *mut cifs_ses,
    pub tl: dfs_cache_tgt_list,
    pub tit: *mut dfs_cache_tgt_iterator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfs_ref_walk {
    pub mnt_ctx: *mut cifs_mount_ctx,
    pub ref: *mut dfs_ref,
    pub refs: [dfs_ref; MAX_NESTED_LINKS],
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
extern "C" {
    pub fn dfs_mount_share(mnt_ctx: *mut cifs_mount_ctx) -> c_int;
}
extern "C" {
    pub fn dfs_cache_canonical_path(_arg: path, _arg: cifs_sb->local_nls, _arg: cifs_remap(cifs_sb)) -> return;
}
//
// cifs_get_smb_ses() already guarantees an active reference of
// @ses->dfs_root_ses when a new session is created, so we need to put extra
// references of all DFS root sessions that were used across the mount process
// in dfs_mount_share().
//
