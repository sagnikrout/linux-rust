//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/filecheck.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// filecheck.h
//
// Online file check.
//
// Copyright (C) 2016 SuSE.  All rights reserved.
//

// File check errno

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_filecheck {
    pub /: *mut *mut list_head fc_head; / File check entry list head,
    pub fc_lock: spinlock_t,
    pub /: *mut *mut unsigned int fc_max; / Maximum number of entry in list,
    pub /: *mut *mut unsigned int fc_size; / Current entry count in list,
    pub /: *mut *mut unsigned int fc_done; / Finished entry count in list,
}

pub const OCFS2_FILECHECK_MAXSIZE: c_int = 100;
pub const OCFS2_FILECHECK_MINSIZE: c_int = 10;
// File check operation type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_filecheck_sysfs_entry {
    pub fs_kobj: kobject,
    pub fs_kobj_unregister: completion,
    pub fs_fcheck: *mut ocfs2_filecheck,
}

extern "C" {
    pub fn ocfs2_filecheck_create_sysfs(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_filecheck_remove_sysfs(osb: *mut ocfs2_super);
}
