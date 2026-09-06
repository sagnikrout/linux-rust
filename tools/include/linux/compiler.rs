//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/compiler.h
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
// compiletime_assert - break build and emit msg if condition is false
// @condition: a compile-time constant condition to check
// @msg:       a message to emit if condition is false
//
// In tradition of POSIX assert, this macro will break the build if the
// supplied condition is *false*, emitting the supplied error message if the
// compiler has support to do so.
//

// Optimization barrier
// The "volatile" is due to gcc bugs

// Macro flag: #define noinline

// Are two types/vars the same type (ignoring qualifiers)?

//
// This returns a constant expression while determining if an argument is
// a constant expression, most importantly without evaluating the argument.
// Glory to Martin Uecker <Martin.Uecker@med.uni-goettingen.de>
//

//
// Similar to statically_true() but produces a constant expression
//
// To be used in conjunction with macros, such as BUILD_BUG_ON_ZERO(),
// which require their input to be a constant expression and for which
// statically_true() would otherwise fail.
//
// This is a trade-off: const_true() requires all its operands to be
// compile time constants. Else, it would always returns false even on
// the most trivial cases like:
//
// true || non_const_var
//
// On the opposite, statically_true() is able to fold more complex
// tautologies and will return true on expressions such as:
//
// !(non_const_var * 8 % 4)
//
// For the general case, statically_true() is better.
//

//
// FIXME: Big hammer to get rid of tons of:
// "warning: always_inline function might not be inlinable"
//
// At least on android-ndk-r12/platforms/android-24/arch-arm
//

// Macro flag: #define __user
// Macro flag: #define __rcu
// Macro flag: #define __read_mostly

//
// Following functions are taken from kernel sources and
// break aliasing rules in their original form.
//
// While kernel is compiled with -fno-strict-aliasing,
// perf uses -Wstrict-aliasing=3 which makes build fail
// under gcc 4.4.
//
// Using extra __may_alias__ type to allow aliasing
// in this case.
//
// Prevent the compiler from merging or refetching reads or writes. The
// compiler is also forbidden from reordering successive instances of
// READ_ONCE and WRITE_ONCE, but only when the compiler is aware of some
// particular ordering. One way to make the compiler aware of ordering is to
// put the two invocations of READ_ONCE or WRITE_ONCE in different C
// statements.
//
// These two macros will also work on aggregate data types like structs or
// unions. If the size of the accessed data type exceeds the word size of
// the machine (e.g., 32 bits or 64 bits) READ_ONCE() and WRITE_ONCE() will
// fall back to memcpy and print a compile-time warning.
//
// Their two major use cases are: (1) Mediating communication between
// process-level code and irq/NMI handlers, all running on the same CPU,
// and (2) Ensuring that the compiler does not fold, spindle, or otherwise
// mutilate accesses that either do not require ordering or that interact
// with an explicit memory barrier or atomic instruction that provides the
// required ordering.
//

// Indirect macros required for expanded argument pasting, eg. __LINE__.

// Make the optimizer believe the variable can be manipulated arbitrarily.

