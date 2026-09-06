//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/regs.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// -----------------------------------------------------------------------
//
// Copyright 2009 Intel Corporation; author H. Peter Anvin
//
// -----------------------------------------------------------------------
//
// Simple helper function for initializing a register set.
//
// Note that this sets EFLAGS_CF in the input register set; this
// makes it easier to catch functions which do nothing but don't
// explicitly set CF.
//

#[no_mangle]
pub unsafe extern "C" fn initregs(reg: *mut biosregs) {
    void initregs(struct biosregs *reg)
    {
    memset(reg, 0, sizeof(*reg));
    reg.eflags |= X86_EFLAGS_CF;
    reg.ds = ds();
    reg.es = ds();
    reg.fs = fs();
    reg.gs = gs();
    }
