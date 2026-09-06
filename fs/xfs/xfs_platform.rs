//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_platform.h
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
// Copyright (c) 2000-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

pub const DEBUG: c_int = 1;

pub const DEBUG_EXPENSIVE: c_int = 1;

pub const XFS_ASSERT_FATAL: c_int = 1;

pub const XFS_WARN: c_int = 1;

//
// Kernel specific type declarations for XFS
//
pub type xfs_dev_t = __u32;
pub type xfs_nlink_t = __u32;

pub const XFS_NATIVE_HOST: c_int = 1;

//
// Size of block device i/o is parameterized here.
// Currently the system supports page-sized i/o.
//

// number of BB's per block device block

//
// Return the address of a label.  Use barrier() so that the optimizer
// won't reorder code to refactor the error jumpouts into a single
// return, which throws off the reported address.
//

//
// XFS wrapper structure for sysfs support. It depends on external data
// structures and is embedded in various internal data structures to implement
// the XFS sysfs object heirarchy. Define it here for broad access throughout
// the codebase.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_kobj {
    pub kobject: kobject,
    pub complete: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstats {
    pub xs_stats: *mut xfsstats __percpu,
    pub xs_kobj: xfs_kobj,
}

extern "C" {
    pub fn MKDEV(0x1ff: sysv_major(dev) &, _arg: sysv_minor(dev)) -> return;
}
extern "C" {
    pub fn sysv_encode_dev(_arg: dev) -> return;
}
//
// Various platform dependent calls that don't fit anywhere else
//

// If @b is a power of 2, return log2(b).  Else return -1.
// If @b is a power of 2, return a mask of the lower bits, else return zero.

//
// Please note that this ASSERT doesn't kill the kernel. It will if the kernel
// has panic_on_warn set.
//

//
// Use this to catch metadata corruptions that are not caught by block or
// structure verifiers. The reason is that the verifiers check corruptions only
// within the scope of the object being verified.
//

//
// make sure we ignore the inode flag if the filesystem doesn't have a
// configured realtime device.
//

//
// Starting in Linux 4.15, the %p (raw pointer value) printk modifier
// prints a hashed version of the pointer to avoid leaking kernel
// pointers into dmesg.  If we're trying to debug the kernel we want the
// raw values, so override this behavior as best we can.
//

