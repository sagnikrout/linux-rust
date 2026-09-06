//! Automatically rewritten from C Header to Rust Module
//! Source: fs/autofs/autofs_i.h
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
// Copyright 1997-1998 Transmeta Corporation - All Rights Reserved
// Copyright 2005-2006 Ian Kent <raven@themaw.net>
//
// Internal header file for autofs

// This is the range of ioctl() numbers we claim as ours

pub const AUTOFS_IOC_COUNT: c_int = 32;

//
// Unified info structure.  This is pointed to by both the dentry and
// inode structures.  Each file in the filesystem has an instance of this
// structure.  It holds a reference to the dentry, so dentries are never
// flushed while the file exists.  All name lookups are dealt with at the
// dentry level, although the filesystem can interfere in the validation
// process.  Readdir is implemented by traversing the dentry lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_info {
    pub dentry: *mut dentry,
    pub flags: c_int,
    pub expire_complete: completion,
    pub active: list_head,
    pub expiring: list_head,
    pub sbi: *mut autofs_sb_info,
    pub exp_timeout: c_ulong,
    pub last_used: c_ulong,
    pub count: c_int,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub rcu: rcu_head,
}

// for expiry, so RCU_walk is
// not permitted.  If it progresses to
// actual expiry attempt, the flag is
// not cleared when EXPIRING is set -
// in that case it gets cleared only
// when it comes to clearing EXPIRING.
//

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_wait_queue {
    pub queue: wait_queue_head_t,
    pub next: *mut autofs_wait_queue,
    pub wait_queue_token: autofs_wqt_t,
// We use the following to see what we are waiting for
    pub name: qstr,
    pub offset: u32,
    pub dev: u32,
    pub ino: u64,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub pid: pid_t,
    pub tgid: pid_t,
// This is for status reporting upon return
    pub status: c_int,
    pub wait_ctr: c_uint,
}

pub const AUTOFS_SBI_MAGIC: c_uint = 0x6d4a556d;
pub const AUTOFS_SBI_CATATONIC: c_uint = 0x0001;
pub const AUTOFS_SBI_STRICTEXPIRE: c_uint = 0x0002;
pub const AUTOFS_SBI_IGNORE: c_uint = 0x0004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_sb_info {
    pub magic: u32,
    pub pipefd: c_int,
    pub pipe: *mut file,
    pub oz_pgrp: *mut pid,
    pub mnt_ns_id: u64,
    pub version: c_int,
    pub sub_version: c_int,
    pub min_proto: c_int,
    pub max_proto: c_int,
    pub flags: c_uint,
    pub exp_timeout: c_ulong,
    pub type: c_uint,
    pub sb: *mut super_block,
    pub wq_mutex: mutex,
    pub pipe_mutex: mutex,
    pub fs_lock: spinlock_t,
    pub /: *mut *mut *mut autofs_wait_queue queues; / Wait queue pointer,
    pub lookup_lock: spinlock_t,
    pub active_list: list_head,
    pub expiring_list: list_head,
    pub rcu: rcu_head,
}

// autofs_oz_mode(): do we see the man behind the curtain?  (The
// processes which do manipulations for us in user space sees the raw
// filesystem without "magic".)
//
extern "C" {
    pub fn autofs_free_ino(: *mut autofs_info);
}
// Expiration
extern "C" {
    pub fn is_autofs_dentry(: *mut dentry) -> c_int;
}
extern "C" {
    pub fn autofs_expire_wait(path: *const path, rcu_walk: c_int) -> c_int;
}
// Device node initialization
extern "C" {
    pub fn autofs_dev_ioctl_init() -> c_int;
}
extern "C" {
    pub fn autofs_dev_ioctl_exit();
}
// Operations structures
// VFS automount flags management functions
// Initializing function
extern "C" {
    pub fn autofs_init_fs_context(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn autofs_clean_ino(: *mut autofs_info);
}
// We want a packet pipe
// We don't expect -EAGAIN
// Queue management functions
extern "C" {
    pub fn autofs_wait_release(: *mut autofs_sb_info, _arg: autofs_wqt_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn autofs_catatonic_mode(: *mut autofs_sb_info);
}
extern "C" {
    pub fn new_encode_dev(_arg: sbi->sb->s_dev) -> return;
}
extern "C" {
    pub fn autofs_kill_sb(: *mut super_block);
}
