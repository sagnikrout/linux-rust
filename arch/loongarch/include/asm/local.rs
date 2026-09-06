//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/local.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// Same as above, but return the result value
//

extern "C" {
    pub fn cmpxchg_local(_arg: &l->a.counter, _arg: old, _arg: new) -> return;
}

//
// local_add_unless - add unless the number is already a given value
// @l: pointer of type local_t
// @a: the amount to add to l...
// @u: ...unless l is equal to u.
//
// Atomically adds @a to @l, if @v was not already @u.
// Returns true if the addition was done.
//

//
// local_sub_and_test - subtract value from variable and test result
// @i: integer value to subtract
// @l: pointer of type local_t
//
// Atomically subtracts @i from @l and returns
// true if the result is zero, or false for all
// other cases.
//

//
// local_inc_and_test - increment and test
// @l: pointer of type local_t
//
// Atomically increments @l by 1
// and returns true if the result is zero, or false for all
// other cases.
//

//
// local_dec_and_test - decrement by 1 and test
// @l: pointer of type local_t
//
// Atomically decrements @l by 1 and
// returns true if the result is 0, or false for all other
// cases.
//

//
// local_add_negative - add and test if negative
// @l: pointer of type local_t
// @i: integer value to add
//
// Atomically adds @i to @l and returns true
// if the result is negative, or false when
// result is greater than or equal to zero.
//

// Use these for per-cpu local_t variables: on some archs they are
// much more efficient than these naive implementations.  Note they take
// a variable, not an address.
//

