//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfs3.h
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
// NFSv3 protocol definitions
//
pub const NFS3_PORT: c_int = 2049;
pub const NFS3_MAXDATA: c_int = 32768;

pub const NFS3_MAXGROUPS: c_int = 16;
pub const NFS3_FHSIZE: c_int = 64;
pub const NFS3_COOKIESIZE: c_int = 4;
pub const NFS3_CREATEVERFSIZE: c_int = 8;
pub const NFS3_COOKIEVERFSIZE: c_int = 8;
pub const NFS3_WRITEVERFSIZE: c_int = 8;

pub const NFS3MODE_FMT: c_int = 0170000;
pub const NFS3MODE_DIR: c_int = 0040000;
pub const NFS3MODE_CHR: c_int = 0020000;
pub const NFS3MODE_BLK: c_int = 0060000;
pub const NFS3MODE_REG: c_int = 0100000;
pub const NFS3MODE_LNK: c_int = 0120000;
pub const NFS3MODE_SOCK: c_int = 0140000;
pub const NFS3MODE_FIFO: c_int = 0010000;
// Flags for access() call
pub const NFS3_ACCESS_READ: c_uint = 0x0001;
pub const NFS3_ACCESS_LOOKUP: c_uint = 0x0002;
pub const NFS3_ACCESS_MODIFY: c_uint = 0x0004;
pub const NFS3_ACCESS_EXTEND: c_uint = 0x0008;
pub const NFS3_ACCESS_DELETE: c_uint = 0x0010;
pub const NFS3_ACCESS_EXECUTE: c_uint = 0x0020;
pub const NFS3_ACCESS_FULL: c_uint = 0x003f;
// Flags for create mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs3_createmode {
    NFS3_CREATE_UNCHECKED = 0,
    NFS3_CREATE_GUARDED = 1,
    NFS3_CREATE_EXCLUSIVE = 2
}

// NFSv3 file system properties
pub const NFS3_FSF_LINK: c_uint = 0x0001;
pub const NFS3_FSF_SYMLINK: c_uint = 0x0002;
pub const NFS3_FSF_HOMOGENEOUS: c_uint = 0x0008;
pub const NFS3_FSF_CANSETTIME: c_uint = 0x0010;
// Some shorthands. See fs/nfsd/nfs3proc.c
pub const NFS3_FSF_DEFAULT: c_uint = 0x001B;
pub const NFS3_FSF_BILLYBOY: c_uint = 0x0018;
pub const NFS3_FSF_READONLY: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs3_ftype {
    NF3NON  = 0,
    NF3REG  = 1,
    NF3DIR  = 2,
    NF3BLK  = 3,
    NF3CHR  = 4,
    NF3LNK  = 5,
    NF3SOCK = 6,
    NF3FIFO = 7,	/* changed from NFSv2 (was 8) */
    NF3BAD  = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs3_time_how {
    DONT_CHANGE		= 0,
    SET_TO_SERVER_TIME	= 1,
    SET_TO_CLIENT_TIME	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_fh {
    pub size: c_ushort,
    pub data: [c_uchar; NFS3_FHSIZE],
}

pub const NFS3_VERSION: c_int = 3;
pub const NFS3PROC_NULL: c_int = 0;
pub const NFS3PROC_GETATTR: c_int = 1;
pub const NFS3PROC_SETATTR: c_int = 2;
pub const NFS3PROC_LOOKUP: c_int = 3;
pub const NFS3PROC_ACCESS: c_int = 4;
pub const NFS3PROC_READLINK: c_int = 5;
pub const NFS3PROC_READ: c_int = 6;
pub const NFS3PROC_WRITE: c_int = 7;
pub const NFS3PROC_CREATE: c_int = 8;
pub const NFS3PROC_MKDIR: c_int = 9;
pub const NFS3PROC_SYMLINK: c_int = 10;
pub const NFS3PROC_MKNOD: c_int = 11;
pub const NFS3PROC_REMOVE: c_int = 12;
pub const NFS3PROC_RMDIR: c_int = 13;
pub const NFS3PROC_RENAME: c_int = 14;
pub const NFS3PROC_LINK: c_int = 15;
pub const NFS3PROC_READDIR: c_int = 16;
pub const NFS3PROC_READDIRPLUS: c_int = 17;
pub const NFS3PROC_FSSTAT: c_int = 18;
pub const NFS3PROC_FSINFO: c_int = 19;
pub const NFS3PROC_PATHCONF: c_int = 20;
pub const NFS3PROC_COMMIT: c_int = 21;
pub const NFS_MNT3_VERSION: c_int = 3;
