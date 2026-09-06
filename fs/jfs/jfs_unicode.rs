//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_unicode.h
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
// Copyright (C) International Business Machines Corp., 2000-2002
// Portions Copyright (C) Christoph Hellwig, 2001-2002
//

extern "C" {
    pub fn get_UCSname(: *mut component_name, : *mut dentry) -> c_int;
}
extern "C" {
    pub fn jfs_strfromUCS_le(: *mut c_char, : *const __le16, _arg: c_int, : *mut nls_table) -> c_int;
}

//
// UniStrcpy:  Copy a string
//
// UniStrncpy:  Copy length limited string with pad
//
// ucs1++ = *ucs2++;
// ucs1++ = 0;
//
// UniStrncmp_le:  Compare length limited string - native to little-endian
//
// UniStrncpy_to_le:  Copy length limited string with pad to little-endian
//
// ucs1++ = cpu_to_le16(*ucs2++);
// ucs1++ = 0;
//
// UniStrncpy_from_le:  Copy length limited string with pad from little-endian
//
// ucs1++ = __le16_to_cpu(*ucs2++);
// ucs1++ = 0;
//
// UniToupper:  Convert a unicode character to upper case
//
// UniStrupr:  Upper case a unicode string
//
// up = UniToupper(*up);
