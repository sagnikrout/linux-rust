//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_rtp_helpers.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// Helper macros - not to be used outside this header.
//

//
// XE_RTP_PASTE_FOREACH - Paste XE_RTP_<@prefix_> on each element of the tuple
// @args, with the end result separated by @sep_. @sep must be one of the
// previously declared macros __XE_RTP_PASTE_SEP_*, or declared with such
// prefix.
//
// Examples:
//
// 1) XE_RTP_PASTE_FOREACH(TEST_, COMMA, (FOO, BAR))
// expands to:
//
// XE_RTP_TEST_FOO , XE_RTP_TEST_BAR
//
// 2) XE_RTP_PASTE_FOREACH(TEST2_, COMMA, (FOO))
// expands to:
//
// XE_RTP_TEST2_FOO
//
// 3) XE_RTP_PASTE_FOREACH(TEST3, BITWISE_OR, (FOO, BAR))
// expands to:
//
// XE_RTP_TEST3_FOO | XE_RTP_TEST3_BAR
//
// 4) #define __XE_RTP_PASTE_SEP_MY_SEP	BANANA
// XE_RTP_PASTE_FOREACH(TEST_, MY_SEP, (FOO, BAR))
// expands to:
//
// XE_RTP_TEST_FOO BANANA XE_RTP_TEST_BAR
//

//
// XE_RTP_DROP_CAST - Drop cast to convert a compound statement to a initializer
//
// Example:
//
// #define foo(a_)	((struct foo){ .a = a_ })
// XE_RTP_DROP_CAST(foo(10))
// expands to:
//
// { .a = 10 }
//

