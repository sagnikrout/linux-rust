//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/uaccess.h
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
// User space memory access functions, these should work
// on any machine that has kernel and user data in the same
// address space, e.g. all NOMMU machines.
//

// (u8 *)to = *((u8  *)from);
// (u16 *)to = get_unaligned((u16  *)from);
// (u32 *)to = get_unaligned((u32  *)from);
// (u64 *)to = get_unaligned((u64  *)from);

// (u8  *)to = *(u8 *)from;

// ((type *)dst) = get_unaligned((type *)(src));			\

// Macro flag: #define INLINE_COPY_USER

//
// These are the main single-value transfer routines.  They automatically
// use the right size if we just have the right pointer type.
// This version just falls back to copy_{from,to}_user, which should
// provide a fast-path for small values.
//

extern "C" {
    pub fn __put_user_bad(__attribute__((noreturn): void)) -> c_int;
}

extern "C" {
    pub fn __get_user_bad(__attribute__((noreturn): void)) -> c_int;
}
//
// Zero Userspace
//

extern "C" {
    pub fn __clear_user(_arg: to, _arg: n) -> return;
}

extern "C" {
    pub fn strnlen_user(src: *const char __user, n: c_long) -> __must_check long;
}
