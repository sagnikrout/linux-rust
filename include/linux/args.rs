//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/args.h
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
// How do these macros work?
//
// In __COUNT_ARGS() _0 to _15 are just placeholders from the start
// in order to make sure _n is positioned over the correct number
// from 15 to 0 (depending on X, which is a variadic argument list).
// They serve no purpose other than occupying a position. Since each
// macro parameter must have a distinct identifier, those identifiers
// are as good as any.
//
// In COUNT_ARGS() we use actual integers, so __COUNT_ARGS() returns
// that as _n.
//
// This counts to 15. Any more, it will return 16th argument.

// Concatenate two parameters, but allow them to be expanded beforehand.

