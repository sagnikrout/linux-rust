//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/stackprotector.h
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
// GCC stack protector support.
//
// Stack protector works by putting a predefined pattern at the start of
// the stack frame and verifying that it hasn't been overwritten when
// returning from the function.  The pattern is called the stack canary
// and is a unique value for each task.
//
pub const _ASM_STACKPROTECTOR_H: c_int = 1;

//
// Initialize the stackprotector canary value.
//
// NOTE: this must only be called from functions that never return
// and it must always be inlined.
//
// In addition, it should be called from a compilation unit for which
// stack protector is disabled. Alternatively, the caller should not end
// with a function call which gets tail-call optimized as that would
// lead to checking a modified canary value.
//

// dummy boot_init_stack_canary() is defined in linux/stackprotector.h

