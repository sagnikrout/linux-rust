//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/cpfile.h
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
// NILFS checkpoint file.
//
// Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Koji Sato.
//

extern "C" {
    pub fn nilfs_cpfile_create_checkpoint(cpfile: *mut inode, cno: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_cpfile_delete_checkpoints(: *mut inode, _arg: __u64, _arg: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_cpfile_delete_checkpoint(: *mut inode, _arg: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_cpfile_change_cpmode(: *mut inode, _arg: __u64, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nilfs_cpfile_is_snapshot(: *mut inode, _arg: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_cpfile_get_stat(: *mut inode, : *mut nilfs_cpstat) -> c_int;
}
