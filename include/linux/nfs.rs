//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nfs.h
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
// NFS protocol definitions
//
// This file contains constants mostly for Version 2 of the protocol,
// but also has a couple of NFSv3 bits in (notably the error codes).
//

// The LOCALIO program is entirely private to Linux and is
// NOT part of the uapi.
//
pub const NFS_LOCALIO_PROGRAM: c_int = 400122;
pub const LOCALIOPROC_NULL: c_int = 0;
pub const LOCALIOPROC_UUID_IS_LOCAL: c_int = 1;
//
// This is the kernel NFS client file handle representation
//
pub const NFS_MAXFHSIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_fh {
    pub size: c_ushort,
    pub data: [c_uchar; NFS_MAXFHSIZE],
}

//
// Returns a zero iff the size and data fields match.
// Checks only "size" bytes in the data field.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs3_stable_how {
    NFS_UNSTABLE = 0,
    NFS_DATA_SYNC = 1,
    NFS_FILE_SYNC = 2,

// used by direct.c to mark verf as invalid
    NFS_INVALID_STABLE_HOW = -1
}

//
// nfs_fhandle_hash - calculate the crc32 hash for the filehandle
// @fh - pointer to filehandle
//
// returns a crc32 hash for the filehandle that is compatible with
// the one displayed by "wireshark".
//
