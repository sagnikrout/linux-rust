//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifs_unicode.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cifs_unicode:  Unicode kernel case support
//
// Function:
// Convert a unicode character to upper or lower case using
// compressed tables.
//
// Copyright (c) International Business Machines  Corp., 2000,2009
//
// Notes:
// These APIs are based on the C library functions.  The semantics
// should match the C functions but with expanded size operands.
//
// The upper/lower functions are based on a table created by mkupr.
// This is a compressed table of upper and lower case conversion.
//

//
// Macs use an older "SFM" mapping of the symbols above. Fortunately it does
// not conflict (although almost does) with the mapping above.
//

//
// Mapping mechanism to use when one of the seven reserved characters is
// encountered.  We can only map using one of the mechanisms at a time
// since otherwise readdir could return directory entries which we would
// not be able to open
//
// NO_MAP_UNI_RSVD  = do not perform any remapping of the character
// SFM_MAP_UNI_RSVD = map reserved characters using SFM scheme (MAC compatible)
// SFU_MAP_UNI_RSVD = map reserved characters ala SFU ("mapchars" option)
//
pub const NO_MAP_UNI_RSVD: c_int = 0;
pub const SFM_MAP_UNI_RSVD: c_int = 1;
pub const SFU_MAP_UNI_RSVD: c_int = 2;
extern "C" {
    pub fn cifs_toupper(in: wchar_t) -> wchar_t;
}
