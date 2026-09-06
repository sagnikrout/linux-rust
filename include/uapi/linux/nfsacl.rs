//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfsacl.h
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
// File: linux/nfsacl.h
//
// (C) 2003 Andreas Gruenbacher <agruen@suse.de>
//
pub const NFS_ACL_PROGRAM: c_int = 100227;
pub const ACLPROC2_NULL: c_int = 0;
pub const ACLPROC2_GETACL: c_int = 1;
pub const ACLPROC2_SETACL: c_int = 2;
pub const ACLPROC2_GETATTR: c_int = 3;
pub const ACLPROC2_ACCESS: c_int = 4;
pub const ACLPROC3_NULL: c_int = 0;
pub const ACLPROC3_GETACL: c_int = 1;
pub const ACLPROC3_SETACL: c_int = 2;
// Flags for the getacl/setacl mode
pub const NFS_ACL: c_uint = 0x0001;
pub const NFS_ACLCNT: c_uint = 0x0002;
pub const NFS_DFACL: c_uint = 0x0004;
pub const NFS_DFACLCNT: c_uint = 0x0008;
pub const NFS_ACL_MASK: c_uint = 0x000f;
// Flag for Default ACL entries
pub const NFS_ACL_DEFAULT: c_uint = 0x1000;
