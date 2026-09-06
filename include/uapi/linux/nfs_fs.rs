//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfs_fs.h
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
// linux/include/linux/nfs_fs.h
//
// Copyright (C) 1992  Rick Sladkey
//
// OS-specific nfs filesystem definitions and declarations
//

// Default timeout values

//
// When flushing a cluster of dirty pages, there can be different
// strategies:
//

// if everything fits in one RPC
//
// NFS debug flags
//
pub const NFSDBG_VFS: c_uint = 0x0001;
pub const NFSDBG_DIRCACHE: c_uint = 0x0002;
pub const NFSDBG_LOOKUPCACHE: c_uint = 0x0004;
pub const NFSDBG_PAGECACHE: c_uint = 0x0008;
pub const NFSDBG_PROC: c_uint = 0x0010;
pub const NFSDBG_XDR: c_uint = 0x0020;
pub const NFSDBG_FILE: c_uint = 0x0040;
pub const NFSDBG_ROOT: c_uint = 0x0080;
pub const NFSDBG_CALLBACK: c_uint = 0x0100;
pub const NFSDBG_CLIENT: c_uint = 0x0200;
pub const NFSDBG_MOUNT: c_uint = 0x0400;
pub const NFSDBG_FSCACHE: c_uint = 0x0800 /* unused */;
pub const NFSDBG_PNFS: c_uint = 0x1000;
pub const NFSDBG_PNFS_LD: c_uint = 0x2000;
pub const NFSDBG_STATE: c_uint = 0x4000;
pub const NFSDBG_XATTRCACHE: c_uint = 0x8000;
pub const NFSDBG_ALL: c_uint = 0xFFFF;
