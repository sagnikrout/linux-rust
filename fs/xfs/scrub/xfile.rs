//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/xfile.h
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
// Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfile {
    pub file: *mut file,
}

extern "C" {
    pub fn xfile_create(description: *const c_char, isize: loff_t, xfilep: *mut xfile) -> c_int;
}
extern "C" {
    pub fn xfile_destroy(xf: *mut xfile);
}
extern "C" {
    pub fn xfile_load(xf: *mut xfile, buf: *mut c_void, count: usize, pos: loff_t) -> c_int;
}
extern "C" {
    pub fn xfile_discard(xf: *mut xfile, pos: loff_t, count: u64);
}
extern "C" {
    pub fn xfile_seek_data(xf: *mut xfile, pos: loff_t) -> loff_t;
}

extern "C" {
    pub fn xfile_put_folio(xf: *mut xfile, folio: *mut folio);
}
