//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/mgmt/tree_connect.h
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
pub struct ksmbd_tree_connect {
    pub id: c_int,
    pub flags: c_uint,
    pub share_conf: *mut ksmbd_share_config,
    pub user: *mut ksmbd_user,
    pub list: list_head,
    pub maximal_access: c_int,
    pub posix_extensions: bool,
    pub refcount: core::sync::atomic::AtomicI32,
    pub t_state: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_tree_conn_status {
    pub ret: c_uint,
    pub tree_conn: *mut ksmbd_tree_connect,
}

extern "C" {
    pub fn ksmbd_tree_connect_put(tcon: *mut ksmbd_tree_connect);
}
extern "C" {
    pub fn ksmbd_tree_conn_session_logoff(sess: *mut ksmbd_session) -> c_int;
}
