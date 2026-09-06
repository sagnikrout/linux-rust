//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/parser.h
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
//
// linux/include/linux/parser.h
//
// Header for lib/parser.c
// Intended use of these functions is parsing filesystem argument lists,
// but could potentially be used anywhere else that simple option=arg
// parsing is required.
//
// associates an integer enumerator with a pattern string.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct match_token {
    pub token: c_int,
    pub pattern: *const c_char,
}

// Maximum number of arguments that match_token will find in a pattern
// Describe the location within a string of a substring
extern "C" {
    pub fn match_token(: *mut c_char, table: match_table_t, args[]: substring_t) -> c_int;
}
extern "C" {
    pub fn match_int(: *mut substring_t, result: *mut c_int) -> c_int;
}
extern "C" {
    pub fn match_uint(s: *mut substring_t, result: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn match_u64(: *mut substring_t, result: *mut u64) -> c_int;
}
extern "C" {
    pub fn match_octal(: *mut substring_t, result: *mut c_int) -> c_int;
}
extern "C" {
    pub fn match_hex(: *mut substring_t, result: *mut c_int) -> c_int;
}
extern "C" {
    pub fn match_wildcard(pattern: *const c_char, str: *const c_char) -> bool;
}
extern "C" {
    pub fn match_strlcpy(: *mut c_char, : *const substring_t, _arg: usize) -> usize;
}
