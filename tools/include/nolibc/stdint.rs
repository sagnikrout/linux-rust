//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/stdint.h
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
// Standard definitions and types for NOLIBC
// Copyright (C) 2023 Vincent Dagonneau <v@vda.io>
//
pub type uint8_t = c_uchar;
pub type int8_t = i8;
pub type uint16_t = c_ushort;
pub type int16_t = signed short;
pub type uint32_t = c_uint;
pub type int32_t = i32;
pub type uint64_t = c_ulonglong;
pub type int64_t = signed long long;
pub type size_t = __SIZE_TYPE__;
pub type ssize_t = signed long;
pub type uintptr_t = c_ulong;
pub type intptr_t = signed long;
pub type ptrdiff_t = signed long;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type int_fast16_t = isize;
pub type uint_fast16_t = usize;
pub type int_fast32_t = isize;
pub type uint_fast32_t = usize;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type intmax_t = __INTMAX_TYPE__;
pub type uintmax_t = __UINTMAX_TYPE__;
// limits of integral types

