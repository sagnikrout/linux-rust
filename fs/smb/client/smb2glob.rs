//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/smb2glob.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Definitions for various global variables and structures
//
// Copyright (C) International Business Machines  Corp., 2002, 2011
// Etersoft, 2012
// Author(s): Steve French (sfrench@us.ibm.com)
// Jeremy Allison (jra@samba.org)
// Pavel Shilovsky (pshilovsky@samba.org) 2012
//
// Constants go here
//
// Identifiers for functions that use the open, operation, close pattern
// in smb2inode.c:smb2_compound_op()
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smb2_compound_ops {
    SMB2_OP_SET_DELETE = 1,
    SMB2_OP_SET_INFO,
    SMB2_OP_QUERY_INFO,
    SMB2_OP_QUERY_DIR,
    SMB2_OP_MKDIR,
    SMB2_OP_RENAME,
    SMB2_OP_HARDLINK,
    SMB2_OP_SET_EOF,
    SMB2_OP_UNLINK,
    SMB2_OP_POSIX_QUERY_INFO,
    SMB2_OP_SET_REPARSE,
    SMB2_OP_GET_REPARSE,
    SMB2_OP_QUERY_WSL_EA,
    SMB2_OP_OPEN_QUERY,
}

// Used when constructing chained read requests.
pub const CHAINED_REQUEST: c_int = 1;
pub const START_OF_CHAIN: c_int = 2;
pub const END_OF_CHAIN: c_int = 4;
pub const RELATED_REQUEST: c_int = 8;
//
// Struct definitions go here
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_to_posix_error {
    pub smb2_status: __u32,
    pub posix_error: c_int,
    pub status_string: *mut c_char,
}
