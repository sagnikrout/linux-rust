//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/mgmt/share_config.h
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
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_share_config {
    pub name: *mut c_char,
    pub path: *mut c_char,
    pub path_sz: c_uint,
    pub flags: c_uint,
    pub veto_list: list_head,
    pub vfs_path: path,
    pub refcount: core::sync::atomic::AtomicI32,

    pub tree_connections: core::sync::atomic::AtomicI32,

    pub hlist: hlist_node,
    pub create_mask: c_ushort,
    pub directory_mask: c_ushort,
    pub force_create_mode: c_ushort,
    pub force_directory_mode: c_ushort,
    pub force_uid: c_ushort,
    pub force_gid: c_ushort,
}

extern "C" {
    pub fn ksmbd_share_config_del(share: *mut ksmbd_share_config);
}
extern "C" {
    pub fn __ksmbd_share_config_put(share: *mut ksmbd_share_config);
}
extern "C" {
    pub fn create_proc_shares() -> c_int;
}
