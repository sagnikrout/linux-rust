//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/compat.h
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
// Architecture specific compatibility types
//

pub type compat_mode_t = u16;

pub type __compat_uid_t = u16;
pub type __compat_gid_t = u16;

pub type compat_dev_t = u16;

pub type compat_ipc_pid_t = u16;

pub type compat_nlink_t = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_stat {
    pub st_dev: u32,
    pub st_ino: compat_ino_t,
    pub st_mode: compat_mode_t,
    pub st_nlink: compat_nlink_t,
    pub st_uid: __compat_uid_t,
    pub st_gid: __compat_gid_t,
    pub st_rdev: u32,
    pub st_size: u32,
    pub st_blksize: u32,
    pub st_blocks: u32,
    pub st_atime: u32,
    pub st_atime_nsec: u32,
    pub st_mtime: u32,
    pub st_mtime_nsec: u32,
    pub st_ctime: u32,
    pub st_ctime_nsec: u32,
    pub __unused4: u32,
    pub __unused5: u32,
}

//
// IA32 uses 4 byte alignment for 64 bit quantities, so we need to pack the
// compat flock64 structure.
//
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

extern "C" {
    pub fn in_ia32_syscall(in_x32_syscall(: ) ||) -> return;
}

extern "C" {
    pub fn in_32bit_syscall() -> return;
}

