//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/platform/acgcc.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: acgcc.h - GCC specific defines, etc.
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

// Function name is used for debug output. Non-ANSI, compiler-dependent

//
// This macro is used to tag functions as "printf-like" because
// some compilers (like GCC) can catch printf format string problems.
//

//
// Some compilers complain about unused variables. Sometimes we don't want to
// use all the variables (for example, _acpi_module_name). This allows us
// to tell the compiler warning in a per-variable manner that a variable
// is unused.
//

// GCC supports __VA_ARGS__ in macros
pub const COMPILER_VA_MACRO: c_int = 1;
// GCC supports native multiply/shift on 32-bit platforms
// Macro flag: #define ACPI_USE_NATIVE_MATH64
// GCC did not support __has_attribute until 5.1.

pub const __has_attribute(x): c_int = 0;

//
// Explicitly mark intentional explicit fallthrough to silence
// -Wimplicit-fallthrough in GCC 7.1+.
//

//
// Flexible array members are not allowed to be part of a union under
// C99, but this is not for any technical reason. Work around the
// limitation.
//

//
// Explicitly mark strings that lack a terminating NUL character so
// that ACPICA can be built with -Wunterminated-string-initialization.
//

