//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/xfblob.h
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
// Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfblob {
    pub xfile: *mut xfile,
    pub last_offset: loff_t,
}

pub type xfblob_cookie = loff_t;
extern "C" {
    pub fn xfblob_create(descr: *const c_char, blobp: *mut xfblob) -> c_int;
}
extern "C" {
    pub fn xfblob_destroy(blob: *mut xfblob);
}
extern "C" {
    pub fn xfblob_free(blob: *mut xfblob, cookie: xfblob_cookie) -> c_int;
}
extern "C" {
    pub fn xfblob_bytes(blob: *mut xfblob) -> c_ulonglong;
}
extern "C" {
    pub fn xfblob_truncate(blob: *mut xfblob);
}
extern "C" {
    pub fn xfblob_store(_arg: blob, _arg: cookie, _arg: xname->name, _arg: xname->len) -> return;
}
