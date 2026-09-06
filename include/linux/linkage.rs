//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/linkage.h
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

// Some toolchains use other characters (e.g. '`') to mark new line in macro

// Macro flag: #define CPP_ASMLINKAGE

//
// For assembly routines.
//
// Note when using these that you must specify the appropriate
// alignment directives yourself
//

//
// This is used by architectures to keep arguments on the stack
// untouched by the compiler by keeping them live until the end.
// The argument stack may be owned by the assembly-language
// caller, not the callee, and gcc doesn't always understand
// that.
//
// We have the return value, and a maximum of six arguments.
//
// This should always be followed by a "return ret" for the
// protection to work (ie no more work that the compiler might
// end up needing stack temporaries for).
//
// Assembly files may be compiled with -traditional ..

// SYM_T_FUNC -- type used by assembler to mark functions

// SYM_T_OBJECT -- type used by assembler to mark data

// SYM_T_NONE -- type used by assembler to mark entries of unknown type

// SYM_A_* -- align the symbol?

// SYM_L_* -- linkage of symbols

// === DEPRECATED annotations ===

// deprecated, use SYM_DATA*, SYM_ENTRY, or similar

// deprecated, use SYM_FUNC_START

// deprecated, use SYM_FUNC_START_WEAK*

// deprecated, use SYM_FUNC_END, SYM_DATA_END, or SYM_END

// deprecated, use SYM_FUNC_END

// === generic annotations ===
// SYM_ENTRY -- use only if you have to for non-paired symbols

// SYM_START -- use only if you have to

// SYM_END -- use only if you have to

// SYM_ALIAS -- use only if you have to

// === code annotations ===
//
// FUNC -- C-like functions (proper stack frame etc.)
// CODE -- non-C code (e.g. irq handlers with different, special stack etc.)
//
// Objtool validates stack for FUNC, but not for CODE.
// Objtool generates debug info for both FUNC & CODE, but needs special
// annotations for each CODE's start (to describe the actual stack frame).
//
// Objtool requires that all code must be contained in an ELF symbol. Symbol
// names that have a  .L prefix do not emit symbol table entries. .L
// prefixed symbols can be used within a code region, but should be avoided for
// denoting a range of code via ``SYM_*_START/END`` annotations.
//
// ALIAS -- does not generate debug info -- the aliased function will
//
// SYM_INNER_LABEL_ALIGN -- only for labels in the middle of code

// SYM_INNER_LABEL -- only for labels in the middle of code

// SYM_FUNC_START -- use for global functions

// SYM_FUNC_START_NOALIGN -- use for global functions, w/o alignment

// SYM_FUNC_START_LOCAL -- use for local functions

// SYM_FUNC_START_LOCAL_NOALIGN -- use for local functions, w/o alignment

// SYM_FUNC_START_WEAK -- use for weak functions

// SYM_FUNC_START_WEAK_NOALIGN -- use for weak functions, w/o alignment

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

// SYM_CODE_START -- use for non-C (special) functions

// SYM_CODE_START_NOALIGN -- use for non-C (special) functions, w/o alignment

// SYM_CODE_START_LOCAL -- use for local non-C (special) functions

//
// SYM_CODE_START_LOCAL_NOALIGN -- use for local non-C (special) functions,
// w/o alignment
//

// SYM_CODE_END -- the end of SYM_CODE_START_LOCAL, SYM_CODE_START, ...

// === data annotations ===
// SYM_DATA_START -- global data symbol

// SYM_DATA_START -- local data symbol

// SYM_DATA_END -- the end of SYM_DATA_START symbol

// SYM_DATA_END_LABEL -- the labeled end of SYM_DATA_START symbol

// SYM_DATA -- start+end wrapper around simple global data

// SYM_DATA_LOCAL -- start+end wrapper around simple local data

