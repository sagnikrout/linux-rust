//! Automatically rewritten from C to Rust
//! Source: lib/zstd/common/zstd_common.c
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
// -
// Dependencies
//
// Macro flag: #define ZSTD_DEPS_NEED_MALLOC

// -
// Version
//
    unsigned ZSTD_versionNumber(void) { return ZSTD_VERSION_NUMBER; }
    const char* ZSTD_versionString(void) { return ZSTD_VERSION_STRING; }
// -
// ZSTD Error Management
//

// ! ZSTD_isError() :
// tells if a return value is an error code
// symbol is required for external callers
    unsigned ZSTD_isError(size_t code) { return ERR_isError(code); }
// ! ZSTD_getErrorName() :
// provides error code string from function result (useful for debugging)
    const char* ZSTD_getErrorName(size_t code) { return ERR_getErrorName(code); }
// ! ZSTD_getError() :
// convert a `size_t` function result into a proper ZSTD_errorCode enum
    ZSTD_ErrorCode ZSTD_getErrorCode(size_t code) { return ERR_getErrorCode(code); }
// ! ZSTD_getErrorString() :
// provides error code string from enum
    const char* ZSTD_getErrorString(ZSTD_ErrorCode code) { return ERR_getErrorString(code); }
