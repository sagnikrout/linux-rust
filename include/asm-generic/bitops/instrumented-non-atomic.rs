//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/bitops/instrumented-non-atomic.h
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
// This file provides wrappers with sanitizer instrumentation for non-atomic
// bit operations.
//
// To use this functionality, an arch's bitops.h file needs to define each of
// the below bit operations with an arch_ prefix (e.g. arch_set_bit(),
// arch___set_bit(), etc.).
//

//
// ___set_bit - Set a bit in memory
// @nr: the bit to set
// @addr: the address to start counting from
//
// Unlike set_bit(), this function is non-atomic. If it is called on the same
// region of memory concurrently, the effect may be that only one operation
// succeeds.
//
// ___clear_bit - Clears a bit in memory
// @nr: the bit to clear
// @addr: the address to start counting from
//
// Unlike clear_bit(), this function is non-atomic. If it is called on the same
// region of memory concurrently, the effect may be that only one operation
// succeeds.
//
// ___change_bit - Toggle a bit in memory
// @nr: the bit to change
// @addr: the address to start counting from
//
// Unlike change_bit(), this function is non-atomic. If it is called on the same
// region of memory concurrently, the effect may be that only one operation
// succeeds.
//
// We treat non-atomic read-write bitops a little more special.
// Given the operations here only modify a single bit, assuming
// non-atomicity of the writer is sufficient may be reasonable
// for certain usage (and follows the permissible nature of the
// assume-plain-writes-atomic rule):
// 1. report read-modify-write races -> check read;
// 2. do not report races with marked readers, but do report
// races with unmarked readers -> check "atomic" write.
//
// Use generic write instrumentation, in case other sanitizers
// or tools are enabled alongside KCSAN.
//
// ___test_and_set_bit - Set a bit and return its old value
// @nr: Bit to set
// @addr: Address to count from
//
// This operation is non-atomic. If two instances of this operation race, one
// can appear to succeed but actually fail.
//
extern "C" {
    pub fn arch___test_and_set_bit(_arg: nr, _arg: addr) -> return;
}
//
// ___test_and_clear_bit - Clear a bit and return its old value
// @nr: Bit to clear
// @addr: Address to count from
//
// This operation is non-atomic. If two instances of this operation race, one
// can appear to succeed but actually fail.
//
extern "C" {
    pub fn arch___test_and_clear_bit(_arg: nr, _arg: addr) -> return;
}
//
// ___test_and_change_bit - Change a bit and return its old value
// @nr: Bit to change
// @addr: Address to count from
//
// This operation is non-atomic. If two instances of this operation race, one
// can appear to succeed but actually fail.
//
extern "C" {
    pub fn arch___test_and_change_bit(_arg: nr, _arg: addr) -> return;
}
//
// _test_bit - Determine whether a bit is set
// @nr: bit number to test
// @addr: Address to start counting from
//
extern "C" {
    pub fn arch_test_bit(_arg: nr, _arg: addr) -> return;
}
//
// _test_bit_acquire - Determine, with acquire semantics, whether a bit is set
// @nr: bit number to test
// @addr: Address to start counting from
//
extern "C" {
    pub fn arch_test_bit_acquire(_arg: nr, _arg: addr) -> return;
}
