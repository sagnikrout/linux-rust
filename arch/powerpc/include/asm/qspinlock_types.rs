//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/qspinlock_types.h
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
// Bitfields in the lock word:
//
// 0: locked bit
// 1-14: lock holder cpu
// 15: lock owner or queuer vcpus observed to be preempted bit
// 16: must queue bit
// 17-31: tail cpu (+1)
//

// 0x00000001
pub const _Q_LOCKED_OFFSET: c_int = 0;
pub const _Q_LOCKED_BITS: c_int = 1;

// 0x00007ffe
pub const _Q_OWNER_CPU_OFFSET: c_int = 1;
pub const _Q_OWNER_CPU_BITS: c_int = 14;

// 0x00008000
pub const _Q_SLEEPY_OFFSET: c_int = 15;
pub const _Q_SLEEPY_BITS: c_int = 1;

// 0x00010000
pub const _Q_MUST_Q_OFFSET: c_int = 16;
pub const _Q_MUST_Q_BITS: c_int = 1;

// 0xfffe0000
pub const _Q_TAIL_CPU_OFFSET: c_int = 17;
pub const _Q_TAIL_CPU_BITS: c_int = 15;

