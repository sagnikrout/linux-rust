//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/permassert.h
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
// Copyright 2023 Red Hat
//

// Utilities for asserting that certain conditions are met

//
// A hack to apply the "warn if unused" attribute to an integral expression.
//
// Since GCC doesn't propagate the warn_unused_result attribute to conditional expressions
// incorporating calls to functions with that attribute, this function can be used to wrap such an
// expression. With optimization enabled, this function contributes no additional instructions, but
// the warn_unused_result attribute still applies to the code calling it.
//
// Assert that an expression is true and return an error if it is not.

// Log a message if the expression is not true.

// Log an assertion failure message.
