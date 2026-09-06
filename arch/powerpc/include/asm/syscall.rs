//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/syscall.h
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
// Copyright (C) 2008 Red Hat, Inc.  All rights reserved.
//
// See asm-generic/syscall.h for descriptions of what we must do here.
//
pub const _ASM_SYSCALL_H: c_int = 1;

extern "C" {
    pub fn long(: *const *const syscall_fn)(struct pt_regs) -> typedef;
}

// ftrace syscalls requires exporting the sys_call_table
//
// Note that we are returning an int here. That means 0xffffffff, ie.
// 32-bit negative 1, will be interpreted as -1 on a 64-bit kernel.
// This is important for seccomp so that compat tasks can set r0 = -1
// to reject the syscall.
//
// Unlike syscall_get_nr(), syscall_set_nr() can be called only when
// the target task is stopped for tracing on entering syscall, so
// there is no need to have the same check syscall_get_nr() has.
//
// If the system call failed,
// regs->gpr[3] contains a positive ERRORCODE.
//
// In the general case it's not obvious that we must deal with
// CCR here, as the syscall exit path will also do that for us.
// However there are some places, eg. the signal code, which
// check ccr to decide if the value in r3 is actually an error.
//
// Mark that a return value has been explicitly set by seccomp or
// ptrace so that system_call_exception() can skip the syscall
// unconditionally, even when the user requested syscall(-1).
//
// Also copy the first argument into orig_gpr3
