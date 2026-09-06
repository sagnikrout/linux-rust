//! Automatically rewritten from C Header to Rust Module
//! Source: fs/debugfs/internal.h
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


// SPDX-License-Identifier: GPL-2.0
//
// internal.h - declarations internal to debugfs
//
// Copyright (C) 2016 Nicolai Stange <nicstange@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_inode_info {
    pub vfs_inode: inode,
    pub raw: *const c_void,
    pub real_fops: *const file_operations,
    pub short_fops: *const debugfs_short_fops,
    pub automount: debugfs_automount_t,
}

extern "C" {
    pub fn container_of(_arg: inode, debugfs_inode_info: struct, _arg: vfs_inode) -> return;
}
// declared over in file.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_fsdata {
    pub real_fops: *const file_operations,
    pub short_fops: *const debugfs_short_fops,
    pub active_users: refcount_t,
    pub active_users_drained: completion,
// protect cancellations
    pub cancellations_mtx: mutex,
    pub cancellations: list_head,
    pub methods: c_uint,
}
