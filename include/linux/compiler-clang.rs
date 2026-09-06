//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/compiler-clang.h
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

// Compiler specific definitions for Clang compiler
// all clang versions usable with the kernel support KASAN ABI version 5
pub const KASAN_ABI_VERSION: c_int = 5;
//
// Clang 22 added preprocessor macros to match GCC, in hopes of eventually
// dropping __has_feature support for sanitizers:
// https://github.com/llvm/llvm-project/commit/568c23bbd3303518c5056d7f03444dae4fdc8a9c
// Create these macros for older versions of clang so that it is easy to clean
// up once the minimum supported version of LLVM for building the kernel always
// creates these macros.
//
// Note: Checking __has_feature(*_sanitizer) is only true if the feature is
// enabled. Therefore it is not required to additionally check defined(CONFIG_*)
// to avoid adding redundant attributes in other configurations.
//

//
// Treat __SANITIZE_HWADDRESS__ the same as __SANITIZE_ADDRESS__ in the kernel.
//

// Macro flag: #define __no_sanitize_address

// Macro flag: #define __no_sanitize_thread

// GCC does not have __SANITIZE_UNDEFINED__

// Macro flag: #define __no_sanitize_undefined

//
// Unlike other sanitizers, KMSAN still inserts code into functions marked with
// no_sanitize("kernel-memory"). Using disable_sanitizer_instrumentation
// provides the behavior consistent with other __no_sanitize_ attributes,
// guaranteeing that __no_sanitize_memory functions remain uninstrumented.
//

//
// The __no_kmsan_checks attribute ensures that a function does not produce
// false positive reports by:
// - initializing all local variables and memory stores in this function;
// - skipping all shadow checks;
// - passing initialized arguments to this function's callees.
//

// Macro flag: #define __no_sanitize_memory
// Macro flag: #define __no_kmsan_checks

//
// Support for __has_feature(coverage_sanitizer) was added in Clang 13 together
// with no_sanitize("coverage"). Prior versions of Clang support coverage
// instrumentation, but cannot be queried for support by the preprocessor.
//

// Macro flag: #define __no_sanitize_coverage

// Only Clang needs to disable the coverage sanitizer for kstack_erase.

//
// Turn individual warnings and errors on and off locally, depending
// on version.
//

// Severity used in pragma directives

// Macro flag: #define __diag_clang_23(s)

//
// clang has horrible behavior with "g" or "rm" constraints for asm
// inputs, turning them into something worse than "m". Avoid using
// constraints with multiple possible uses (but "ir" seems to be ok):
//
// https://github.com/llvm/llvm-project/issues/20571
//

//
// Declare compiler support for __typeof_unqual__() operator.
//
// Bindgen uses LLVM even if our C compiler is GCC, so we cannot
// rely on the auto-detected CONFIG_CC_HAS_TYPEOF_UNQUAL.
//
