//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/blkpg.h
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

// The argument structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkpg_ioctl_arg {
    pub op: c_int,
    pub flags: c_int,
    pub datalen: c_int,
    pub data: *mut void __user,
}

// The subfunctions (for the op field)
pub const BLKPG_ADD_PARTITION: c_int = 1;
pub const BLKPG_DEL_PARTITION: c_int = 2;
pub const BLKPG_RESIZE_PARTITION: c_int = 3;
// Sizes of name fields. Unused at present.
pub const BLKPG_DEVNAMELTH: c_int = 64;
pub const BLKPG_VOLNAMELTH: c_int = 64;
// The data structure for ADD_PARTITION and DEL_PARTITION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkpg_partition {
    pub /: *mut *mut long long start; / starting offset in bytes,
    pub /: *mut *mut long long length; / length in bytes,
    pub /: *mut *mut int pno; / partition number,
    pub /: *mut *mut char devname[BLKPG_DEVNAMELTH]; / unused / ignored,
    pub /: *mut *mut char volname[BLKPG_VOLNAMELTH]; / unused / ignore,
}
