//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kstrtox.h
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

// Internal, do not use.
extern "C" {
    pub fn _kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> int __must_check;
}
extern "C" {
    pub fn _kstrtol(s: *const c_char, base: c_uint, res: *mut c_long) -> int __must_check;
}
extern "C" {
    pub fn kstrtoull(s: *const c_char, base: c_uint, res: *mut c_ulonglong) -> int __must_check;
}
extern "C" {
    pub fn kstrtoll(s: *const c_char, base: c_uint, res: *mut c_longlong) -> int __must_check;
}
//
// kstrtoul - convert a string to an unsigned long
// @s: The start of the string. The string must be null-terminated, and may also
// include a single newline before its terminating null. The first character
// may also be a plus sign, but not a minus sign.
// @base: The number base to use. The maximum supported base is 16. If base is
// given as 0, then the base of the string is automatically detected with the
// conventional semantics - If it begins with 0x the number will be parsed as a
// hexadecimal (case insensitive), if it otherwise begins with 0, it will be
// parsed as an octal number. Otherwise it will be parsed as a decimal.
// @res: Where to write the result of the conversion on success.
//
// Returns 0 on success, -ERANGE on overflow and -EINVAL on parsing error.
// Preferred over simple_strtoul(). Return code must be checked.
//
// We want to shortcut function call, but
// __builtin_types_compatible_p(unsigned long, unsigned long long) = 0.
//
extern "C" {
    pub fn kstrtoull(_arg: s, _arg: base, )res: *mut (unsigned long long) -> return;
}
extern "C" {
    pub fn _kstrtoul(_arg: s, _arg: base, _arg: res) -> return;
}
//
// kstrtol - convert a string to a long
// @s: The start of the string. The string must be null-terminated, and may also
// include a single newline before its terminating null. The first character
// may also be a plus sign or a minus sign.
// @base: The number base to use. The maximum supported base is 16. If base is
// given as 0, then the base of the string is automatically detected with the
// conventional semantics - If it begins with 0x the number will be parsed as a
// hexadecimal (case insensitive), if it otherwise begins with 0, it will be
// parsed as an octal number. Otherwise it will be parsed as a decimal.
// @res: Where to write the result of the conversion on success.
//
// Returns 0 on success, -ERANGE on overflow and -EINVAL on parsing error.
// Preferred over simple_strtol(). Return code must be checked.
//
// We want to shortcut function call, but
// __builtin_types_compatible_p(long, long long) = 0.
//
extern "C" {
    pub fn kstrtoll(_arg: s, _arg: base, )res: *mut (long long) -> return;
}
extern "C" {
    pub fn _kstrtol(_arg: s, _arg: base, _arg: res) -> return;
}
extern "C" {
    pub fn kstrtouint(s: *const c_char, base: c_uint, res: *mut c_uint) -> int __must_check;
}
extern "C" {
    pub fn kstrtoint(s: *const c_char, base: c_uint, res: *mut c_int) -> int __must_check;
}
extern "C" {
    pub fn kstrtoull(_arg: s, _arg: base, _arg: res) -> return;
}
extern "C" {
    pub fn kstrtoll(_arg: s, _arg: base, _arg: res) -> return;
}
extern "C" {
    pub fn kstrtouint(_arg: s, _arg: base, _arg: res) -> return;
}
extern "C" {
    pub fn kstrtoint(_arg: s, _arg: base, _arg: res) -> return;
}
extern "C" {
    pub fn kstrtou16(s: *const c_char, base: c_uint, res: *mut u16) -> int __must_check;
}
extern "C" {
    pub fn kstrtos16(s: *const c_char, base: c_uint, res: *mut i16) -> int __must_check;
}
extern "C" {
    pub fn kstrtou8(s: *const c_char, base: c_uint, res: *mut u8) -> int __must_check;
}
extern "C" {
    pub fn kstrtos8(s: *const c_char, base: c_uint, res: *mut i8) -> int __must_check;
}
extern "C" {
    pub fn kstrtobool(s: *const c_char, res: *mut bool) -> int __must_check;
}
extern "C" {
    pub fn kstrtoudec64(s: *const c_char, scale: c_uint, res: *mut u64) -> int __must_check;
}
extern "C" {
    pub fn kstrtodec64(s: *const c_char, scale: c_uint, res: *mut i64) -> int __must_check;
}
extern "C" {
    pub fn kstrtoull_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut c_ulonglong) -> int __must_check;
}
extern "C" {
    pub fn kstrtoll_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut c_longlong) -> int __must_check;
}
extern "C" {
    pub fn kstrtoul_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut c_ulong) -> int __must_check;
}
extern "C" {
    pub fn kstrtol_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut c_long) -> int __must_check;
}
extern "C" {
    pub fn kstrtouint_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut c_uint) -> int __must_check;
}
extern "C" {
    pub fn kstrtoint_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut c_int) -> int __must_check;
}
extern "C" {
    pub fn kstrtou16_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut u16) -> int __must_check;
}
extern "C" {
    pub fn kstrtos16_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut i16) -> int __must_check;
}
extern "C" {
    pub fn kstrtou8_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut u8) -> int __must_check;
}
extern "C" {
    pub fn kstrtos8_from_user(s: *const char __user, count: usize, base: c_uint, res: *mut i8) -> int __must_check;
}
extern "C" {
    pub fn kstrtobool_from_user(s: *const char __user, count: usize, res: *mut bool) -> int __must_check;
}
extern "C" {
    pub fn kstrtoull_from_user(_arg: s, _arg: count, _arg: base, _arg: res) -> return;
}
extern "C" {
    pub fn kstrtoll_from_user(_arg: s, _arg: count, _arg: base, _arg: res) -> return;
}
extern "C" {
    pub fn kstrtouint_from_user(_arg: s, _arg: count, _arg: base, _arg: res) -> return;
}
extern "C" {
    pub fn kstrtoint_from_user(_arg: s, _arg: count, _arg: base, _arg: res) -> return;
}
//
// Use kstrto<foo> instead.
//
// NOTE: simple_strto<foo> does not check for the range overflow and,
// depending on the input, may give interesting results.
//
// Use these functions if and only if you cannot use kstrto<foo>, because
// the conversion ends on the first non-digit character, which may be far
// beyond the supported range. It might be useful to parse the strings like
// 10x50 or 12:21 without altering original string or temporary buffer in use.
// Keep in mind above caveat.
//
extern "C" {
    pub fn simple_strtoul(cp: *const c_char, endp: *mut c_char, base: c_uint) -> c_ulong;
}
extern "C" {
    pub fn simple_strtol(cp: *const c_char, endp: *mut c_char, base: c_uint) -> c_long;
}
extern "C" {
    pub fn simple_strtoull(cp: *const c_char, endp: *mut c_char, base: c_uint) -> c_ulonglong;
}
extern "C" {
    pub fn simple_strtoll(cp: *const c_char, endp: *mut c_char, base: c_uint) -> c_longlong;
}
