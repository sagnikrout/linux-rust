//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/portability_macros.h
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
// This header file contains macro definitions to support portability.
// This header is shared between C and ASM code, so it MUST only
// contain macro definitions. It MUST not contain any C code.
//
// This header ONLY defines macros to detect platforms/feature support.
//
// compat. with non-clang compilers

pub const __has_attribute(x): c_int = 0;

// compat. with non-clang compilers

// compat. with non-clang compilers

// detects whether we are being compiled under msan
// detects whether we are being compiled under asan
// detects whether we are being compiled under dfsan
// Mark the internal assembly functions as hidden

// Compile time determination of BMI2 support
// Enable runtime BMI2 dispatch based on the CPU.
// Enabled for clang & gcc >=4.8 on x86 when BMI2 isn't enabled by default.
//

//
// Only enable assembly for GNU C compatible compilers,
// because other platforms may not support GAS assembly syntax.
//
// Only enable assembly for Linux / MacOS / Win32, other platforms may
// work, but they haven't been tested. This could likely be
// extended to BSD systems.
//
// Disable assembly when MSAN is enabled, because MSAN requires
// 100% of code to be instrumented to work.
//
pub const ZSTD_ASM_SUPPORTED: c_int = 1;
//
// Determines whether we should enable assembly for x86-64
// with BMI2.
//
// Enable if all of the following conditions hold:
// - ASM hasn't been explicitly disabled by defining ZSTD_DISABLE_ASM
// - Assembly is supported
// - We are compiling for x86-64 and either:
// - DYNAMIC_BMI2 is enabled
// - BMI2 is supported at compile time
//
pub const ZSTD_ENABLE_ASM_X86_64_BMI2: c_int = 0;
//
// For x86 ELF targets, add .note.gnu.property section for Intel CET in
// assembly sources when CET is enabled.
//
// Additionally, any function that may be called indirectly must begin
// with ZSTD_CET_ENDBRANCH.
//

