//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/refcounttree.h
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
// refcounttree.h
//
// Copyright (C) 2009 Oracle.  All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_refcount_tree {
    pub rf_node: rb_node,
    pub rf_blkno: u64,
    pub rf_generation: u32,
    pub rf_getcnt: kref,
    pub rf_sem: rw_semaphore,
    pub rf_lockres: ocfs2_lock_res,
    pub rf_removed: c_int,
// the following 4 fields are used by caching_info.
    pub rf_lock: spinlock_t,
    pub rf_ci: ocfs2_caching_info,
    pub rf_io_mutex: mutex,
    pub rf_sb: *mut super_block,
}

extern "C" {
    pub fn ocfs2_purge_refcount_trees(osb: *mut ocfs2_super);
}
//
// Some refcount caller need to do more work after we modify the data b-tree
// during refcount operation(including CoW and add refcount flag), and make the
// transaction complete. So it must give us this structure so that we can do it
// within our transaction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_post_refcount {
    pub /: *mut *mut int credits; / credits it need for journal.,
    pub /: *mut *mut *mut ocfs2_post_refcount_func func; / real function.,
    pub para: *mut c_void,
}

extern "C" {
    pub fn ocfs2_remove_refcount_tree(inode: *mut inode, di_bh: *mut buffer_head) -> c_int;
}
