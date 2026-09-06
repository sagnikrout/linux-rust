//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/string.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2013 Regents of the University of California
//

extern "C" {
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> asmlinkage int;
}
extern "C" {
    pub fn strlen(: *const c_char) -> asmlinkage __kernel_size_t;
}
extern "C" {
    pub fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> asmlinkage int;
}
extern "C" {
    pub fn strnlen(: *const c_char, _arg: usize) -> asmlinkage __kernel_size_t;
}

// For those files which don't want to check by kasan.

