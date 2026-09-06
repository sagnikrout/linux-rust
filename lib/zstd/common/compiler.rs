//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zstd/common/compiler.h
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
// Compiler specifics
//
// force inlining

// Macro flag: #define INLINE_KEYWORD
// Macro flag: #define FORCE_INLINE_ATTR

//
// Macro flag: #define WIN_CDECL
// UNUSED_ATTR tells the compiler it is okay if the function is unused.

//
// FORCE_INLINE_TEMPLATE is used to define C "templates", which take constant
// parameters. They must be inlined for the compiler to eliminate the constant
// branches.
//

//
// HINT_INLINE is used to help the compiler generate better code. It is *not
// used for "templates", so it can be tweaked based on the compilers
// performance.
//
// gcc-4.8 and gcc-4.9 have been shown to benefit from leaving off the
// always_inline attribute.
//
// clang up to 5.0.0 (trunk) benefit tremendously from the always_inline
// attribute.
//

// "soft" inline :
// The compiler is free to select if it's a good idea to inline or not.
// The main objective is to silence compiler warnings
// when a defined function in included but not used.
//
// Note : this macro is prefixed `MEM_` because it used to be provided by `mem.h` unit.
// Updating the prefix is probably preferable, but requires a fairly large codemod,
// since this name is used everywhere.
//

// force no inlining

// target attribute

// Target attribute for BMI2 dynamic dispatch.
// Enable lzcnt, bmi, and bmi2.
// We test for bmi1 & bmi2. lzcnt is included in bmi1.
//

// prefetch
// can be disabled, by declaring NO_PREFETCH build macro

pub const CACHELINE_SIZE: c_int = 64;

// vectorization
// older GCC (pre gcc-4.3 picked as the cutoff) uses a different syntax,
// and some compilers, like Intel ICC and MCST LCC, do not support it at all.

// Tell the compiler that a branch is likely or unlikely.
// Only use these macros if it causes the compiler to generate better code.
// If you can remove a LIKELY/UNLIKELY annotation without speed changes in gcc
// and clang, please do.
//

// disable warnings
// compile time determination of SIMD support
// C-language Attributes are added in C23.

// Only use C++ attributes in C++. Some compilers report support for C++
// attributes when compiling with C.
//
pub const ZSTD_HAS_CPP_ATTRIBUTE(x): c_int = 0;
// Define ZSTD_FALLTHROUGH macro for annotating switch case with the 'fallthrough' attribute.
// - C23: https://en.cppreference.com/w/c/language/attributes/fallthrough
// - CPP17: https://en.cppreference.com/w/cpp/language/attributes/fallthrough
// - Else: __attribute__((__fallthrough__))
//

// -
// Alignment
//
// @return 1 if @u is a 2^n value, 0 otherwise
// useful to check a value is valid for alignment restrictions
// this test was initially positioned in mem.h,
// but this file is removed (or replaced) for linux kernel
// so it's now hosted in compiler.h,
// which remains valid for both user & kernel spaces.
//

// covers gcc, clang & MSVC
// note : this section must come first, before C11,
// due to a limitation in the kernel source generator

// C90-compatible alignment macro (GCC/Clang). Adjust for other compilers if needed.

// -
// Sanitizer
//
// Zstd relies on pointer overflow in its decompressor.
// We add this attribute to functions that rely on pointer overflow.
//

// gcc < 8 only has signed-integer-overlow which triggers on pointer overflow

// older versions of clang [3.7, 5.0) will warn that pointer-overflow is ignored.

//
// Helper function to perform a wrapped pointer difference without triggering
// UBSAN.
//
// @returns lhs - rhs with wrapping
//
// Helper function to perform a wrapped pointer add without triggering UBSAN.
//
// @return ptr + add with wrapping
//
// Helper function to perform a wrapped pointer subtraction without triggering
// UBSAN.
//
// @return ptr - sub with wrapping
//
// Helper function to add to a pointer that works around C's undefined behavior
// of adding 0 to NULL.
//
// @returns `ptr + add` except it defines `NULL + 0 == NULL`.
//
// Issue #3240 reports an ASAN failure on an llvm-mingw build. Out of an
// abundance of caution, disable our custom poisoning on mingw.

pub const ZSTD_ASAN_DONT_POISON_WORKSPACE: c_int = 1;

pub const ZSTD_MSAN_DONT_POISON_WORKSPACE: c_int = 1;

