//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/cmpxchg.h
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
// Copyright (C) 2014 Regents of the University of California
//

//
// Atomic compare and exchange.  Compare OLD with MEM, if identical,
// store NEW in MEM.  Return the initial value in MEM.  Success is
// indicated by comparing RETURN with OLD.
//

//
// These macros are here to improve the readability of the arch_cmpxchg_XXX()
// macros.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union __u128_halves {
    pub full: u128,
    pub high: u64 low,,
}

//
// Despite wrs.nto being "WRS-with-no-timeout", in the absence of changes to
// @val we expect it to still terminate within a "reasonable" amount of time
// for an implementation-specific other reason, a pending, locally-enabled
// interrupt, or because it has been configured to raise an illegal
// instruction exception.
//

