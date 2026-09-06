//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/string_helpers.h
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

// Descriptions of the types of units to print in
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum string_size_units {
    STRING_UNITS_10,	/* use powers of 10^3 (standard SI) */
    STRING_UNITS_2,		/* use binary powers of 2^10 */
    STRING_UNITS_MASK	= BIT(0),

// Modifiers
    STRING_UNITS_NO_SPACE	= BIT(30),
    STRING_UNITS_NO_BYTES	= BIT(31),
}

extern "C" {
    pub fn parse_int_array(buf: *const c_char, count: usize, array: *mut c_int) -> c_int;
}
extern "C" {
    pub fn parse_int_array_user(from: *const char __user, count: usize, array: *mut c_int) -> c_int;
}

extern "C" {
    pub fn string_unescape(src: *mut c_char, dst: *mut c_char, size: usize, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn string_unescape(_arg: buf, _arg: buf, _arg: 0, _arg: flags) -> return;
}
extern "C" {
    pub fn string_unescape(_arg: src, _arg: dst, _arg: size, _arg: UNESCAPE_ANY) -> return;
}
extern "C" {
    pub fn string_unescape_any(_arg: buf, _arg: buf, _arg: 0) -> return;
}

extern "C" {
    pub fn string_escape_mem(_arg: src, _arg: isz, _arg: dst, _arg: osz, _arg: ESCAPE_ANY_NP, _arg: only) -> return;
}
extern "C" {
    pub fn string_escape_mem(_arg: src, _arg: strlen(src), _arg: dst, _arg: sz, _arg: flags, _arg: only) -> return;
}
extern "C" {
    pub fn string_escape_str(_arg: src, _arg: dst, _arg: sz, _arg: ESCAPE_ANY_NP, _arg: only) -> return;
}
// dst++ = toupper(*src);
// dst++ = tolower(*src);
extern "C" {
    pub fn kfree_strarray(array: *mut c_char, n: usize);
}
