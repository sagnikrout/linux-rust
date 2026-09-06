//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/atomic_ll_sc.h
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
// Based on arch/arm/include/asm/atomic.h
//
// Copyright (C) 1996 Russell King.
// Copyright (C) 2002 Deep Blue Solutions Ltd.
// Copyright (C) 2012 ARM Ltd.
//

// Macro flag: #define K

//
// AArch64 UP and SMP safe atomic ops.  We use load exclusive and
// store exclusive to ensure that these are atomic.  We may loop
// to ensure that the update happens.
//

//
// GAS converts the mysterious and undocumented BIC (immediate) alias to
// an AND (immediate) instruction with the immediate inverted. We don't
// have a constraint for this, so fall back to register.
//

//
// GAS converts the mysterious and undocumented BIC (immediate) alias to
// an AND (immediate) instruction with the immediate inverted. We don't
// have a constraint for this, so fall back to register.
//

// \
// Sub-word sizes require explicit casting so that the compare  \
// part of the cmpxchg doesn't end up interpreting non-zero	\
// upper bits of the register containing "old".			\
// \
//
// Earlier versions of GCC (no later than 8.1.0) appear to incorrectly
// handle the 'K' constraint for the value 4294967295 - thus we use no
// constraint for 32 bit operations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union __u128_halves {
    pub full: u128,
    pub high: u64 low,,
}

