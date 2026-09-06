//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/statfs.h
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
// Most 64-bit platforms use 'long', while most 32-bit platforms use '__u32'.
// Yes, they differ in signedness as well as size.
// Special cases can override it for themselves -- except for S390x, which
// is just a little too special for us. And MIPS, which I'm not touching
// with a 10' pole.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct statfs {
    pub f_type: __statfs_word,
    pub f_bsize: __statfs_word,
    pub f_blocks: __statfs_word,
    pub f_bfree: __statfs_word,
    pub f_bavail: __statfs_word,
    pub f_files: __statfs_word,
    pub f_ffree: __statfs_word,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __statfs_word,
    pub f_frsize: __statfs_word,
    pub f_flags: __statfs_word,
    pub f_spare: [__statfs_word; 4],
}

//
// ARM needs to avoid the 32-bit padding at the end, for consistency
// between EABI and OABI
//

// Macro flag: #define ARCH_PACK_STATFS64

#[repr(C)]
#[derive(Copy, Clone)]
pub struct statfs64 {
    pub f_type: __statfs_word,
    pub f_bsize: __statfs_word,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __statfs_word,
    pub f_frsize: __statfs_word,
    pub f_flags: __statfs_word,
    pub f_spare: [__statfs_word; 4],
    pub ARCH_PACK_STATFS64: },
//
// IA64 and x86_64 need to avoid the 32-bit padding at the end,
// to be compatible with the i386 ABI
//

// Macro flag: #define ARCH_PACK_COMPAT_STATFS64

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_statfs64 {
    pub f_type: __u32,
    pub f_bsize: __u32,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __u32,
    pub f_frsize: __u32,
    pub f_flags: __u32,
    pub f_spare: [__u32; 4],
    pub ARCH_PACK_COMPAT_STATFS64: },
