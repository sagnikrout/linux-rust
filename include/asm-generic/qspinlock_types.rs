//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/qspinlock_types.h
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
// Queued spinlock
//
// (C) Copyright 2013-2015 Hewlett-Packard Development Company, L.P.
//
// Authors: Waiman Long <waiman.long@hp.com>
//

//
// By using the whole 2nd least significant byte for the
// pending bit, we can allow better optimization of the lock
// acquisition for the pending bit holder.
//

//
// Initializier
//

//
// Bitfields in the atomic value:
//
// When NR_CPUS < 16K
// 0- 7: locked byte
// 8-15: pending byte
// 16-17: tail index
// 18-31: tail cpu (+1)
//
// When NR_CPUS >= 16K
// 0- 7: locked byte
// 8: pending
// 9-10: tail index
// 11-31: tail cpu (+1)
//

pub const _Q_LOCKED_OFFSET: c_int = 0;
pub const _Q_LOCKED_BITS: c_int = 8;

pub const _Q_PENDING_BITS: c_int = 8;

pub const _Q_PENDING_BITS: c_int = 1;

pub const _Q_TAIL_IDX_BITS: c_int = 2;

