//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_message.h
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
pub const __XFS_MESSAGE_H: c_int = 1;

extern "C" {
    pub fn assfail(mp: *mut xfs_mount, expr: *mut c_char, f: *mut c_char, l: c_int);
}
extern "C" {
    pub fn asswarn(mp: *mut xfs_mount, expr: *mut c_char, f: *mut c_char, l: c_int);
}
extern "C" {
    pub fn xfs_hex_dump(p: *const c_void, length: c_int);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_experimental_feat {
    XFS_EXPERIMENTAL_SHRINK,
    XFS_EXPERIMENTAL_LARP,

    XFS_EXPERIMENTAL_MAX,
}

extern "C" {
    pub fn xfs_warn_experimental(mp: *mut xfs_mount, f: xfs_experimental_feat);
}
