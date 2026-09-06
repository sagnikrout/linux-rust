//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/error_private.h
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


// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//
// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//
// Note : this module is expected to remain private, do not expose it

// Macro flag: #define ERROR_H_MODULE
//
// Dependencies
//

//
// Compiler-specific
//

// -
// Customization (error_public.h)
//
pub type ERR_enum = ZSTD_ErrorCode;

// -
// Error codes handling
//

// check and forward error code

// -
// Error Strings
//
extern "C" {
    pub fn ERR_getErrorString(_arg: ERR_getErrorCode(code)) -> return;
}
//
// Ignore: this is an internal helper.
//
// This is a helper function to help force C99-correctness during compilation.
// Under strict compilation modes, variadic macro arguments can't be empty.
// However, variadic function arguments can be. Using a function therefore lets
// us statically check that at least one (string) argument was passed,
// independent of the compilation flags.
//
// Ignore: this is an internal helper.
//
// We want to force this function invocation to be syntactically correct, but
// we don't want to force runtime evaluation of its arguments.
//

//
// Return the specified error if the condition evaluates to true.
//
// In debug modes, prints additional information.
// In order to do that (particularly, printing the conditional that failed),
// this can't just wrap RETURN_ERROR().
//

//
// Unconditionally return the specified error.
//
// In debug modes, prints additional information.
//

//
// If the provided expression evaluates to an error code, returns that error code.
//
// In debug modes, prints additional information.
//

