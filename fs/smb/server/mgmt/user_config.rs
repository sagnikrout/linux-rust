//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/mgmt/user_config.h
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
pub struct ksmbd_user {
    pub flags: c_ushort,
    pub uid: c_uint,
    pub gid: c_uint,
    pub name: *mut c_char,
    pub passkey_sz: usize,
    pub passkey: *mut c_char,
    pub ngroups: c_int,
    pub sgid: *mut gid_t,
}

extern "C" {
    pub fn ksmbd_free_user(user: *mut ksmbd_user);
}
extern "C" {
    pub fn ksmbd_anonymous_user(user: *mut ksmbd_user) -> bool;
}
extern "C" {
    pub fn ksmbd_compare_user(u1: *mut ksmbd_user, u2: *mut ksmbd_user) -> bool;
}
