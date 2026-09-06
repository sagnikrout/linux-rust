//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/common/smb2status.h
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
// SMB2 Status code (network error) definitions
// Definitions are from MS-ERREF
//
// Copyright (c) International Business Machines  Corp., 2009,2011
// Author(s): Steve French (sfrench@us.ibm.com)
//
// 0 1 2 3 4 5 6 7 8 9 0 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F
// SEV C N <-------Facility--------> <------Error Status Code------>
//
// C is set if "customer defined" error, N bit is reserved and MBZ
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntstatus {
// Facility is the high 12 bits of the following field
    pub /: *mut *mut __le32 Facility; / low 2 bits Severity, next is Customer, then rsrvd,
    pub Code: __le32,
}

//
// The comment at the end of each definition indicates `posix_error`
// field of `struct status_to_posix_error`, it is used to generate the
// `smb2_error_map_table` array.
//

//
// 'OCCURED' is typo in MS-ERREF, it should be 'OCCURRED',
// but we'll keep it consistent with MS-ERREF.
//

// See MS-SMB2 3.3.5.4

