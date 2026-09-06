//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/auto_dev-ioctl.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright 2008 Red Hat, Inc. All rights reserved.
// Copyright 2008 Ian Kent <raven@themaw.net>
//
// This file is part of the Linux kernel and is made available under
// the terms of the GNU General Public License, version 2, or at your
// option, any later version, incorporated herein by reference.
//

pub const AUTOFS_DEV_IOCTL_VERSION_MAJOR: c_int = 1;
pub const AUTOFS_DEV_IOCTL_VERSION_MINOR: c_int = 1;

//
// An ioctl interface for autofs mount point control.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_protover {
    pub version: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_protosubver {
    pub sub_version: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_openmount {
    pub devid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_ready {
    pub token: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_fail {
    pub token: __u32,
    pub status: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_setpipefd {
    pub pipefd: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_timeout {
    pub timeout: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_requester {
    pub uid: __u32,
    pub gid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_expire {
    pub how: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_askumount {
    pub may_umount: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_ismountpoint {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_in {
    pub type: __u32,
    pub in: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_out {
    pub devid: __u32,
    pub magic: __u32,
    pub out: },
}

//
// All the ioctls use this structure.
// When sending a path size must account for the total length
// of the chunk of memory otherwise it is the size of the
// structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct autofs_dev_ioctl {
    pub ver_major: __u32,
    pub ver_minor: __u32,
    pub in: *mut *mut __u32 size; / total size of data passed,
// including this struct
    pub /: *mut *mut __s32 ioctlfd; / automount command fd,
// Command parameters
    pub protover: args_protover,
    pub protosubver: args_protosubver,
    pub openmount: args_openmount,
    pub ready: args_ready,
    pub fail: args_fail,
    pub setpipefd: args_setpipefd,
    pub timeout: args_timeout,
    pub requester: args_requester,
    pub expire: args_expire,
    pub askumount: args_askumount,
    pub ismountpoint: args_ismountpoint,
}

// Get various version info
// Open mount ioctl fd
// Close mount ioctl fd
// Mount/expire status returns
// Activate/deactivate autofs mount
// Expiry timeout
// Get mount last requesting uid and gid
// Check for eligible expire candidates
// Request busy status
// Check if path is a mountpoint

