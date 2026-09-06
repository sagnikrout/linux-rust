//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfs4_mount.h
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
// linux/include/linux/nfs4_mount.h
//
// Copyright (C) 2002  Trond Myklebust
//
// structure passed from user-space to kernel-space during an nfsv4 mount
//
// WARNING!  Do not delete or change the order of these fields.  If
// a new field is required then add it to the end.  The version field
// tracks which fields are present.  This will ensure some measure of
// mount-to-kernel version compatibility.  Some of these aren't used yet
// but here they are anyway.
//
pub const NFS4_MOUNT_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_string {
    pub len: c_uint,
    pub data: *const *const char __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_mount_data {
    pub /: *mut *mut int version; / 1,
    pub /: *mut *mut int flags; / 1,
    pub /: *mut *mut int rsize; / 1,
    pub /: *mut *mut int wsize; / 1,
    pub /: *mut *mut int timeo; / 1,
    pub /: *mut *mut int retrans; / 1,
    pub /: *mut *mut int acregmin; / 1,
    pub /: *mut *mut int acregmax; / 1,
    pub /: *mut *mut int acdirmin; / 1,
    pub /: *mut *mut int acdirmax; / 1,
// see the definition of 'struct clientaddr4' in RFC3010
    pub /: *mut *mut nfs_string client_addr; / 1,
// Mount path
    pub /: *mut *mut nfs_string mnt_path; / 1,
// Server details
    pub /: *mut *mut nfs_string hostname; / 1,
// Server IP address
    pub /: *mut *mut unsigned int host_addrlen; / 1,
    pub /: *mut *mut *mut sockaddr __user  host_addr; / 1,
// Transport protocol to use
    pub /: *mut *mut int proto; / 1,
// Pseudo-flavours to use for authentication. See RFC2623
    pub /: *mut *mut int auth_flavourlen; / 1,
    pub /: *mut *mut *mut int __user auth_flavours; / 1,
}

// bits in the flags field
// Note: the fields that correspond to existing NFSv2/v3 mount options
// should mirror the values from include/linux/nfs_mount.h
//
pub const NFS4_MOUNT_SOFT: c_uint = 0x0001	/* 1 */;
pub const NFS4_MOUNT_INTR: c_uint = 0x0002	/* 1 */;
pub const NFS4_MOUNT_NOCTO: c_uint = 0x0010	/* 1 */;
pub const NFS4_MOUNT_NOAC: c_uint = 0x0020	/* 1 */;
pub const NFS4_MOUNT_STRICTLOCK: c_uint = 0x1000	/* 1 */;
pub const NFS4_MOUNT_UNSHARED: c_uint = 0x8000	/* 1 */;
pub const NFS4_MOUNT_FLAGMASK: c_uint = 0x9033;
