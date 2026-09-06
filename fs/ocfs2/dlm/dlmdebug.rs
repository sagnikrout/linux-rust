//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/dlm/dlmdebug.h
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
// dlmdebug.h
//
// Copyright (C) 2008 Oracle.  All rights reserved.
//
extern "C" {
    pub fn dlm_print_one_mle(mle: *mut dlm_master_list_entry);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_lockres {
    pub dl_len: c_int,
    pub dl_buf: *mut c_char,
    pub dl_ctxt: *mut dlm_ctxt,
    pub dl_res: *mut dlm_lock_resource,
}

extern "C" {
    pub fn dlm_debug_init(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_create_debugfs_subroot(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_destroy_debugfs_subroot(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_create_debugfs_root();
}
extern "C" {
    pub fn dlm_destroy_debugfs_root();
}

