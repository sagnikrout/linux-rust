//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/statfs.h
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
// S390 version
//
// Derived from "include/asm-i386/statfs.h"
//
// We can't use <asm-generic/statfs.h> because in 64-bit mode
// we mix ints of different sizes in our struct statfs.
//

pub type fsid_t = __kernel_fsid_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct statfs {
    pub f_type: c_uint,
    pub f_bsize: c_uint,
    pub f_blocks: c_ulong,
    pub f_bfree: c_ulong,
    pub f_bavail: c_ulong,
    pub f_files: c_ulong,
    pub f_ffree: c_ulong,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: c_uint,
    pub f_frsize: c_uint,
    pub f_flags: c_uint,
    pub f_spare: [c_uint; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct statfs64 {
    pub f_type: c_uint,
    pub f_bsize: c_uint,
    pub f_blocks: c_ulonglong,
    pub f_bfree: c_ulonglong,
    pub f_bavail: c_ulonglong,
    pub f_files: c_ulonglong,
    pub f_ffree: c_ulonglong,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: c_uint,
    pub f_frsize: c_uint,
    pub f_flags: c_uint,
    pub f_spare: [c_uint; 5],
}
