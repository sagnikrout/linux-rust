//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kstack_erase.h
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
// Check that the poison value points to the unused hole in the
// virtual memory map for your platform.
//

pub const KSTACK_ERASE_SEARCH_DEPTH: c_int = 128;

//
// The lowest address on tsk's stack which we can plausibly erase.
//
// The lowest unsigned long on the task stack contains STACK_END_MAGIC,
// which we must not corrupt.
//
// The address immediately after the highest address on tsk's stack which we
// can plausibly erase.
//
// The task's pt_regs lives at the top of the task stack and will be
// overwritten by exception entry, so there's no need to erase them.
//
// Find the address immediately above the poisoned region of the stack, where
// that region falls between 'low' (inclusive) and 'high' (exclusive).
//

extern "C" {
    pub fn stackleak_erase() -> asmlinkage void noinstr;
}
extern "C" {
    pub fn stackleak_erase_on_task_stack() -> asmlinkage void noinstr;
}
extern "C" {
    pub fn stackleak_erase_off_task_stack() -> asmlinkage void noinstr;
}
extern "C" {
    pub fn __sanitizer_cov_stack_depth() -> void __no_caller_saved_registers noinstr;
}

