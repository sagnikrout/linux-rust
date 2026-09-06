//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/include/linux/linkage.h
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
// linkage.h ... for including arch/x86/lib/memcpy_64.S
// Some toolchains use other characters (e.g. '`') to mark new line in macro

// SYM_T_FUNC -- type used by assembler to mark functions

// SYM_A_* -- align the symbol?

// SYM_L_* -- linkage of symbols

// === generic annotations ===
// SYM_ENTRY -- use only if you have to for non-paired symbols

// SYM_START -- use only if you have to

// SYM_END -- use only if you have to

// SYM_ALIAS -- use only if you have to

// SYM_FUNC_START -- use for global functions

// SYM_FUNC_START_LOCAL -- use for local functions

// SYM_FUNC_START_WEAK -- use for weak functions

//
// SYM_FUNC_END -- the end of SYM_FUNC_START_LOCAL, SYM_FUNC_START,
// SYM_FUNC_START_WEAK, ...
//

//
// SYM_FUNC_ALIAS -- define a global alias for an existing function
//

//
// SYM_FUNC_ALIAS_LOCAL -- define a local alias for an existing function
//

//
// SYM_FUNC_ALIAS_WEAK -- define a weak global alias for an existing function
//

// In the kernel sources (include/linux/cfi_types.h), this has a different
// definition when CONFIG_CFI is used, for tools/ just use the !cfi
// definition:

