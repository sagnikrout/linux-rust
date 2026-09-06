//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/hvconsole.h
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
// hvconsole.h
// Copyright (C) 2004 Ryan S Arnold, IBM Corporation
//
// LPAR console support.
//

//
// PSeries firmware will only send/recv up to 16 bytes of character data per
// hcall.
//
pub const MAX_VIO_PUT_CHARS: c_int = 16;
pub const SIZE_VIO_GET_CHARS: c_int = 16;
//
// Vio firmware always attempts to fetch MAX_VIO_GET_CHARS chars.  The 'count'
// parm is included to conform to put_chars() function pointer template
//
extern "C" {
    pub fn hvc_get_chars(vtermno: u32, buf: *mut u8, count: usize) -> isize;
}
extern "C" {
    pub fn hvc_put_chars(vtermno: u32, buf: *const u8, count: usize) -> isize;
}
// Provided by HVC VIO
extern "C" {
    pub fn hvc_vio_init_early();
}

