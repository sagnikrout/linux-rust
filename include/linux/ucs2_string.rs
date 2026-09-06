//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ucs2_string.h
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

pub type ucs2_char_t = u16;
extern "C" {
    pub fn ucs2_strnlen(s: *const ucs2_char_t, maxlength: usize) -> c_ulong;
}
extern "C" {
    pub fn ucs2_strlen(s: *const ucs2_char_t) -> c_ulong;
}
extern "C" {
    pub fn ucs2_strsize(data: *const ucs2_char_t, maxlength: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn ucs2_strscpy(dst: *mut ucs2_char_t, src: *const ucs2_char_t, count: usize) -> isize;
}
extern "C" {
    pub fn ucs2_strncmp(a: *const ucs2_char_t, b: *const ucs2_char_t, len: usize) -> c_int;
}
extern "C" {
    pub fn ucs2_utf8size(src: *const ucs2_char_t) -> c_ulong;
}
