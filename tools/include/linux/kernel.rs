//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/kernel.h
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

// Macro flag: #define cpu_to_be16
// Macro flag: #define cpu_to_be32
// Macro flag: #define cpu_to_be64
// Macro flag: #define be16_to_cpu
// Macro flag: #define be32_to_cpu
// Macro flag: #define be64_to_cpu

// Macro flag: #define cpu_to_le16
// Macro flag: #define cpu_to_le32
// Macro flag: #define cpu_to_le64
// Macro flag: #define le16_to_cpu
// Macro flag: #define le32_to_cpu
// Macro flag: #define le64_to_cpu

extern "C" {
    pub fn vscnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn scnprintf(buf: *mut *mut c_char, size: usize, fmt: *const *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn scnprintf_pad(buf: *mut *mut c_char, size: usize, fmt: *const *const c_char, ...) -> c_int;
}

pub const current_gfp_context(k): c_int = 0;
// Macro flag: #define synchronize_rcu()
