//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/bitops/instrumented-atomic.h
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
// This file provides wrappers with sanitizer instrumentation for atomic bit
// operations.
//
// To use this functionality, an arch's bitops.h file needs to define each of
// the below bit operations with an arch_ prefix (e.g. arch_set_bit(),
// arch___set_bit(), etc.).
//

//
// set_bit - Atomically set a bit in memory
// @nr: the bit to set
// @addr: the address to start counting from
//
// This is a relaxed atomic operation (no implied memory barriers).
//
// Note that @nr may be almost arbitrarily large; this function is not
// restricted to acting on a single-word quantity.
//
// clear_bit - Clears a bit in memory
// @nr: Bit to clear
// @addr: Address to start counting from
//
// This is a relaxed atomic operation (no implied memory barriers).
//
// change_bit - Toggle a bit in memory
// @nr: Bit to change
// @addr: Address to start counting from
//
// This is a relaxed atomic operation (no implied memory barriers).
//
// Note that @nr may be almost arbitrarily large; this function is not
// restricted to acting on a single-word quantity.
//
// test_and_set_bit - Set a bit and return its old value
// @nr: Bit to set
// @addr: Address to count from
//
// This is an atomic fully-ordered operation (implied full memory barrier).
//
extern "C" {
    pub fn arch_test_and_set_bit(_arg: nr, _arg: addr) -> return;
}
//
// test_and_clear_bit - Clear a bit and return its old value
// @nr: Bit to clear
// @addr: Address to count from
//
// This is an atomic fully-ordered operation (implied full memory barrier).
//
extern "C" {
    pub fn arch_test_and_clear_bit(_arg: nr, _arg: addr) -> return;
}
//
// test_and_change_bit - Change a bit and return its old value
// @nr: Bit to change
// @addr: Address to count from
//
// This is an atomic fully-ordered operation (implied full memory barrier).
//
extern "C" {
    pub fn arch_test_and_change_bit(_arg: nr, _arg: addr) -> return;
}
