//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/debug.h
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
// debug
// Part of FSE library
// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// You can contact the author at :
// - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
//
// This source code is licensed under both the BSD-style license (found in the
// LICENSE file in the root directory of this source tree) and the GPLv2 (found
// in the COPYING file in the root directory of this source tree).
// You may select, at your option, one of the above-listed licenses.
//
// The purpose of this header is to enable debug functions.
// They regroup assert(), DEBUGLOG() and RAWLOG() for run-time,
// and DEBUG_STATIC_ASSERT() for compile-time.
//
// By default, DEBUGLEVEL==0, which means run-time debug is disabled.
//
// Level 1 enables assert() only.
// Starting level 2, traces can be generated and pushed to stderr.
// The higher the level, the more verbose the traces.
//
// It's possible to dynamically adjust level using variable g_debug_level,
// which is only declared if DEBUGLEVEL>=2,
// and is a global variable, not multi-thread protected (use with care)
//

// Macro flag: #define DEBUG_H_12987983217
// static assert is triggered at compile time, leaving no runtime artefact.
// static assert only works with compile-time constants.
// Also, this variant can only be used inside a function.

// DEBUGLEVEL is expected to be defined externally,
// typically through compiler command line.
// Value must be a number.

// recommended values for DEBUGLEVEL :
// 0 : release mode, no debug, all run-time checks disabled
// 1 : enables assert() only, no display
// 2 : reserved, for currently active debug path
// 3 : events once per object lifetime (CCtx, CDict, etc.)
// 4 : events once per frame
// 5 : events once per block
// 6 : events once per sequence (verbose)
// 7+: events at every position (*very* verbose)
//
// It's generally inconvenient to output traces > 5.
// In which case, it's possible to selectively trigger high verbosity levels
// by modifying g_debug_level.
//

