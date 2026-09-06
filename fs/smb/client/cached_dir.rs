//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cached_dir.h
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
// Functions to handle the cached directory entries
//
// Copyright (c) 2022, Ronnie Sahlberg <lsahlber@redhat.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cached_dirent {
    pub entry: list_head,
    pub name: *mut c_char,
    pub namelen: c_int,
    pub pos: loff_t,
    pub fattr: cifs_fattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cached_dirents {
    pub is_valid:1: bool,
    pub is_failed:1: bool,
    pub /*: *mut *mut file file;,
// Used to associate the cache with a single
// open file instance.
//
    pub de_mutex: mutex,
    pub /: *mut *mut loff_t pos; / Expected ctx->pos,
    pub entries: list_head,
// accounting for cached entries in this directory
    pub entries_count: c_ulong,
    pub bytes_used: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cached_fid {
    pub entry: list_head,
    pub cfids: *mut cached_fids,
    pub path: *const c_char,
    pub has_lease: bool,
    pub is_open: bool,
    pub on_list: bool,
    pub file_all_info_is_valid: bool,
    pub /: *mut *mut unsigned long time; / jiffies of when lease was taken,
    pub /: *mut *mut unsigned long last_access_time; / jiffies of when last accessed,
    pub refcount: kref,
    pub fid: cifs_fid,
    pub tcon: *mut cifs_tcon,
    pub dentry: *mut dentry,
    pub put_work: work_struct,
    pub close_work: work_struct,
    pub dirents: cached_dirents,
// Must be last as it ends in a flexible-array member.
    pub file_all_info: smb2_file_all_info,
}

// default MAX_CACHED_FIDS is 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cached_fids {
// Must be held when:
// - accessing the cfids->entries list
// - accessing the cfids->dying list
//
    pub cfid_list_lock: spinlock_t,
    pub num_entries: c_int,
    pub entries: list_head,
    pub dying: list_head,
    pub laundromat_work: delayed_work,
// aggregate accounting for all cached dirents under this tcon
    pub total_dirents_entries: atomic_long_t,
    pub total_dirents_bytes: core::sync::atomic::AtomicI64,
}

// Module-wide directory cache accounting (defined in cifsfs.c)
extern "C" {
    pub fn free_cached_dirs(cfids: *mut cached_fids);
}
extern "C" {
    pub fn close_cached_dir(cfid: *mut cached_fid);
}
extern "C" {
    pub fn close_all_cached_dirs(cifs_sb: *mut cifs_sb_info);
}
extern "C" {
    pub fn invalidate_all_cached_dirs(tcon: *mut cifs_tcon, sync: bool);
}
extern "C" {
    pub fn cached_dir_lease_break(tcon: *mut cifs_tcon, lease_key[16]: __u8) -> bool;
}
