//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/zstd_errors.h
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

// Macro flag: #define ZSTD_ERRORS_H_398273423
// =====   ZSTDERRORLIB_API : control library symbols visibility   =====
// Macro flag: #define ZSTDERRORLIB_VISIBLE

// -
// Error codes list
// -
// Error codes _values_ are pinned down since v1.3.1 only.
// Therefore, don't rely on values if you may link to any version < v1.3.1.
//
// Only values < 100 are considered stable.
//
// note 1 : this API shall be used with static linking only.
// dynamic linking is not yet officially supported.
// note 2 : Prefer relying on the enum than on its value whenever possible
// This is the only supported way to use the error list < v1.3.1
// note 3 : ZSTD_isError() is always correct, whatever the library version.
//
// following error codes are __NOT STABLE__, they can be removed or changed in future versions
