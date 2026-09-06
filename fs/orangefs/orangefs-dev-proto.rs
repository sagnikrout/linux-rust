//! Automatically rewritten from C Header to Rust Module
//! Source: fs/orangefs/orangefs-dev-proto.h
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
// (C) 2001 Clemson University and The University of Chicago
//
// See COPYING in top-level directory.
//
// types and constants shared between user space and kernel space for
// device interaction using a common protocol
//
// valid orangefs kernel operation types
//
pub const ORANGEFS_VFS_OP_INVALID: c_uint = 0xFF000000;
pub const ORANGEFS_VFS_OP_FILE_IO: c_uint = 0xFF000001;
pub const ORANGEFS_VFS_OP_LOOKUP: c_uint = 0xFF000002;
pub const ORANGEFS_VFS_OP_CREATE: c_uint = 0xFF000003;
pub const ORANGEFS_VFS_OP_GETATTR: c_uint = 0xFF000004;
pub const ORANGEFS_VFS_OP_REMOVE: c_uint = 0xFF000005;
pub const ORANGEFS_VFS_OP_MKDIR: c_uint = 0xFF000006;
pub const ORANGEFS_VFS_OP_READDIR: c_uint = 0xFF000007;
pub const ORANGEFS_VFS_OP_SETATTR: c_uint = 0xFF000008;
pub const ORANGEFS_VFS_OP_SYMLINK: c_uint = 0xFF000009;
pub const ORANGEFS_VFS_OP_RENAME: c_uint = 0xFF00000A;
pub const ORANGEFS_VFS_OP_STATFS: c_uint = 0xFF00000B;
pub const ORANGEFS_VFS_OP_TRUNCATE: c_uint = 0xFF00000C;
pub const ORANGEFS_VFS_OP_RA_FLUSH: c_uint = 0xFF00000D;
pub const ORANGEFS_VFS_OP_FS_MOUNT: c_uint = 0xFF00000E;
pub const ORANGEFS_VFS_OP_FS_UMOUNT: c_uint = 0xFF00000F;
pub const ORANGEFS_VFS_OP_GETXATTR: c_uint = 0xFF000010;
pub const ORANGEFS_VFS_OP_SETXATTR: c_uint = 0xFF000011;
pub const ORANGEFS_VFS_OP_LISTXATTR: c_uint = 0xFF000012;
pub const ORANGEFS_VFS_OP_REMOVEXATTR: c_uint = 0xFF000013;
pub const ORANGEFS_VFS_OP_PARAM: c_uint = 0xFF000014;
pub const ORANGEFS_VFS_OP_PERF_COUNT: c_uint = 0xFF000015;
pub const ORANGEFS_VFS_OP_CANCEL: c_uint = 0xFF00EE00;
pub const ORANGEFS_VFS_OP_FSYNC: c_uint = 0xFF00EE01;
pub const ORANGEFS_VFS_OP_FSKEY: c_uint = 0xFF00EE02;
pub const ORANGEFS_VFS_OP_READDIRPLUS: c_uint = 0xFF00EE03;
pub const ORANGEFS_VFS_OP_FEATURES: c_uint = 0xFF00EE05 /* 2.9.6 */;
// features is a 64-bit unsigned bitmask
pub const ORANGEFS_FEATURE_READAHEAD: c_int = 1;
//
// Misc constants. Please retain them as multiples of 8!
// Otherwise 32-64 bit interactions will be messed up :)
//
pub const ORANGEFS_MAX_DEBUG_STRING_LEN: c_uint = 0x00000800;
pub const ORANGEFS_MAX_DIRENT_COUNT_READDIR: c_int = 512;

