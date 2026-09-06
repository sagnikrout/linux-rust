//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/ia32.h
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
// 32 bit structures for IA32 support.
//

// signal.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucontext_ia32 {
    pub uc_flags: c_uint,
    pub uc_link: c_uint,
    pub uc_stack: compat_stack_t,
    pub uc_mcontext: sigcontext_32,
    pub /: *mut *mut compat_sigset_t uc_sigmask; / mask last for extensibility,
}

// This matches struct stat64 in glibc2.2, hence the absolutely
// insane amounts of padding around dev_t's.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat64 {
    pub st_dev: c_ulonglong,
    pub __pad0: [c_uchar; 4],
pub const STAT64_HAS_BROKEN_ST_INO: c_int = 1;
    pub __st_ino: c_uint,
    pub st_mode: c_uint,
    pub st_nlink: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub st_rdev: c_ulonglong,
    pub __pad3: [c_uchar; 4],
    pub st_size: c_longlong,
    pub st_blksize: c_uint,
    pub /: *mut *mut long long st_blocks;/ Number 512-byte blocks allocated,
    pub st_atime: unsigned,
    pub st_atime_nsec: unsigned,
    pub st_mtime: unsigned,
    pub st_mtime_nsec: unsigned,
    pub st_ctime: unsigned,
    pub st_ctime_nsec: unsigned,
    pub st_ino: c_ulonglong,
    pub __attribute__((packed)): },
    pub __ia32_enabled: extern bool,
    pub __ia32_enabled: return,
    pub false: __ia32_enabled =,

    pub IS_ENABLED(CONFIG_X86_32): return,

    pub ia32_enabled(): bool enabled =,
    pub ia32_emulation=on\n"): pr_notice_once("32-bit emulation disabled. You can reenable with,
    pub enabled: return,
