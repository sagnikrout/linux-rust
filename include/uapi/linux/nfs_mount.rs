//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfs_mount.h
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
// linux/include/linux/nfs_mount.h
//
// Copyright (C) 1992  Rick Sladkey
//
// structure passed from user-space to kernel-space during an nfs mount
//

//
// WARNING!  Do not delete or change the order of these fields.  If
// a new field is required then add it to the end.  The version field
// tracks which fields are present.  This will ensure some measure of
// mount-to-kernel version compatibility.  Some of these aren't used yet
// but here they are anyway.
//
pub const NFS_MOUNT_VERSION: c_int = 6;
pub const NFS_MAX_CONTEXT_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_mount_data {
    pub /: *mut *mut int version; / 1,
    pub /: *mut *mut int fd; / 1,
    pub /: *mut *mut nfs2_fh old_root; / 1,
    pub /: *mut *mut int flags; / 1,
    pub /: *mut *mut int rsize; / 1,
    pub /: *mut *mut int wsize; / 1,
    pub /: *mut *mut int timeo; / 1,
    pub /: *mut *mut int retrans; / 1,
    pub /: *mut *mut int acregmin; / 1,
    pub /: *mut *mut int acregmax; / 1,
    pub /: *mut *mut int acdirmin; / 1,
    pub /: *mut *mut int acdirmax; / 1,
    pub /: *mut *mut sockaddr_in addr; / 1,
    pub /: *mut *mut char hostname[NFS_MAXNAMLEN + 1]; / 1,
    pub /: *mut *mut int namlen; / 2,
    pub /: *mut *mut unsigned int bsize; / 3,
    pub /: *mut *mut nfs3_fh root; / 4,
    pub /: *mut *mut int pseudoflavor; / 5,
    pub /: *mut *mut char context[NFS_MAX_CONTEXT_LEN + 1]; / 6,
}

// bits in the flags field visible to user space
pub const NFS_MOUNT_SOFT: c_uint = 0x0001	/* 1 */;
pub const NFS_MOUNT_INTR: c_uint = 0x0002	/* 1 */ /* now unused, but ABI */;
pub const NFS_MOUNT_SECURE: c_uint = 0x0004	/* 1 */;
pub const NFS_MOUNT_POSIX: c_uint = 0x0008	/* 1 */;
pub const NFS_MOUNT_NOCTO: c_uint = 0x0010	/* 1 */;
pub const NFS_MOUNT_NOAC: c_uint = 0x0020	/* 1 */;
pub const NFS_MOUNT_TCP: c_uint = 0x0040	/* 2 */;
pub const NFS_MOUNT_VER3: c_uint = 0x0080	/* 3 */;
pub const NFS_MOUNT_KERBEROS: c_uint = 0x0100	/* 3 */;
pub const NFS_MOUNT_NONLM: c_uint = 0x0200	/* 3 */;
pub const NFS_MOUNT_BROKEN_SUID: c_uint = 0x0400	/* 4 */;
pub const NFS_MOUNT_NOACL: c_uint = 0x0800	/* 4 */;
pub const NFS_MOUNT_STRICTLOCK: c_uint = 0x1000	/* reserved for NFSv4 */;
pub const NFS_MOUNT_SECFLAVOUR: c_uint = 0x2000	/* 5 non-text parsed mount data only */;
pub const NFS_MOUNT_NORDIRPLUS: c_uint = 0x4000	/* 5 */;
pub const NFS_MOUNT_UNSHARED: c_uint = 0x8000	/* 5 */;
pub const NFS_MOUNT_FLAGMASK: c_uint = 0xFFFF;
