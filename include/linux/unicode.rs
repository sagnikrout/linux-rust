//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/unicode.h
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

pub const UNICODE_MAJ_SHIFT: c_int = 16;
pub const UNICODE_MIN_SHIFT: c_int = 8;

//
// Two normalization forms are supported:
// 1) NFDI
// - Apply unicode normalization form NFD.
// - Remove any Default_Ignorable_Code_Point.
// 2) NFDICF
// - Apply unicode normalization form NFD.
// - Remove any Default_Ignorable_Code_Point.
// - Apply a full casefold (C + F).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum utf8_normalization {
    UTF8_NFDI = 0,
    UTF8_NFDICF,
    UTF8_NMAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unicode_map {
    pub version: c_uint,
    pub ntab: [*const utf8data; UTF8_NMAX],
    pub tables: *const utf8data_table,
}

extern "C" {
    pub fn utf8_validate(um: *const unicode_map, str: *const qstr) -> c_int;
}
extern "C" {
    pub fn utf8_unload(um: *mut unicode_map);
}
extern "C" {
    pub fn utf8_parse_version(version: *mut c_char) -> c_int;
}
