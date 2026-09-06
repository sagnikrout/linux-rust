//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/bitops/generic-non-atomic.h
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
// Generic definitions for bit operations, should not be used in regular code
// directly.
//
// generic___set_bit - Set a bit in memory
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
// generic___change_bit - Toggle a bit in memory
// @nr: the bit to change
// @addr: the address to start counting from
//
// Unlike change_bit(), this function is non-atomic and may be reordered.
// If it's called on the same region of memory simultaneously, the effect
// may be that only one operation succeeds.
//
// p ^= mask;
//
// generic___test_and_set_bit - Set a bit and return its old value
// @nr: Bit to set
// @addr: Address to count from
//
// This operation is non-atomic and can be reordered.
// If two examples of this operation race, one can appear to succeed
// but actually fail.  You must protect multiple accesses with a lock.
//
// p = old | mask;
//
// generic___test_and_clear_bit - Clear a bit and return its old value
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
// generic_test_bit - Determine whether a bit is set
// @nr: bit number to test
// @addr: Address to start counting from
//
// Unlike the bitops with the '__' prefix above, this one *is* atomic,
// so `volatile` must always stay here with no cast-aways. See
// `Documentation/atomic_bitops.txt` for the details.
//
// generic_test_bit_acquire - Determine, with acquire semantics, whether a bit is set
// @nr: bit number to test
// @addr: Address to start counting from
//
// const_*() definitions provide good compile-time optimizations when
// the passed arguments can be resolved at compile time.
//

//
// const_test_bit - Determine whether a bit is set
// @nr: bit number to test
// @addr: Address to start counting from
//
// A version of generic_test_bit() which discards the `volatile` qualifier to
// allow a compiler to optimize code harder. Non-atomic and to be called only
// for testing compile-time constants, e.g. by the corresponding macros, not
// directly from "regular" code.
//
