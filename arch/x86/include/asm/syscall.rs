//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/syscall.h
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
// Access to user system call parameters and results
//
// Copyright (C) 2008-2009 Red Hat, Inc.  All rights reserved.
//
// See asm-generic/syscall.h for descriptions of what we must do here.
//

// This is used purely for kernel/trace/trace_syscalls.c
extern "C" {
    pub fn long(: *const *const sys_call_ptr_t)(struct pt_regs) -> typedef;
}
//
// Only the low 32 bits of orig_ax are meaningful, so we return int.
// This importantly ignores the high bits on 64-bit, so comparisons
// sign-extend the low 32 bits.
//

//
// TS_COMPAT is set for 32-bit syscall entries and then
// remains set until we return to user mode.
//
// Sign-extend the value so (int)-EFOO becomes (long)-EFOO
// and will match correctly in comparisons.
//

// args++ = regs->bx;
// args++ = regs->cx;
// args++ = regs->dx;
// args++ = regs->si;
// args++ = regs->di;
// args   = regs->bp;

// args++ = regs->di;
// args++ = regs->si;
// args++ = regs->dx;
// args++ = regs->r10;
// args++ = regs->r8;
// args   = regs->r9;

// x32 tasks should be considered AUDIT_ARCH_X86_64.
extern "C" {
    pub fn do_syscall_64(regs: *mut pt_regs, nr: c_long) -> bool;
}
extern "C" {
    pub fn do_int80_emulation(regs: *mut pt_regs);
}

extern "C" {
    pub fn do_int80_syscall_32(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_fast_syscall_32(regs: *mut pt_regs) -> bool;
}
extern "C" {
    pub fn do_SYSENTER_32(regs: *mut pt_regs) -> bool;
}
