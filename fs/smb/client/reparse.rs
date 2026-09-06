//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/reparse.h
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
// Copyright (c) 2024 Paulo Alcantara <pc@manguebit.com>
//

pub const REPARSE_SYM_PATH_MAX: c_int = 4060;
//
// Used only by cifs.ko to ignore reparse points from files when client or
// server doesn't support FSCTL_GET_REPARSE_POINT.
//

extern "C" {
    pub fn MKDEV(0xffffffff: v &, 32: v >>) -> return;
}
extern "C" {
    pub fn make_kuid(_arg: current_user_ns(), _arg: uid) -> return;
}
extern "C" {
    pub fn make_kgid(_arg: current_user_ns(), _arg: gid) -> return;
}
//
// Match a reparse point inode if reparse tag and ctime haven't changed.
//
// Windows Server updates ctime of reparse points when their data have changed.
// The server doesn't allow changing reparse tags from existing reparse points,
// though it's worth checking.
//
// Do not match reparse tags when client or server doesn't support
// FSCTL_GET_REPARSE_POINT.  @fattr->cf_cifstag should contain correct
// reparse tag from query dir response but the client won't be able to
// read the reparse point data anyway.  This spares us a revalidation.
//
extern "C" {
    pub fn le32_to_cpu(_arg: data->posix_fi.DosAttributes) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: data->fi.Attributes) -> return;
}
