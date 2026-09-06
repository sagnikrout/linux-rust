//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/mgmt/ksmbd_ida.h
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

//
// 2.2.1.6.7 TID Generation
// The value 0xFFFF MUST NOT be used as a valid TID. All other
// possible values for TID, including zero (0x0000), are valid.
// The value 0xFFFF is used to specify all TIDs or no TID,
// depending upon the context in which it is used.
//
extern "C" {
    pub fn ksmbd_acquire_smb2_tid(ida: *mut ida) -> c_int;
}
//
// 2.2.1.6.8 UID Generation
// The value 0xFFFE was declared reserved in the LAN Manager 1.0
// documentation, so a value of 0xFFFE SHOULD NOT be used as a
// valid UID.<21> All other possible values for a UID, excluding
// zero (0x0000), are valid.
//
extern "C" {
    pub fn ksmbd_acquire_smb2_uid(ida: *mut ida) -> c_int;
}
extern "C" {
    pub fn ksmbd_acquire_async_msg_id(ida: *mut ida) -> c_int;
}
extern "C" {
    pub fn ksmbd_acquire_id(ida: *mut ida) -> c_int;
}
extern "C" {
    pub fn ksmbd_release_id(ida: *mut ida, id: c_int);
}
