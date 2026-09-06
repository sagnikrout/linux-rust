//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/x86/include/asm/asm.h
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

// 32 bit

// 64 bit

// 32 bit

// 64 bit

// Exception table entry

// For C file, we already have NOKPROBE_SYMBOL macro
//
// This output constraint should be used for any inline asm which has a "call"
// instruction.  Otherwise the asm may be inserted before the frame pointer
// gets set up by the containing function.  If you forget to do this, objtool
// may print a "call without frame pointer save/setup" warning.
//
extern "C" {
    pub fn asm(_arg: _ASM_SP) -> register unsigned long current_stack_pointer;
}

