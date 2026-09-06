//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/string.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// string function definitions for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

//
// As much as possible, please keep functions alphabetically sorted.
//

// might be ignored by the compiler without -ffreestanding, then found as
// missing.
//

// must be exported, as it's used by libgcc on ARM

// might be ignored by the compiler without -ffreestanding, then found as
// missing.
//
// prevent gcc from recognizing memset() here
extern "C" {
    pub fn volatile(_arg: "") -> __asm__;
}
// (p++) = b;

// this function is only used with arguments that are not constants or when
// it's not known because optimizations are disabled. Note that gcc 12
// recognizes an strlen() pattern and replaces it with a jump to strlen(),
// thus itself, hence the asm() statement below that's meant to disable this
// confusing practice.
//
extern "C" {
    pub fn strlen(str: *const c_char) -> usize;
}
// do not trust __builtin_constant_p() at -O0, as clang will emit a test and
// the two branches, then will rely on an external definition of strlen().
//

//
// We want len < size-1. But as size is unsigned and can wrap
// around, we use len + 1 instead.
//
// dst = 0;
