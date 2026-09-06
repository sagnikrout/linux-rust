//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/vfs.h
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
// Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//

//
// Enumeration for stream type.
//
// CreateOptions

pub const CREATE_OPTION_READONLY: c_uint = 0x10000000;
// system. NB not sent over wire
pub const CREATE_OPTION_SPECIAL: c_uint = 0x20000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_dir_info {
    pub name: *const c_char,
    pub wptr: *mut c_char,
    pub rptr: *mut c_char,
    pub name_len: c_int,
    pub out_buf_len: c_int,
    pub num_scan: c_int,
    pub num_entry: c_int,
    pub data_count: c_int,
    pub last_entry_offset: c_int,
    pub hide_dot_file: bool,
    pub flags: c_int,
    pub last_entry_off_align: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_readdir_data {
    pub ctx: dir_context,
    pub private: *mut c_void,
    pub dirent: *mut c_char,
}

// ksmbd kstat wrapper to get valid create time when reading dir entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_kstat {
    pub kstat: *mut kstat,
    pub create_time: c_ulonglong,
    pub file_attributes: __le32,
    pub /: *mut *mut bool has_ads_stream; / AAPL READDIR_ATTR V2 xattr-presence flag,
}

extern "C" {
    pub fn ksmbd_vfs_lock_parent(parent: *mut dentry, child: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_create(work: *mut ksmbd_work, name: *const c_char, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_mkdir(work: *mut ksmbd_work, name: *const c_char, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_fsync(work: *mut ksmbd_work, fid: u64, p_id: u64) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_remove_file(work: *mut ksmbd_work, path: *const path) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_getattr(path: *const path, stat: *mut kstat) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_listxattr(dentry: *mut dentry, list: *mut c_char) -> isize;
}
extern "C" {
    pub fn ksmbd_vfs_kern_path_end_removing(path: *const path);
}
extern "C" {
    pub fn ksmbd_vfs_empty_dir(fp: *mut ksmbd_file) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_set_fadvise(filp: *mut file, option: __le32);
}
extern "C" {
    pub fn ksmbd_vfs_zero_holes(fp: *mut ksmbd_file) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_unlink(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_posix_lock_wait(flock: *mut file_lock);
}
extern "C" {
    pub fn ksmbd_vfs_posix_lock_unblock(flock: *mut file_lock);
}
extern "C" {
    pub fn ksmbd_vfs_remove_sd_xattrs(idmap: *mut mnt_idmap, path: *const path) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_update_compressed_fattr(dentry: *mut dentry, fattr: *mut __le32);
}
extern "C" {
    pub fn ksmbd_vfs_get_compression(fp: *mut ksmbd_file, fmt: *mut u16) -> c_int;
}
extern "C" {
    pub fn ksmbd_vfs_set_compression(work: *mut ksmbd_work, fp: *mut ksmbd_file, fmt: u16) -> c_int;
}
