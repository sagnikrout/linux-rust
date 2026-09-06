//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/export.h
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


// SPDX-License-Identifier: GPL-2.0-only

//
// This comment block is used by fixdep. Please do not remove.
//
// When CONFIG_MODVERSIONS is changed from n to y, all source files having
// EXPORT_SYMBOL variants must be re-compiled because genksyms is run as a
// side effect of the *.o build rule.
//

//
// LLVM integrated assembler cam merge adjacent string literals (like
// C and GNU-as) passed to '.ascii', but not to '.asciz' and chokes on:
//
// .asciz "MODULE_" "kvm" ;
//

//
// Allow symbol exports to be disabled completely so that C code may
// be reused in other execution contexts such as the UEFI stub or the
// decompressor.
//

//
// With CONFIG_GENDWARFKSYMS, ensure the compiler emits debugging
// information for all exported symbols, including those defined in
// different TUs, by adding a __gendwarfksyms_ptr_<symbol> pointer
// that's discarded during the final link.
//

// Macro flag: #define __GENDWARFKSYMS_EXPORT(sym)

