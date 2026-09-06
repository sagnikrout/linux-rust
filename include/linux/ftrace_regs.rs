//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ftrace_regs.h
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
// For archs that just copy pt_regs in ftrace regs, it can use this default.
// If an architecture does not use pt_regs, it must define all the below
// accessor functions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __arch_ftrace_regs {
    pub regs: pt_regs,
}

//
// ftrace_partial_regs_update - update the original ftrace_regs from regs
// @fregs: The ftrace_regs to update from @regs
// @regs: The partial regs from ftrace_partial_regs() that was updated
//
// Some architectures have the partial regs living in the ftrace_regs
// structure, whereas other architectures need to make a different copy
// of the @regs. If a partial @regs is retrieved by ftrace_partial_regs() and
// if the code using @regs updates a field (like the instruction pointer or
// stack pointer) it may need to propagate that change to the original @fregs
// it retrieved the partial @regs from. Use this function to guarantee that
// update happens.
//

// This can be overridden by the architectures

