//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/stat.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub const STAT_HAVE_NSEC: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __old_kernel_stat {
    pub st_dev: c_ushort,
    pub st_ino: c_ushort,
    pub st_mode: c_ushort,
    pub st_nlink: c_ushort,
    pub st_uid: c_ushort,
    pub st_gid: c_ushort,
    pub st_rdev: c_ushort,
    pub st_size: c_ulong,
    pub st_atime: c_ulong,
    pub st_mtime: c_ulong,
    pub st_ctime: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: c_ulong,
    pub st_ino: __kernel_ino_t,

    pub st_nlink: c_ulong,
    pub st_mode: __kernel_mode_t,

    pub st_mode: __kernel_mode_t,
    pub st_nlink: c_ushort,

    pub st_uid: __kernel_uid32_t,
    pub st_gid: __kernel_gid32_t,
    pub st_rdev: c_ulong,
    pub st_size: c_long,
    pub st_blksize: c_ulong,
    pub st_blocks: c_ulong,
    pub st_atime: c_ulong,
    pub st_atime_nsec: c_ulong,
    pub st_mtime: c_ulong,
    pub st_mtime_nsec: c_ulong,
    pub st_ctime: c_ulong,
    pub st_ctime_nsec: c_ulong,
    pub __unused4: c_ulong,
    pub __unused5: c_ulong,

    pub __unused6: c_ulong,

}

// This matches struct stat64 in glibc2.1. Only used for 32 bit.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat64 {
    pub /: *mut *mut unsigned long long st_dev; / Device.,
    pub /: *mut *mut unsigned long long st_ino; / File serial number.,
    pub /: *mut *mut unsigned int st_mode; / File mode.,
    pub /: *mut *mut unsigned int st_nlink; / Link count.,
    pub /: *mut *mut unsigned int st_uid; / User ID of the file's owner.,
    pub /: *mut *mut unsigned int st_gid; / Group ID of the file's group.,
    pub /: *mut *mut unsigned long long st_rdev; / Device number, if device.,
    pub __pad2: c_ushort,
    pub /: *mut *mut long long st_size; / Size of file, in bytes.,
    pub /: *mut *mut int st_blksize; / Optimal block size for I/O.,
    pub /: *mut *mut long long st_blocks; / Number 512-byte blocks allocated.,
    pub /: *mut *mut int st_atime; / Time of last access.,
    pub st_atime_nsec: c_uint,
    pub /: *mut *mut int st_mtime; / Time of last modification.,
    pub st_mtime_nsec: c_uint,
    pub /: *mut *mut int st_ctime; / Time of last status change.,
    pub st_ctime_nsec: c_uint,
    pub __unused4: c_uint,
    pub __unused5: c_uint,
}
