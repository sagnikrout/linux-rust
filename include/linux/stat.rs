//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/stat.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kstat {
    pub /: *mut *mut u32 result_mask; / What fields the user got,
    pub mode: umode_t,
    pub nlink: c_uint,
    pub /: *mut *mut uint32_t blksize; / Preferred I/O size,
    pub attributes: u64,
    pub attributes_mask: u64,

    pub ino: u64,
    pub dev: dev_t,
    pub rdev: dev_t,
    pub /: *mut *mut kuid_t uid; / This is logically a vfsuid_t.,
    pub /: *mut *mut kgid_t gid; / This is logically a vfsgid_t.,
    pub size: loff_t,
    pub atime: timespec64,
    pub mtime: timespec64,
    pub ctime: timespec64,
    pub /: *mut *mut timespec64 btime; / File creation time,
    pub blocks: u64,
    pub mnt_id: u64,
    pub change_cookie: u64,
    pub subvol: u64,
    pub dio_mem_align: u32,
    pub dio_offset_align: u32,
    pub dio_read_offset_align: u32,
    pub atomic_write_unit_min: u32,
    pub atomic_write_unit_max: u32,
    pub atomic_write_unit_max_opt: u32,
    pub atomic_write_segments_max: u32,
}

// These definitions are internal to the kernel for now. Mainly used by nfsd.
// mask values
pub const STATX_CHANGE_COOKIE: c_uint = 0x40000000U	/* Want/got stx_change_attr */;
// file attribute values
pub const STATX_ATTR_CHANGE_MONOTONIC: c_uint = 0x8000000000000000ULL /* version monotonically increases */;
