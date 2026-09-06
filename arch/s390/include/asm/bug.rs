//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/bug.h
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
// Variable Argument List (va_list) as defined in ELF Application
// Binary Interface s390x Supplement documentation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_va_list {
    pub __gpr: c_long,
    pub __fpr: c_long,
    pub __overflow_arg_area: *mut c_void,
    pub __reg_save_area: *mut c_void,
}

extern "C" {
    pub fn __WARN_trap(bug: *mut bug_entry, ...);
}

// prevent tail-call optimization */				\

// Macro flag: #define HAVE_ARCH_BUG
// Macro flag: #define HAVE_ARCH_BUG_FORMAT
// Macro flag: #define HAVE_ARCH_BUG_FORMAT_ARGS

// Macro flag: #define ARCH_WARN_REACHABLE

