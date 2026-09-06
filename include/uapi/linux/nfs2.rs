//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfs2.h
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
// NFS protocol definitions
//
// This file contains constants for Version 2 of the protocol.
//
pub const NFS2_PORT: c_int = 2049;
pub const NFS2_MAXDATA: c_int = 8192;
pub const NFS2_MAXPATHLEN: c_int = 1024;
pub const NFS2_MAXNAMLEN: c_int = 255;
pub const NFS2_MAXGROUPS: c_int = 16;
pub const NFS2_FHSIZE: c_int = 32;
pub const NFS2_COOKIESIZE: c_int = 4;

pub const NFS2MODE_FMT: c_int = 0170000;
pub const NFS2MODE_DIR: c_int = 0040000;
pub const NFS2MODE_CHR: c_int = 0020000;
pub const NFS2MODE_BLK: c_int = 0060000;
pub const NFS2MODE_REG: c_int = 0100000;
pub const NFS2MODE_LNK: c_int = 0120000;
pub const NFS2MODE_SOCK: c_int = 0140000;
pub const NFS2MODE_FIFO: c_int = 0010000;
// NFSv2 file types - beware, these are not the same in NFSv3
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs2_ftype {
    NF2NON = 0,
    NF2REG = 1,
    NF2DIR = 2,
    NF2BLK = 3,
    NF2CHR = 4,
    NF2LNK = 5,
    NF2SOCK = 6,
    NF2BAD = 7,
    NF2FIFO = 8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs2_fh {
    pub data: [c_char; NFS2_FHSIZE],
}

//
// Procedure numbers for NFSv2
//
pub const NFS2_VERSION: c_int = 2;
pub const NFSPROC_NULL: c_int = 0;
pub const NFSPROC_GETATTR: c_int = 1;
pub const NFSPROC_SETATTR: c_int = 2;
pub const NFSPROC_ROOT: c_int = 3;
pub const NFSPROC_LOOKUP: c_int = 4;
pub const NFSPROC_READLINK: c_int = 5;
pub const NFSPROC_READ: c_int = 6;
pub const NFSPROC_WRITECACHE: c_int = 7;
pub const NFSPROC_WRITE: c_int = 8;
pub const NFSPROC_CREATE: c_int = 9;
pub const NFSPROC_REMOVE: c_int = 10;
pub const NFSPROC_RENAME: c_int = 11;
pub const NFSPROC_LINK: c_int = 12;
pub const NFSPROC_SYMLINK: c_int = 13;
pub const NFSPROC_MKDIR: c_int = 14;
pub const NFSPROC_RMDIR: c_int = 15;
pub const NFSPROC_READDIR: c_int = 16;
pub const NFSPROC_STATFS: c_int = 17;
