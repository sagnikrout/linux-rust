//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/stat.h
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
// Derived from "include/asm-i386/stat.h"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad1: c_uint,
    pub st_rdev: c_ulong,
    pub st_size: c_ulong,
    pub st_atime: c_ulong,
    pub st_atime_nsec: c_ulong,
    pub st_mtime: c_ulong,
    pub st_mtime_nsec: c_ulong,
    pub st_ctime: c_ulong,
    pub st_ctime_nsec: c_ulong,
    pub st_blksize: c_ulong,
    pub st_blocks: c_long,
    pub __unused: [c_ulong; 3],
}

pub const STAT_HAVE_NSEC: c_int = 1;
