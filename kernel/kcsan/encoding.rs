//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/kcsan/encoding.h
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
// KCSAN watchpoint encoding.
//
// Copyright (C) 2019, Google LLC.
//

pub const INVALID_WATCHPOINT: c_int = 0;
pub const CONSUMED_WATCHPOINT: c_int = 1;
//
// The maximum useful size of accesses for which we set up watchpoints is the
// max range of slots we check on an access.
//

//
// Number of bits we use to store size info.
//

//
// This encoding for addresses discards the upper (1 for is-write + SIZE_BITS);
// however, most 64-bit architectures do not use the full 64-bit address space.
// Also, in order for a false positive to be observable 2 things need to happen:
//
// 1. different addresses but with the same encoded address race;
// 2. and both map onto the same watchpoint slots;
//
// Both these are assumed to be very unlikely. However, in case it still
// happens, the report logic will filter out the false positive (see report.c).
//

// Bitmasks for the encoded watchpoint access information.

//
// While we can encode addrs<PAGE_SIZE, avoid crashing with a NULL
// pointer deref inside KCSAN.
//
// addr_masked =    (unsigned long)watchpoint & WATCHPOINT_ADDR_MASK;
// size	     =   ((unsigned long)watchpoint & WATCHPOINT_SIZE_MASK) >> WATCHPOINT_ADDR_BITS;
// is_write    = !!((unsigned long)watchpoint & WATCHPOINT_WRITE_MASK);
//
// Return watchpoint slot for an address.
//
