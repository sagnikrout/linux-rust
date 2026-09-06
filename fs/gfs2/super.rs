//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/super.h
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
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//

// Supported fs format version range

extern "C" {
    pub fn gfs2_lm_unmount(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_jindex_free(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_jdesc_check(jd: *mut gfs2_jdesc) -> c_int;
}
extern "C" {
    pub fn gfs2_make_fs_rw(sdp: *mut gfs2_sbd) -> c_int;
}
extern "C" {
    pub fn gfs2_make_fs_ro(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_online_uevent(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_destroy_threads(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_statfs_init(sdp: *mut gfs2_sbd) -> c_int;
}
extern "C" {
    pub fn update_statfs(sdp: *mut gfs2_sbd, m_bh: *mut buffer_head);
}
extern "C" {
    pub fn gfs2_statfs_sync(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn gfs2_freeze_func(work: *mut work_struct);
}
extern "C" {
    pub fn free_local_statfs_inodes(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn free_sbd(sdp: *mut gfs2_sbd);
}
