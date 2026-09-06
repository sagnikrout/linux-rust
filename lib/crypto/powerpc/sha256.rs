//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/powerpc/sha256.h
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
//
// SHA-256 Secure Hash Algorithm, SPE optimized
//
// Based on generic implementation. The assembler module takes care
// about the SPE registers so it can run from interrupt context.
//
// Copyright (c) 2015 Markus Stockhausen <stockhausen@collogia.de>
//

//
// MAX_BYTES defines the number of bytes that are allowed to be processed
// between preempt_disable() and preempt_enable(). SHA256 takes ~2,000
// operations per 64 bytes. e500 cores can issue two arithmetic instructions
// per clock cycle using one 32/64 bit unit (SU1) and one 32 bit unit (SU2).
// Thus 1KB of input data will need an estimated maximum of 18,000 cycles.
// Headroom for cache misses included. Even with the low end model clocked
// at 667 MHz this equals to a critical time window of less than 27us.
//
pub const MAX_BYTES: c_int = 1024;
// We just start SPE operations and will save SPE registers later.
// reenable preemption
// cut input data into smaller blocks
