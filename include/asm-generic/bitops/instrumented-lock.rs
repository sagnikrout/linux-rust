//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/bitops/instrumented-lock.h
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
// This file provides wrappers with sanitizer instrumentation for bit
// locking operations.
//
// To use this functionality, an arch's bitops.h file needs to define each of
// the below bit operations with an arch_ prefix (e.g. arch_set_bit(),
// arch___set_bit(), etc.).
//

//
// clear_bit_unlock - Clear a bit in memory, for unlock
// @nr: the bit to set
// @addr: the address to start counting from
//
// This operation is atomic and provides release barrier semantics.
//
// __clear_bit_unlock - Clears a bit in memory
// @nr: Bit to clear
// @addr: Address to start counting from
//
// This is a non-atomic operation but implies a release barrier before the
// memory operation. It can be used for an unlock if no other CPUs can
// concurrently modify other bits in the word.
//
// test_and_set_bit_lock - Set a bit and return its old value, for lock
// @nr: Bit to set
// @addr: Address to count from
//
// This operation is atomic and provides acquire barrier semantics if
// the returned value is 0.
// It can be used to implement bit locks.
//
extern "C" {
    pub fn arch_test_and_set_bit_lock(_arg: nr, _arg: addr) -> return;
}
//
// xor_unlock_is_negative_byte - XOR a single byte in memory and test if
// it is negative, for unlock.
// @mask: Change the bits which are set in this mask.
// @addr: The address of the word containing the byte to change.
//
// Changes some of bits 0-6 in the word pointed to by @addr.
// This operation is atomic and provides release barrier semantics.
// Used to optimise some folio operations which are commonly paired
// with an unlock or end of writeback.  Bit 7 is used as PG_waiters to
// indicate whether anybody is waiting for the unlock.
//
// Return: Whether the top bit of the byte is set.
//
extern "C" {
    pub fn arch_xor_unlock_is_negative_byte(_arg: mask, _arg: addr) -> return;
}
