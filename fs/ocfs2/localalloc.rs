//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/localalloc.h
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
// localalloc.h
//
// Function prototypes
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//
extern "C" {
    pub fn ocfs2_load_local_alloc(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_shutdown_local_alloc(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_la_set_sizes(osb: *mut ocfs2_super, requested_mb: c_int);
}
extern "C" {
    pub fn ocfs2_la_default_mb(osb: *mut ocfs2_super) -> c_uint;
}
extern "C" {
    pub fn ocfs2_la_enable_worker(work: *mut work_struct);
}
