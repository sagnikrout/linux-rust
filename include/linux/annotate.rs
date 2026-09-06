//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/annotate.h
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

// Macro flag: #define ASM_ANNOTATE(type)
// Macro flag: #define ASM_ANNOTATE_DATA(type)

//
// Annotate away the various 'relocation to !ENDBR` complaints; knowing that
// these relocations will never be used for indirect calls.
//

//
// This should be used immediately before an indirect jump/call. It tells
// objtool the subsequent indirect jump/call is vouched safe for retpoline
// builds.
//

//
// See linux/instrumentation.h
//

//
// objtool annotation to ignore the alternatives and only consider the original
// instruction(s).
//

//
// This macro indicates that the following intra-function call is valid.
// Any non-annotated intra-function call will cause objtool to issue a warning.
//

//
// Use objtool to validate the entry requirement that all code paths do
// VALIDATE_UNRET_END before RET.
//
// NOTE: The macro must be used at the beginning of a global symbol, otherwise
// it will be ignored.
//

//
// This should be used to refer to an instruction that is considered
// terminating, like a noreturn CALL or UD2 when we know they are not -- eg
// WARN using UD2.
//

//
// This should not be used; it annotates away CFI violations. There are a few
// valid use cases like kexec handover to the next kernel image, and there is
// no security concern there.
//
// There are also a few real issues annotated away, like EFI because we can't
// control the EFI code.
//

//
// Annotate a special section entry.  This emables livepatch module generation
// to find and extract individual special section entries as needed.
//

// ANNOTATE_INSTR_BEGIN		ANNOTATE type=ANNOTYPE_INSTR_BEGIN
// ANNOTATE_INSTR_END		ANNOTATE type=ANNOTYPE_INSTR_END

