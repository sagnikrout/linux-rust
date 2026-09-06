//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/stat.h
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
// Copyright (C) 2012 ARM Ltd.
//

//
// struct stat64 is needed for compat tasks only. Its definition is different
// from the generic struct stat64.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat64 {
    pub st_dev: compat_u64,
    pub __pad0: [c_uchar; 4],
pub const STAT64_HAS_BROKEN_ST_INO: c_int = 1;
    pub __st_ino: compat_ulong_t,
    pub st_mode: compat_uint_t,
    pub st_nlink: compat_uint_t,
    pub st_uid: compat_ulong_t,
    pub st_gid: compat_ulong_t,
    pub st_rdev: compat_u64,
    pub __pad3: [c_uchar; 4],
    pub st_size: compat_s64,
    pub st_blksize: compat_ulong_t,
    pub /: *mut *mut compat_u64 st_blocks; / Number of 512-byte blocks allocated.,
    pub st_atime: compat_ulong_t,
    pub st_atime_nsec: compat_ulong_t,
    pub st_mtime: compat_ulong_t,
    pub st_mtime_nsec: compat_ulong_t,
    pub st_ctime: compat_ulong_t,
    pub st_ctime_nsec: compat_ulong_t,
    pub st_ino: compat_u64,
}

