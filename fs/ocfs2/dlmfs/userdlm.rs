//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/dlmfs/userdlm.h
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
// userdlm.h
//
// Userspace dlm defines
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//

// user_lock_res->l_flags flags.

// the lvb

// dlm_lock

// downconvert

// destroying this
// lock.

// workqueue

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_lock_res {
    pub l_lock: spinlock_t,
    pub l_flags: c_int,
pub const USER_DLM_LOCK_ID_MAX_LEN: c_int = 32;
    pub l_name: [c_char; USER_DLM_LOCK_ID_MAX_LEN],
    pub l_namelen: c_int,
    pub l_level: c_int,
    pub l_ro_holders: c_uint,
    pub l_ex_holders: c_uint,
    pub l_lksb: ocfs2_dlm_lksb,
    pub l_requested: c_int,
    pub l_blocking: c_int,
    pub l_event: wait_queue_head_t,
    pub l_work: work_struct,
}

extern "C" {
    pub fn user_dlm_destroy_lock(lockres: *mut user_lock_res) -> c_int;
}
extern "C" {
    pub fn user_dlm_read_lvb(inode: *mut inode, val: *mut c_char) -> bool;
}
extern "C" {
    pub fn user_dlm_unregister(conn: *mut ocfs2_cluster_connection);
}
extern "C" {
    pub fn user_dlm_set_locking_protocol();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlmfs_inode_private {
    pub ip_conn: *mut ocfs2_cluster_connection,
    pub /: *mut *mut user_lock_res ip_lockres; / unused for directories.,
    pub ip_parent: *mut inode,
    pub ip_vfs_inode: inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlmfs_filp_private {
    pub fp_lock_level: c_int,
}

pub const DLMFS_MAGIC: c_uint = 0x76a9f425;
