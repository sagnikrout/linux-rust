//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/falloc.h
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
// Space reservation ioctls and argument structure
// are designed to be compatible with the legacy XFS ioctls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct space_resv {
    pub l_type: __s16,
    pub l_whence: __s16,
    pub l_start: __s64,
    pub /: *mut *mut __s64 l_len; / len == 0 means until end of file,
    pub l_sysid: __s32,
    pub l_pid: __u32,
    pub /: *mut *mut __s32 l_pad[4]; / reserved area,
}

//
// Mask of all supported fallocate modes.  Only one can be set at a time.
//
// In addition to the mode bit, the mode argument can also encode flags.
// FALLOC_FL_KEEP_SIZE is the only supported flag so far.
//

// on ia32 l_start is on a 32-bit boundary

#[repr(C)]
#[derive(Copy, Clone)]
pub struct space_resv_32 {
    pub l_type: __s16,
    pub l_whence: __s16,
    pub __attribute__((packed)): __s64 l_start,
// len == 0 means until end of file
    pub __attribute__((packed)): __s64 l_len,
    pub l_sysid: __s32,
    pub l_pid: __u32,
    pub /: *mut *mut __s32 l_pad[4]; / reserve area,
}

