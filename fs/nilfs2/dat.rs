//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/dat.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// NILFS disk address translation.
//
// Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Koji Sato.
//

extern "C" {
    pub fn nilfs_dat_translate(: *mut inode, _arg: __u64, : *mut sector_t) -> c_int;
}
extern "C" {
    pub fn nilfs_dat_prepare_alloc(: *mut inode, : *mut nilfs_palloc_req) -> c_int;
}
extern "C" {
    pub fn nilfs_dat_commit_alloc(: *mut inode, : *mut nilfs_palloc_req);
}
extern "C" {
    pub fn nilfs_dat_abort_alloc(: *mut inode, : *mut nilfs_palloc_req);
}
extern "C" {
    pub fn nilfs_dat_prepare_start(: *mut inode, : *mut nilfs_palloc_req) -> c_int;
}
extern "C" {
    pub fn nilfs_dat_prepare_end(: *mut inode, : *mut nilfs_palloc_req) -> c_int;
}
extern "C" {
    pub fn nilfs_dat_commit_end(: *mut inode, : *mut nilfs_palloc_req, _arg: c_int);
}
extern "C" {
    pub fn nilfs_dat_abort_end(: *mut inode, : *mut nilfs_palloc_req);
}
extern "C" {
    pub fn nilfs_dat_mark_dirty(: *mut inode, _arg: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_dat_freev(: *mut inode, : *mut __u64, _arg: usize) -> c_int;
}
extern "C" {
    pub fn nilfs_dat_move(: *mut inode, _arg: __u64, _arg: sector_t) -> c_int;
}
extern "C" {
    pub fn nilfs_dat_get_vinfo(: *mut inode, : *mut c_void, int: unsigned, _arg: usize) -> isize;
}
