//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfsd/debug.h
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
// linux/include/linux/nfsd/debug.h
//
// Debugging-related stuff for nfsd
//
// Copyright (C) 1995 Olaf Kirch <okir@monad.swb.de>
//

//
// knfsd debug flags
//
pub const NFSDDBG_SOCK: c_uint = 0x0001;
pub const NFSDDBG_FH: c_uint = 0x0002;
pub const NFSDDBG_EXPORT: c_uint = 0x0004;
pub const NFSDDBG_SVC: c_uint = 0x0008;
pub const NFSDDBG_PROC: c_uint = 0x0010;
pub const NFSDDBG_FILEOP: c_uint = 0x0020;
pub const NFSDDBG_AUTH: c_uint = 0x0040;
pub const NFSDDBG_REPCACHE: c_uint = 0x0080;
pub const NFSDDBG_XDR: c_uint = 0x0100;
pub const NFSDDBG_LOCKD: c_uint = 0x0200;
pub const NFSDDBG_PNFS: c_uint = 0x0400;
pub const NFSDDBG_ALL: c_uint = 0x7FFF;
pub const NFSDDBG_NOCHANGE: c_uint = 0xFFFF;
