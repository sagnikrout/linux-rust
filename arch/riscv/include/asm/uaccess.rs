//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/uaccess.h
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
// This file was copied from include/asm-generic/uaccess.h
//

// Virtual addresses are sign-extended; physical addresses are zero-extended.

//
// User space memory access functions
//

//
// This is the smallest unsigned integer type that can fit a value
// (up to 'long long')
//

//
// The exception table consists of pairs of addresses: the first is the
// address of an instruction that is allowed to fault, and the second is
// the address at which the program should continue.  No registers are
// modified, so it is entirely up to the continuation code to figure out
// what to do.
//
// All the routines below use bits of fixup code that are out of line
// with the main instruction path.  This means when everything is well,
// we don't even have to jump over them.  Further, they do not intrude
// on our cache or tlb entries.
//
pub const __LSW: c_int = 0;
pub const __MSW: c_int = 1;
//
// The "__xxx" versions of the user access functions do not verify the address
// space - it must have been done previously with a separate "access_ok()"
// call.
//

//
// Use a temporary variable for the output of the asm goto to avoid a
// triggering an LLVM assertion due to sign extending the output when
// it is used in later function calls:
// https://github.com/llvm/llvm-project/issues/143795
//

//
// __get_user: - Get a simple variable from user space, with less checking.
// @x:   Variable to store result.
// @ptr: Source address, in user space.
//
// Context: User context only.  This function may sleep.
//
// This macro copies a single simple variable from user space to kernel
// space.  It supports simple types like char and int, but not larger
// data types like structures or arrays.
//
// @ptr must have pointer-to-simple-variable type, and the result of
// dereferencing @ptr must be assignable to @x without a cast.
//
// Caller must check the pointer with access_ok() before calling this
// function.
//
// Returns zero on success, or -EFAULT on error.
// On error, the variable @x is set to zero.
//

//
// get_user: - Get a simple variable from user space.
// @x:   Variable to store result.
// @ptr: Source address, in user space.
//
// Context: User context only.  This function may sleep.
//
// This macro copies a single simple variable from user space to kernel
// space.  It supports simple types like char and int, but not larger
// data types like structures or arrays.
//
// @ptr must have pointer-to-simple-variable type, and the result of
// dereferencing @ptr must be assignable to @x without a cast.
//
// Returns zero on success, or -EFAULT on error.
// On error, the variable @x is set to zero.
//

//
// __put_user: - Write a simple value into user space, with less checking.
// @x:   Value to copy to user space.
// @ptr: Destination address, in user space.
//
// Context: User context only.  This function may sleep.
//
// This macro copies a single simple value from kernel space to user
// space.  It supports simple types like char and int, but not larger
// data types like structures or arrays.
//
// @ptr must have pointer-to-simple-variable type, and @x must be assignable
// to the result of dereferencing @ptr. The value of @x is copied to avoid
// re-ordering where @x is evaluated inside the block that enables user-space
// access (thus bypassing user space protection if @x is a function).
//
// Caller must check the pointer with access_ok() before calling this
// function.
//
// Returns zero on success, or -EFAULT on error.
//

//
// put_user: - Write a simple value into user space.
// @x:   Value to copy to user space.
// @ptr: Destination address, in user space.
//
// Context: User context only.  This function may sleep.
//
// This macro copies a single simple value from kernel space to user
// space.  It supports simple types like char and int, but not larger
// data types like structures or arrays.
//
// @ptr must have pointer-to-simple-variable type, and @x must be assignable
// to the result of dereferencing @ptr.
//
// Returns zero on success, or -EFAULT on error.
//

extern "C" {
    pub fn __asm_copy_from_user(_arg: to, _arg: untagged_addr(from), _arg: n) -> return;
}
extern "C" {
    pub fn __asm_copy_to_user(_arg: untagged_addr(to), _arg: from, _arg: n) -> return;
}
extern "C" {
    pub fn strncpy_from_user(dest: *mut c_char, src: *const char __user, count: c_long) -> c_long;
}
extern "C" {
    pub fn strnlen_user(str: *const char __user, n: c_long) -> long __must_check;
}
extern "C" {
    pub fn __clear_user(addr: *mut void __user, n: c_ulong) -> unsigned long __must_check;
}

//
// We want the unsafe accessors to always be inlined and use
// the error labels - thus the macro games.
//

