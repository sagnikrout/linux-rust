//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/boot/string.h
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
// Undef any of these macros coming from string_32.h.

extern "C" {
    pub fn memcmp(s1: *const c_void, s2: *const c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn bcmp(s1: *const c_void, s2: *const c_void, len: usize) -> c_int;
}
// Access builtin version by default.

extern "C" {
    pub fn strcmp(str1: *const c_char, str2: *const c_char) -> c_int;
}
extern "C" {
    pub fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int;
}
extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
}
extern "C" {
    pub fn strnlen(s: *const c_char, maxlen: usize) -> usize;
}
extern "C" {
    pub fn simple_strtol(cp: *const c_char, endp: *mut c_char, base: c_uint) -> c_long;
}
extern "C" {
    pub fn boot_kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
}
