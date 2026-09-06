//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/stat.h
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
// Everybody gets this wrong and has to stick with it for all
// eternity. Hopefully, this version gets used by new architectures
// so they don't fall into the same traps.
//
// stat64 is copied from powerpc64, with explicit padding added.
// stat is the same structure layout on 64-bit, without the 'long long'
// types.
//
// By convention, 64 bit architectures use the stat interface, while
// 32 bit architectures use the stat64 interface. Note that we don't
// provide an __old_kernel_stat here, which new architecture should
// not have to start with.
//

pub const STAT_HAVE_NSEC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub /: *mut *mut unsigned long st_dev; / Device.,
    pub /: *mut *mut unsigned long st_ino; / File serial number.,
    pub /: *mut *mut unsigned int st_mode; / File mode.,
    pub /: *mut *mut unsigned int st_nlink; / Link count.,
    pub /: *mut *mut unsigned int st_uid; / User ID of the file's owner.,
    pub /: *mut *mut unsigned int st_gid; / Group ID of the file's group.,
    pub /: *mut *mut unsigned long st_rdev; / Device number, if device.,
    pub __pad1: c_ulong,
    pub /: *mut *mut long st_size; / Size of file, in bytes.,
    pub /: *mut *mut int st_blksize; / Optimal block size for I/O.,
    pub __pad2: c_int,
    pub /: *mut *mut long st_blocks; / Number 512-byte blocks allocated.,
    pub /: *mut *mut long st_atime; / Time of last access.,
    pub st_atime_nsec: c_ulong,
    pub /: *mut *mut long st_mtime; / Time of last modification.,
    pub st_mtime_nsec: c_ulong,
    pub /: *mut *mut long st_ctime; / Time of last status change.,
    pub st_ctime_nsec: c_ulong,
    pub __unused4: c_uint,
    pub __unused5: c_uint,
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
    pub __pad1: c_ulonglong,
    pub /: *mut *mut long long st_size; / Size of file, in bytes.,
    pub /: *mut *mut int st_blksize; / Optimal block size for I/O.,
    pub __pad2: c_int,
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

