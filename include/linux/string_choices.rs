//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/string_choices.h
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
// Here provide a series of helpers in the str_$TRUE_$FALSE format (you can
// also expand some helpers as needed), where $TRUE and $FALSE are their
// corresponding literal strings. These helpers can be used in the printing
// and also in other places where constant strings are required. Using these
// helpers offers the following benefits:
// 1) Reducing the hardcoding of strings, which makes the code more elegant
// through these simple literal-meaning helpers.
// 2) Unifying the output, which prevents the same string from being printed
// in various forms, such as enable/disable, enabled/disabled, en/dis.
// 3) Deduping by the linker, which results in a smaller binary file.
//

//
// str_plural - Return the simple pluralization based on English counts
// @num: Number used for deciding pluralization
//
// If @num is 1, returns empty string, otherwise returns "s".
//
