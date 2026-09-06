//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/syscall.h
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
// Copyright (C) 2008-2009 Red Hat, Inc.  All rights reserved.
// Copyright 2010 Tilera Corporation. All Rights Reserved.
// Copyright 2015 Regents of the University of California, Berkeley
//
// See asm-generic/syscall.h for descriptions of what we must do here.
//

// The array of function pointers for syscalls.
//
// Only the low 32 bits of orig_a0 are meaningful, so we return int.
// This importantly ignores the high bits on 64-bit, so comparisons
// sign-extend the low 32 bits.
//

extern "C" {
    pub fn long(: *const *const syscall_t)(struct pt_regs) -> typedef;
}

extern "C" {
    pub fn sys_riscv_flush_icache(_arg: uintptr_t, _arg: uintptr_t, _arg: uintptr_t) -> asmlinkage long;
}
