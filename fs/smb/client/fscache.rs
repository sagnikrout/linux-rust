//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/fscache.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// CIFS filesystem cache interface definitions
//
// Copyright (c) 2010 Novell, Inc.
// Authors(s): Suresh Jayaraman (sjayaraman@suse.de>
//

//
// Coherency data attached to CIFS volume within the cache
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_fscache_volume_coherency_data {
    pub /: *mut *mut __le64 resource_id; / unique server resource id,
    pub vol_create_time: __le64,
    pub vol_serial_number: __le32,
    pub __packed: },
//
// Coherency data attached to CIFS inode within the cache.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_fscache_inode_coherency_data {
    pub last_write_time_sec: __le64,
    pub last_change_time_sec: __le64,
    pub last_write_time_nsec: __le32,
    pub last_change_time_nsec: __le32,
}

//
// fscache.c
//
extern "C" {
    pub fn cifs_fscache_get_super_cookie(tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn cifs_fscache_release_super_cookie(tcon: *mut cifs_tcon);
}
extern "C" {
    pub fn cifs_fscache_get_inode_cookie(inode: *mut inode);
}
extern "C" {
    pub fn cifs_fscache_unuse_inode_cookie(inode: *mut inode, update: bool);
}
extern "C" {
    pub fn cifs_fscache_release_inode_cookie(inode: *mut inode);
}
extern "C" {
    pub fn cifs_fscache_get_super_cookie(tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn cifs_fscache_release_super_cookie(tcon: *mut cifs_tcon);
}
extern "C" {
    pub fn cifs_fscache_get_inode_cookie(inode: *mut inode);
}
extern "C" {
    pub fn cifs_fscache_release_inode_cookie(inode: *mut inode);
}
extern "C" {
    pub fn cifs_fscache_unuse_inode_cookie(inode: *mut inode, update: bool);
}
extern "C" {
    pub fn netfs_i_cookie(_arg: &CIFS_I(inode)->netfs) -> return;
}
extern "C" {
    pub fn fscache_cookie_enabled(_arg: cifs_inode_cookie(inode)) -> return;
}

