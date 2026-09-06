//! Automatically rewritten from C Header to Rust Module
//! Source: fs/unicode/utf8n.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2014 SGI.
// All rights reserved.
//

extern "C" {
    pub fn utf8version_is_supported(um: *const unicode_map, version: c_uint) -> c_int;
}
//
// Determine the length of the normalized from of the string,
// excluding any terminating NULL byte.
// Returns 0 if only ignorable code points are present.
// Returns -1 if the input is not valid UTF-8.
//
// Needed in struct utf8cursor below.

//
// Cursor structure used by the normalizer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8cursor {
    pub um: *const unicode_map,
    pub n: utf8_normalization,
    pub s: *const c_char,
    pub p: *const c_char,
    pub ss: *const c_char,
    pub sp: *const c_char,
    pub len: c_uint,
    pub slen: c_uint,
    pub ccc: short int,
    pub nccc: short int,
    pub hangul: [c_uchar; UTF8HANGULLEAF],
}

//
// Initialize a utf8cursor to normalize a string.
// Returns 0 on success.
// Returns -1 on failure.
//
// Get the next byte in the normalization.
// Returns a value > 0 && < 256 on success.
// Returns 0 when the end of the normalization is reached.
// Returns -1 if the string being normalized is not valid UTF-8.
//
extern "C" {
    pub fn utf8byte(u8c: *mut utf8cursor) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8data {
    pub maxage: c_uint,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8data_table {
    pub utf8agetab: *const c_uint,
    pub utf8agetab_size: c_int,
    pub utf8nfdicfdata: *const utf8data,
    pub utf8nfdicfdata_size: c_int,
    pub utf8nfdidata: *const utf8data,
    pub utf8nfdidata_size: c_int,
    pub utf8data: *const c_uchar,
}
