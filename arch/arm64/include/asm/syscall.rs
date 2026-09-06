//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/syscall.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 ARM Ltd.
//

extern "C" {
    pub fn long(regs: *const *const syscall_fn_t)(struct pt_regs) -> typedef;
}

//
// When the syscall number is set to -1, the syscall will be
// skipped.  In this case the syscall return value has to be
// set explicitly, otherwise the first syscall argument is
// returned as the syscall return value.
//
// Also copy the first argument into orig_x0
// so that syscall_get_arguments() would return it
// instead of the previous value.
//
// We don't care about endianness (__AUDIT_ARCH_LE bit) here because
// AArch64 has the same system calls both on little- and big- endian.
//
extern "C" {
    pub fn syscall_trace_enter(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn syscall_trace_exit(regs: *mut pt_regs);
}
