//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/string.h
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

extern "C" {
    pub fn argv_free(argv: *mut c_char);
}
extern "C" {
    pub fn strtobool(s: *const c_char, res: *mut bool) -> c_int;
}

//
// glibc based builds needs the extern while uClibc doesn't.
// However uClibc headers also define __GLIBC__ hence the hack below
//

// pragma diagnostic was introduced in gcc 4.6

extern "C" {
    pub fn strlcpy(dest: *mut c_char, src: *const c_char, size: usize) -> usize;
}

//
// strstarts - does @str start with @prefix?
// @str: string to examine
// @prefix: prefix to look for.
//
// Checks if a string ends with another.
//
extern "C" {
    pub fn skip_spaces(: *const c_char) -> *mut char  __must_check;
}
extern "C" {
    pub fn remove_spaces(s: *mut c_char);
}
extern "C" {
    pub fn memparse(ptr: *const c_char, retptr: *mut c_char) -> c_ulonglong;
}
