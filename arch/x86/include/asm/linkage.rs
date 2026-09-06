//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/linkage.h
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
// The generic version tends to create spurious ENDBR instructions under
// certain conditions.
//

// Macro flag: #define FUNCTION_PADDING

//
// Depending on -fpatchable-function-entry=N,N usage (CONFIG_CALL_PADDING) the
// CFI symbol layout changes.
//
// Without CALL_PADDING:
//
// .align	FUNCTION_ALIGNMENT
// __cfi_##name:
// .skip	FUNCTION_PADDING, 0x90
// .byte   0xb8
// .long	__kcfi_typeid_##name
// name:
//
// With CALL_PADDING:
//
// .align FUNCTION_ALIGNMENT
// __cfi_##name:
// .byte	0xb8
// .long	__kcfi_typeid_##name
// .skip	FUNCTION_PADDING, 0x90
// name:
//
// In both cases the whole thing is FUNCTION_ALIGNMENT aligned and sized.
//

// Macro flag: #define CFI_PRE_PADDING

// Macro flag: #define CFI_POST_PADDING

// UML needs to be able to override memcpy() and friends for KASAN.

// SYM_TYPED_FUNC_START -- use for indirectly called globals, w/ CFI type

// SYM_FUNC_START -- use for global functions

// SYM_FUNC_START_NOALIGN -- use for global functions, w/o alignment

// SYM_FUNC_START_LOCAL -- use for local functions

// SYM_FUNC_START_LOCAL_NOALIGN -- use for local functions, w/o alignment

// SYM_FUNC_START_WEAK -- use for weak functions

// SYM_FUNC_START_WEAK_NOALIGN -- use for weak functions, w/o alignment

//
// Expose 'sym' to the startup code in arch/x86/boot/startup/, by emitting an
// alias prefixed with __pi_
//

