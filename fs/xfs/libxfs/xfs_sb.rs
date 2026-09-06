//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_sb.h
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
// Copyright (c) 2000-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
extern "C" {
    pub fn xfs_log_sb(tp: *mut xfs_trans);
}
extern "C" {
    pub fn xfs_sync_sb(mp: *mut xfs_mount, wait: bool) -> c_int;
}
extern "C" {
    pub fn xfs_sync_sb_buf(mp: *mut xfs_mount, update_rtsb: bool) -> c_int;
}
extern "C" {
    pub fn xfs_sb_mount_common(mp: *mut xfs_mount, sbp: *mut xfs_sb);
}
extern "C" {
    pub fn xfs_sb_mount_rextsize(mp: *mut xfs_mount, sbp: *mut xfs_sb);
}
extern "C" {
    pub fn xfs_sb_from_disk(to: *mut xfs_sb, from: *mut xfs_dsb);
}
extern "C" {
    pub fn xfs_sb_to_disk(to: *mut xfs_dsb, from: *mut xfs_sb);
}
extern "C" {
    pub fn xfs_sb_quota_from_disk(sbp: *mut xfs_sb);
}
extern "C" {
    pub fn xfs_sb_good_version(sbp: *mut xfs_sb) -> bool;
}
extern "C" {
    pub fn xfs_sb_version_to_features(sbp: *mut xfs_sb) -> u64;
}
extern "C" {
    pub fn xfs_update_secondary_sbs(mp: *mut xfs_mount) -> c_int;
}

extern "C" {
    pub fn xfs_validate_rt_geometry(sbp: *mut xfs_sb) -> bool;
}
extern "C" {
    pub fn xfs_compute_rextslog(rtextents: xfs_rtbxlen_t) -> u8;
}
extern "C" {
    pub fn xfs_compute_rgblklog(rgextents: xfs_rtxlen_t, rextsize: xfs_rgblock_t) -> c_int;
}
