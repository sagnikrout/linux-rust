//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/local.h
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
// local_sub_and_test - subtract value from variable and test result
// @i: integer value to subtract
// @l: pointer to type local_t
//
// Atomically subtracts @i from @l and returns
// true if the result is zero, or false for all
// other cases.
//
extern "C" {
    pub fn GEN_BINARY_RMWcc(_arg: _ASM_SUB, _arg: l->a.counter, _arg: e, _arg: "er", _arg: i) -> return;
}
//
// local_dec_and_test - decrement and test
// @l: pointer to type local_t
//
// Atomically decrements @l by 1 and
// returns true if the result is 0, or false for all other
// cases.
//
extern "C" {
    pub fn GEN_UNARY_RMWcc(_arg: _ASM_DEC, _arg: l->a.counter, _arg: e) -> return;
}
//
// local_inc_and_test - increment and test
// @l: pointer to type local_t
//
// Atomically increments @l by 1
// and returns true if the result is zero, or false for all
// other cases.
//
extern "C" {
    pub fn GEN_UNARY_RMWcc(_arg: _ASM_INC, _arg: l->a.counter, _arg: e) -> return;
}
//
// local_add_negative - add and test if negative
// @i: integer value to add
// @l: pointer to type local_t
//
// Atomically adds @i to @l and returns true
// if the result is negative, or false when
// result is greater than or equal to zero.
//
extern "C" {
    pub fn GEN_BINARY_RMWcc(_arg: _ASM_ADD, _arg: l->a.counter, _arg: s, _arg: "er", _arg: i) -> return;
}
//
// local_add_return - add and return
// @i: integer value to add
// @l: pointer to type local_t
//
// Atomically adds @i to @l and returns @i + @l
//
extern "C" {
    pub fn local_add_return(_arg: -i, _arg: l) -> return;
}

extern "C" {
    pub fn cmpxchg_local(_arg: &l->a.counter, _arg: old, _arg: new) -> return;
}
//
// Implement local_xchg using CMPXCHG instruction without the LOCK prefix.
// XCHG is expensive due to the implied LOCK prefix.  The processor
// cannot prefetch cachelines if XCHG is used.
//
// local_add_unless - add unless the number is already a given value
// @l: pointer of type local_t
// @a: the amount to add to l...
// @u: ...unless l is equal to u.
//
// Atomically adds @a to @l, if @v was not already @u.
// Returns true if the addition was done.
//

// On x86_32, these are no better than the atomic variants.
// On x86-64 these are better than the atomic variants on SMP kernels
// because they dont use a lock prefix.
//

