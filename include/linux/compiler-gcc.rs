//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/compiler-gcc.h
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

//
// Common definitions for all gcc versions go here.
//

//
// This macro obfuscates arithmetic on a variable address so that gcc
// shouldn't recognize the original var, and make assumptions about it.
//
// This is needed because the C standard makes it undefined to do
// pointer arithmetic on "objects" outside their boundaries and the
// gcc optimizers assume this is the case. In particular they
// assume such arithmetic does not wrap.
//
// A miscompilation has been observed because of this on PPC.
// To work around it we hide the relationship of the pointer and the object
// using this macro.
//
// Versions of the ppc64 compiler before 4.1 had a bug where use of
// RELOC_HIDE could trash r30. The bug can be worked around by changing
// the inline assembly constraint from =g to =r, in this particular
// case either is valid.
//

//
// calling noreturn functions, __builtin_unreachable() and __builtin_trap()
// confuse the stack allocation in gcc, leading to overly large stack
// frames, see https://gcc.gnu.org/bugzilla/show_bug.cgi?id=82365
//
// Adding an empty inline assembly before it works around the problem
//

pub const KASAN_ABI_VERSION: c_int = 5;

pub const KASAN_ABI_VERSION: c_int = 4;

// Macro flag: #define __no_sanitize_thread

//
// Only supported since gcc >= 12
//

// Macro flag: #define __no_sanitize_coverage

//
// Treat __SANITIZE_HWADDRESS__ the same as __SANITIZE_ADDRESS__ in the kernel,
// matching the defines used by Clang.
//

//
// GCC does not support KMSAN.
//
// Macro flag: #define __no_sanitize_memory
// Macro flag: #define __no_kmsan_checks
//
// Turn individual warnings and errors on and off locally, depending
// on version.
//

// Severity used in pragma directives

// Macro flag: #define __diag_GCC_8(s)

//
// Prior to 9.1, -Wno-alloc-size-larger-than (and therefore the "alloc_size"
// attribute) do not work, and must be disabled.
//

//
// Declare compiler support for __typeof_unqual__() operator.
//
// Bindgen uses LLVM even if our C compiler is GCC, so we cannot
// rely on the auto-detected CONFIG_CC_HAS_TYPEOF_UNQUAL.
//
