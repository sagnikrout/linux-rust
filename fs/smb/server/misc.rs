//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/misc.h
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

extern "C" {
    pub fn match_pattern(str: *const c_char, len: usize, pattern: *const c_char) -> c_int;
}
extern "C" {
    pub fn ksmbd_validate_filename(filename: *mut c_char) -> c_int;
}
extern "C" {
    pub fn parse_stream_name(filename: *mut c_char, stream_name: *mut c_char, s_type: *mut c_int) -> c_int;
}
extern "C" {
    pub fn get_nlink(st: *mut kstat) -> c_int;
}
extern "C" {
    pub fn ksmbd_conv_path_to_unix(path: *mut c_char);
}
extern "C" {
    pub fn ksmbd_strip_last_slash(path: *mut c_char);
}
extern "C" {
    pub fn ksmbd_conv_path_to_windows(path: *mut c_char);
}
pub const KSMBD_DIR_INFO_ALIGNMENT: c_int = 8;

extern "C" {
    pub fn ksmbd_NTtimeToUnix(ntutc: __le64) -> timespec64;
}
extern "C" {
    pub fn ksmbd_UnixTimeToNT(t: timespec64) -> u64;
}
extern "C" {
    pub fn ksmbd_systime() -> c_longlong;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_const_name {
    pub const_value: c_uint,
    pub name: *const c_char,
}

extern "C" {
    pub fn ksmbd_proc_init() -> c_int;
}
extern "C" {
    pub fn ksmbd_proc_cleanup();
}
extern "C" {
    pub fn ksmbd_proc_reset();
}

