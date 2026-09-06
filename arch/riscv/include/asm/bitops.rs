//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/bitops.h
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
// Copyright (C) 2012 Regents of the University of California
//

extern "C" {
    pub fn generic___ffs(_arg: word) -> return;
}
//
// __ffs - find first set bit in a long word
// @word: The word to search
//
// Undefined if no set bit exists, so code should check against 0 first.
//

extern "C" {
    pub fn generic___fls(_arg: word) -> return;
}
//
// __fls - find last set bit in a long word
// @word: the word to search
//
// Undefined if no set bit exists, so code should check against 0 first.
//

extern "C" {
    pub fn generic_ffs(_arg: x) -> return;
}
//
// ffs - find first set bit in a word
// @x: the word to search
//
// This is defined the same way as the libc and compiler builtin ffs routines.
//
// ffs(value) returns 0 if value is 0 or the position of the first set bit if
// value is nonzero. The first (least significant) bit is at position 1.
//

extern "C" {
    pub fn generic_fls(_arg: x) -> return;
}
//
// fls - find last set bit in a word
// @x: the word to search
//
// This is defined in a similar way as ffs, but returns the position of the most
// significant set bit.
//
// fls(value) returns 0 if value is 0 or the position of the last set bit if
// value is nonzero. The last (most significant) bit is at position 32.
//

// Bitmask modifiers

//
// arch_test_and_set_bit - Set a bit and return its old value
// @nr: Bit to set
// @addr: Address to count from
//
// This is an atomic fully-ordered operation (implied full memory barrier).
//
extern "C" {
    pub fn __test_and_op_bit(_arg: or, _arg: __NOP, _arg: nr, _arg: addr) -> return;
}
//
// arch_test_and_clear_bit - Clear a bit and return its old value
// @nr: Bit to clear
// @addr: Address to count from
//
// This is an atomic fully-ordered operation (implied full memory barrier).
//
extern "C" {
    pub fn __test_and_op_bit(_arg: and, _arg: __NOT, _arg: nr, _arg: addr) -> return;
}
//
// arch_test_and_change_bit - Change a bit and return its old value
// @nr: Bit to change
// @addr: Address to count from
//
// This operation is atomic and cannot be reordered.
// It also implies a memory barrier.
//
extern "C" {
    pub fn __test_and_op_bit(_arg: xor, _arg: __NOP, _arg: nr, _arg: addr) -> return;
}
//
// arch_set_bit - Atomically set a bit in memory
// @nr: the bit to set
// @addr: the address to start counting from
//
// Note: there are no guarantees that this function will not be reordered
// on non x86 architectures, so if you are writing portable code,
// make sure not to rely on its reordering guarantees.
//
// Note that @nr may be almost arbitrarily large; this function is not
// restricted to acting on a single-word quantity.
//
// arch_clear_bit - Clears a bit in memory
// @nr: Bit to clear
// @addr: Address to start counting from
//
// Note: there are no guarantees that this function will not be reordered
// on non x86 architectures, so if you are writing portable code,
// make sure not to rely on its reordering guarantees.
//
// arch_change_bit - Toggle a bit in memory
// @nr: Bit to change
// @addr: Address to start counting from
//
// change_bit()  may be reordered on other architectures than x86.
// Note that @nr may be almost arbitrarily large; this function is not
// restricted to acting on a single-word quantity.
//
// arch_test_and_set_bit_lock - Set a bit and return its old value, for lock
// @nr: Bit to set
// @addr: Address to count from
//
// This operation is atomic and provides acquire barrier semantics.
// It can be used to implement bit locks.
//
extern "C" {
    pub fn __test_and_op_bit_ord(_arg: or, _arg: __NOP, _arg: nr, _arg: addr, _arg: .aq) -> return;
}
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
// This operation is like clear_bit_unlock, however it is not atomic.
// It does provide release barrier semantics so it can be used to unlock
// a bit lock, however it would only be used if no other CPU can modify
// any bits in the memory until the lock is released (a good example is
// if the bit lock itself protects access to the other bits in the word).
//
// On RISC-V systems there seems to be no benefit to taking advantage of the
// non-atomic property here: it's a lot more instructions and we still have to
// provide release semantics anyway.
//

