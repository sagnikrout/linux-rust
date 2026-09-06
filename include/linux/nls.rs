//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nls.h
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

// Unicode has changed over the years.  Unicode code points no longer
// fit into 16 bits; as of Unicode 5 valid code points range from 0
// to 0x10ffff (17 planes, where each plane holds 65536 code points).
//
// The original decision to represent Unicode characters as 16-bit
// wchar_t values is now outdated.  But plane 0 still includes the
// most commonly used characters, so we will retain it.  The newer
// 32-bit unicode_t type can be used when it is necessary to
// represent the full Unicode character set.
//
// Plane-0 Unicode character
pub type wchar_t = u16;
pub const MAX_WCHAR_T: c_uint = 0xffff;
// Arbitrary Unicode character
pub type unicode_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nls_table {
    pub charset: *const c_char,
    pub alias: *const c_char,
    pub boundlen): *mut *mut *mut int (uni2char) (wchar_t uni, unsigned char out, int,
    pub uni): *mut wchar_t,
    pub charset2lower: *const c_uchar,
    pub charset2upper: *const c_uchar,
    pub owner: *mut module,
    pub next: *mut nls_table,
}

// this value hold the maximum octet of charset

// Byte order for UTF-16 strings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum utf16_endian {
    UTF16_HOST_ENDIAN,
    UTF16_LITTLE_ENDIAN,
    UTF16_BIG_ENDIAN
}

// nls_base.c
extern "C" {
    pub fn __register_nls(: *mut nls_table, : *mut module) -> c_int;
}
extern "C" {
    pub fn unregister_nls(: *mut nls_table) -> c_int;
}
extern "C" {
    pub fn unload_nls(: *mut nls_table);
}

extern "C" {
    pub fn utf8_to_utf32(s: *const u8, len: c_int, pu: *mut unicode_t) -> c_int;
}
extern "C" {
    pub fn utf32_to_utf8(u: unicode_t, s: *mut u8, maxlen: c_int) -> c_int;
}
//
// nls_nullsize - return length of null character for codepage
// @codepage - codepage for which to return length of NULL terminator
//
// Since we can't guarantee that the null terminator will be a particular
// length, we have to check against the codepage. If there's a problem
// determining it, assume a single-byte NULL terminator.
//

