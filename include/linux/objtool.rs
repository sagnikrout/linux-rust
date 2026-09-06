//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/objtool.h
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

// struct unwind_hint */				\
//
// This macro marks the given function's stack frame as "non-standard", which
// tells objtool to ignore the function when doing stack metadata validation.
// It should only be used in special cases where you're 100% sure it won't
// affect the reliability of frame pointers and kernel stack traces.
//
// For more information, see tools/objtool/Documentation/objtool.txt.
//

// __func_stack_frame_non_standard_##func = func
//
// STACK_FRAME_NON_STANDARD_FP() is a frame-pointer-specific function ignore
// for the case where a function is intentionally missing frame pointer setup,
// but otherwise needs objtool/ORC coverage when frame pointers are disabled.
//

// Macro flag: #define STACK_FRAME_NON_STANDARD_FP(func)

//
// In asm, there are two kinds of code: normal C-type callable functions and
// the rest.  The normal callable functions can be called by other code, and
// don't do anything unusual with the stack.  Such normal callable functions
// are annotated with SYM_FUNC_{START,END}.  Most asm code falls in this
// category.  In this case, no special debugging annotations are needed because
// objtool can automatically generate the ORC data for the ORC unwinder to read
// at runtime.
//
// Anything which doesn't fall into the above category, such as syscall and
// interrupt handlers, tends to not be called directly by other functions, and
// often does unusual non-C-function-type things with the stack pointer.  Such
// code needs to be annotated such that objtool can understand it.  The
// following CFI hint macros are for this type of code.
//
// These macros provide hints to objtool about the state of the stack at each
// instruction.  Objtool starts from the hints and follows the code flow,
// making automatic CFI adjustments when it sees pushes and pops, filling out
// the debuginfo as necessary.  It will also warn if it sees any
// inconsistencies.
//
// struct unwind_hint

// Macro flag: #define STACK_FRAME_NON_STANDARD(func)
// Macro flag: #define STACK_FRAME_NON_STANDARD_FP(func)

// Macro flag: #define VALIDATE_UNRET_BEGIN

