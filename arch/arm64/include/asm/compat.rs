//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/compat.h
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

pub type compat_mode_t = u16;

pub type __compat_uid_t = u16;
pub type __compat_gid_t = u16;

pub type compat_ipc_pid_t = u16;

//
// Architecture specific compatibility types
//

pub type __compat_uid16_t = u16;
pub type __compat_gid16_t = u16;
pub type compat_nlink_t = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_stat {

    pub st_dev: c_short,
    pub __pad1: c_short,

    pub st_dev: compat_dev_t,

    pub st_ino: compat_ino_t,
    pub st_mode: compat_mode_t,
    pub st_nlink: compat_ushort_t,
    pub st_uid: __compat_uid16_t,
    pub st_gid: __compat_gid16_t,

    pub st_rdev: c_short,
    pub __pad2: c_short,

    pub st_rdev: compat_dev_t,

    pub st_size: compat_off_t,
    pub st_blksize: compat_off_t,
    pub st_blocks: compat_off_t,
    pub st_atime: old_time32_t,
    pub st_atime_nsec: compat_ulong_t,
    pub st_mtime: old_time32_t,
    pub st_mtime_nsec: compat_ulong_t,
    pub st_ctime: old_time32_t,
    pub st_ctime_nsec: compat_ulong_t,
    pub __unused4: [compat_ulong_t; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_statfs {
    pub f_type: c_int,
    pub f_bsize: c_int,
    pub f_blocks: c_int,
    pub f_bfree: c_int,
    pub f_bavail: c_int,
    pub f_files: c_int,
    pub f_ffree: c_int,
    pub f_fsid: compat_fsid_t,
    pub /: *mut *mut int f_namelen; / SunOS ignores this field.,
    pub f_frsize: c_int,
    pub f_flags: c_int,
    pub f_spare: [c_int; 4],
}

pub const COMPAT_MINSIGSTKSZ: c_int = 2048;
extern "C" {
    pub fn test_thread_flag(_arg: TIF_32BIT) -> return;
}
extern "C" {
    pub fn test_ti_thread_flag(_arg: thread, _arg: TIF_32BIT) -> return;
}
extern "C" {
    pub fn compat_arm_syscall(regs: *mut pt_regs, scno: c_int) -> c_long;
}

