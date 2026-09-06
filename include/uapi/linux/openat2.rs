//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/openat2.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// Arguments for how openat2(2) should open the target path. If only @flags and
// @mode are non-zero, then openat2(2) operates very similarly to openat(2).
//
// However, unlike openat(2), unknown or invalid bits in @flags result in
// -EINVAL rather than being silently ignored. @mode must be zero unless one of
// {O_CREAT, O_TMPFILE} are set.
//
// @flags: O_* flags.
// @mode: O_CREAT/O_TMPFILE file mode.
// @resolve: RESOLVE_* flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct open_how {
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
}

//
// how->flags bits exclusive to openat2(2). These live in the upper 32 bits
// of @flags so that they cannot be expressed by open(2) / openat(2), whose
// @flags argument is a C int.
//

// how->resolve flags for openat2(2).
pub const RESOLVE_NO_XDEV: c_uint = 0x01 /* Block mount-point crossings;
pub const RESOLVE_NO_MAGICLINKS: c_uint = 0x02 /* Block traversal through procfs-style;
pub const RESOLVE_NO_SYMLINKS: c_uint = 0x04 /* Block traversal through all symlinks;
pub const RESOLVE_BENEATH: c_uint = 0x08 /* Block "lexical" trickery like;
pub const RESOLVE_IN_ROOT: c_uint = 0x10 /* Make all jumps to "/" and "..";
pub const RESOLVE_CACHED: c_uint = 0x20 /* Only complete if resolution can be;
