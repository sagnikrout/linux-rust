//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/bitops/lock.h
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
// arch_test_and_set_bit_lock - Set a bit and return its old value, for lock
// @nr: Bit to set
// @addr: Address to count from
//
// This operation is atomic and provides acquire barrier semantics if
// the returned value is 0.
// It can be used to implement bit locks.
//
// arch_clear_bit_unlock - Clear a bit in memory, for unlock
// @nr: the bit to set
// @addr: the address to start counting from
//
// This operation is atomic and provides release barrier semantics.
//
// arch___clear_bit_unlock - Clear a bit in memory, for unlock
// @nr: the bit to set
// @addr: the address to start counting from
//
// A weaker form of clear_bit_unlock() as used by __bit_lock_unlock(). If all
// the bits in the word are protected by this lock some archs can use weaker
// ops to safely unlock.
//
// See for example x86's implementation.
//

