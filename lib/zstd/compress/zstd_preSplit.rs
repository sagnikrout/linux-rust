//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/compress/zstd_preSplit.h
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

pub const ZSTD_SLIPBLOCK_WORKSPACESIZE: c_int = 8208;
// ZSTD_splitBlock():
// @level must be a value between 0 and 4.
// higher levels spend more energy to detect block boundaries.
// @workspace must be aligned for size_t.
// @wkspSize must be at least >= ZSTD_SLIPBLOCK_WORKSPACESIZE
// note:
// For the time being, this function only accepts full 128 KB blocks.
// Therefore, @blockSize must be == 128 KB.
// While this could be extended to smaller sizes in the future,
// it is not yet clear if this would be useful. TBD.
//
