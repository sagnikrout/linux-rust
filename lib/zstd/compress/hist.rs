//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/compress/hist.h
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
// hist : Histogram functions
// part of Finite State Entropy project
// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// You can contact the author at :
// - FSE source repository : https://github.com/Cyan4973/FiniteStateEntropy
// - Public forum : https://groups.google.com/forum/#!forum/lz4c
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//
// --- dependencies ---

// --- simple histogram functions ---
// ! HIST_count():
// Provides the precise count of each byte within a table 'count'.
// 'count' is a table of unsigned int, of minimum size (*maxSymbolValuePtr+1).
// Updates *maxSymbolValuePtr with actual largest symbol value detected.
// @return : count of the most frequent symbol (which isn't identified).
// or an error code, which can be tested using HIST_isError().
// note : if return == srcSize, there is only one symbol.
//
// --- advanced histogram functions ---
pub const HIST_WKSP_SIZE_U32: c_int = 1024;

// HIST_count_wksp() :
// Same as HIST_count(), but using an externally provided scratch buffer.
// Benefit is this function will use very little stack space.
// `workSpace` is a writable buffer which must be 4-bytes aligned,
// `workSpaceSize` must be >= HIST_WKSP_SIZE
//
// HIST_countFast() :
// same as HIST_count(), but blindly trusts that all byte values within src are <= *maxSymbolValuePtr.
// This function is unsafe, and will segfault if any value within `src` is `> *maxSymbolValuePtr`
//
// HIST_countFast_wksp() :
// Same as HIST_countFast(), but using an externally provided scratch buffer.
// `workSpace` is a writable buffer which must be 4-bytes aligned,
// `workSpaceSize` must be >= HIST_WKSP_SIZE
//
// ! HIST_count_simple() :
// Same as HIST_countFast(), this function is unsafe,
// and will segfault if any value within `src` is `> *maxSymbolValuePtr`.
// It is also a bit slower for large inputs.
// However, it does not need any additional memory (not even on stack).
// @return : count of the most frequent symbol.
// Note this function doesn't produce any error (i.e. it must succeed).
//
// ! HIST_add() :
// Lowest level: just add nb of occurrences of characters from @src into @count.
// @count is not reset. @count array is presumed large enough (i.e. 1 KB).
//
extern "C" {
    pub fn HIST_add(count: *mut *mut unsigned, src: *const *const c_void, srcSize: usize);
}
