//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/futex.h
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

// oval = oldval;						\
// oval = val;

//
// Wrap LSUI instructions with uaccess_ttbr0_enable()/disable(), as
// PAN toggling is not required.
//

// oval = oldval;						\
// oldval = oval64.futex[futex_pos];
//
// Undo the bitwise negation applied to the oparg passed from
// arch_futex_atomic_op_inuser() with FUTEX_OP_ANDN.
//
extern "C" {
    pub fn __lsui_futex_atomic_andnot(_arg: ~oparg, _arg: uaddr, _arg: oval) -> return;
}
//
// there are no ldteor/stteor instructions...
//
// oval = oldval;
// oval = curval;

extern "C" {
    pub fn __lsui_llsc_body(_arg: futex_cmpxchg, _arg: uaddr, _arg: oldval, _arg: newval, _arg: oval) -> return;
}
extern "C" {
    pub fn __futex_cmpxchg(_arg: uaddr, _arg: oldval, _arg: newval, _arg: uval) -> return;
}
