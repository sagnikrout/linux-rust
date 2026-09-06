//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/sync_bitops.h
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
// Copyright 1992, Linus Torvalds.
//
// These have to be done with inline assembly: that way the bit-setting
// is guaranteed to be atomic. All bit operations return 0 if the bit
// was cleared before the operation and != 0 if it was not.
//
// bit 0 is the LSB of addr; bit 32 is the LSB of (addr+1).
//

//
// sync_set_bit - Atomically set a bit in memory
// @nr: the bit to set
// @addr: the address to start counting from
//
// This function is atomic and may not be reordered.  See __set_bit()
// if you do not require the atomic guarantees.
//
// Note that @nr may be almost arbitrarily large; this function is not
// restricted to acting on a single-word quantity.
//
// sync_clear_bit - Clears a bit in memory
// @nr: Bit to clear
// @addr: Address to start counting from
//
// sync_clear_bit() is atomic and may not be reordered.  However, it does
// not contain a memory barrier, so if it is used for locking purposes,
// you should call smp_mb__before_atomic() and/or smp_mb__after_atomic()
// in order to ensure changes are visible on other processors.
//
// sync_change_bit - Toggle a bit in memory
// @nr: Bit to change
// @addr: Address to start counting from
//
// sync_change_bit() is atomic and may not be reordered.
// Note that @nr may be almost arbitrarily large; this function is not
// restricted to acting on a single-word quantity.
//
// sync_test_and_set_bit - Set a bit and return its old value
// @nr: Bit to set
// @addr: Address to count from
//
// This operation is atomic and cannot be reordered.
// It also implies a memory barrier.
//
extern "C" {
    pub fn GEN_BINARY_RMWcc(__ASM_SIZE(bts): "lock ", _arg: *mut addr, _arg: c, _arg: "Ir", _arg: nr) -> return;
}
//
// sync_test_and_clear_bit - Clear a bit and return its old value
// @nr: Bit to clear
// @addr: Address to count from
//
// This operation is atomic and cannot be reordered.
// It also implies a memory barrier.
//
extern "C" {
    pub fn GEN_BINARY_RMWcc(__ASM_SIZE(btr): "lock ", _arg: *mut addr, _arg: c, _arg: "Ir", _arg: nr) -> return;
}
//
// sync_test_and_change_bit - Change a bit and return its old value
// @nr: Bit to change
// @addr: Address to count from
//
// This operation is atomic and cannot be reordered.
// It also implies a memory barrier.
//
extern "C" {
    pub fn GEN_BINARY_RMWcc(__ASM_SIZE(btc): "lock ", _arg: *mut addr, _arg: c, _arg: "Ir", _arg: nr) -> return;
}

