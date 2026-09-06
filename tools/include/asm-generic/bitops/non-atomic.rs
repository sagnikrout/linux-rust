//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/asm-generic/bitops/non-atomic.h
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
// ___set_bit - Set a bit in memory
// @nr: the bit to set
// @addr: the address to start counting from
//
// Unlike set_bit(), this function is non-atomic and may be reordered.
// If it's called on the same region of memory simultaneously, the effect
// may be that only one operation succeeds.
//
// p  |= mask;
// p &= ~mask;
//
// ___change_bit - Toggle a bit in memory
// @nr: the bit to change
// @addr: the address to start counting from
//
// Unlike change_bit(), this function is non-atomic and may be reordered.
// If it's called on the same region of memory simultaneously, the effect
// may be that only one operation succeeds.
//
// p ^= mask;
//
// ___test_and_set_bit - Set a bit and return its old value
// @nr: Bit to set
// @addr: Address to count from
//
// This operation is non-atomic and can be reordered.
// If two examples of this operation race, one can appear to succeed
// but actually fail.  You must protect multiple accesses with a lock.
//
// p = old | mask;
//
// ___test_and_clear_bit - Clear a bit and return its old value
// @nr: Bit to clear
// @addr: Address to count from
//
// This operation is non-atomic and can be reordered.
// If two examples of this operation race, one can appear to succeed
// but actually fail.  You must protect multiple accesses with a lock.
//
// p = old & ~mask;
// WARNING: non atomic and it can be reordered!
// p = old ^ mask;
//
// _test_bit - Determine whether a bit is set
// @nr: bit number to test
// @addr: Address to start counting from
//
