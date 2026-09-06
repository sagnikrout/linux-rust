//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/stat.h
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

pub const STAT_HAVE_NSEC: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_mode: c_ushort,
    pub st_nlink: c_ushort,
    pub st_uid: c_ushort,
    pub st_gid: c_ushort,
    pub st_rdev: c_ulong,
    pub st_size: c_ulong,
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
}

// We don't need to memset the whole thing just to initialize the padding

pub const STAT64_HAS_BROKEN_ST_INO: c_int = 1;
// This matches struct stat64 in glibc2.1, hence the absolutely
// insane amounts of padding around dev_t's.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat64 {
    pub st_dev: c_ulonglong,
    pub __pad0: [c_uchar; 4],
    pub __st_ino: c_ulong,
    pub st_mode: c_uint,
    pub st_nlink: c_uint,
    pub st_uid: c_ulong,
    pub st_gid: c_ulong,
    pub st_rdev: c_ulonglong,
    pub __pad3: [c_uchar; 4],
    pub st_size: c_longlong,
    pub st_blksize: c_ulong,
// Number 512-byte blocks allocated.
    pub st_blocks: c_ulonglong,
    pub st_atime: c_ulong,
    pub st_atime_nsec: c_ulong,
    pub st_mtime: c_ulong,
    pub st_mtime_nsec: c_uint,
    pub st_ctime: c_ulong,
    pub st_ctime_nsec: c_ulong,
    pub st_ino: c_ulonglong,
}

// We don't need to memset the whole thing just to initialize the padding

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: __kernel_ulong_t,
    pub st_ino: __kernel_ulong_t,
    pub st_nlink: __kernel_ulong_t,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_uint,
    pub st_rdev: __kernel_ulong_t,
    pub st_size: __kernel_long_t,
    pub st_blksize: __kernel_long_t,
    pub /: *mut *mut __kernel_long_t st_blocks; / Number 512-byte blocks allocated.,
    pub st_atime: __kernel_ulong_t,
    pub st_atime_nsec: __kernel_ulong_t,
    pub st_mtime: __kernel_ulong_t,
    pub st_mtime_nsec: __kernel_ulong_t,
    pub st_ctime: __kernel_ulong_t,
    pub st_ctime_nsec: __kernel_ulong_t,
    pub __unused: [__kernel_long_t; 3],
}

// We don't need to memset the whole thing just to initialize the padding

// for 32bit emulation and 32 bit kernels
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

    pub st_size: c_uint,
    pub st_atime: c_uint,
    pub st_mtime: c_uint,
    pub st_ctime: c_uint,

}
