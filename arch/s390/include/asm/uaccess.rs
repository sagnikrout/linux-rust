//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/uaccess.h
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
// S390 version
// Copyright IBM Corp. 1999, 2000
// Author(s): Hartmut Penner (hp@de.ibm.com),
// Martin Schwidefsky (schwidefsky@de.ibm.com)
//
// Derived from "include/asm-i386/uaccess.h"
//
// User space memory access functions
//

extern "C" {
    pub fn debug_user_asce(exit: c_int);
}

// Macro flag: #define INLINE_COPY_USER
extern "C" {
    pub fn __put_user_bad() -> int __noreturn;
}

extern "C" {
    pub fn __get_user_bad() -> int __noreturn;
}

// to = 0;							\

// to = 0;							\

//
// Copy a null terminated string from userspace.
//
extern "C" {
    pub fn strncpy_from_user(dst: *mut c_char, src: *const char __user, count: c_long) -> long __must_check;
}
extern "C" {
    pub fn strnlen_user(src: *const char __user, count: c_long) -> long __must_check;
}
extern "C" {
    pub fn __clear_user(_arg: to, _arg: n) -> return;
}
extern "C" {
    pub fn memcpy(_arg: dst, _arg: src, _arg: size) -> return;
}
extern "C" {
    pub fn __s390_kernel_write(_arg: dst, _arg: src, _arg: size) -> return;
}
extern "C" {
    pub fn __mvc_kernel_nofault_bad() -> void __noreturn;
}

