//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/div64.h
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
// Copyright (C) 2003 Bernardo Innocenti <bernie@develer.com>
// Based on former asm-ppc/div64.h and asm-m68knommu/div64.h
//
// Optimization for constant divisors on 32-bit machines:
// Copyright (C) 2006-2015 Nicolas Pitre
//
// The semantics of do_div() is, in C++ notation, observing that the name
// is a function-like macro and the n parameter has the semantics of a C++
// reference:
//
// uint32_t do_div(uint64_t &n, uint32_t base)
// {
// uint32_t remainder = n % base;
// n = n / base;
// return remainder;
// }
//
// NOTE: macro parameter n is evaluated multiple times,
// beware of side effects!
//

//
// do_div - returns 2 values: calculate remainder and update new dividend
// @n: uint64_t dividend (will be updated)
// @base: uint32_t divisor
//
// Summary:
// ``uint32_t remainder = n % base;``
// ``n = n / base;``
//
// Return: (uint32_t)remainder
//
// NOTE: macro parameter @n is evaluated multiple times,
// beware of side effects!
//

//
// If the divisor happens to be constant, we determine the appropriate
// inverse at compile time to turn the division into a few inline
// multiplications which ought to be much faster.
//
// (It is unfortunate that gcc doesn't perform all this internally.)
//

// \
// Multiplication by reciprocal of b: n / b = n * (p / b) / p	\
// \
// We rely on the fact that most of this code gets optimized	\
// away at compile time due to constant propagation and only	\
// a few multiplication instructions should remain.		\
// Hence this monstrous macro (static inline doesn't always	\
// do the trick here).						\
// \
// determine MSB of b */					\
// compute m = ((p << 64) + b - 1) / b */			\
// one less than the dividend with highest result */		\
// test our ___m with res = m * x / (p << 64) */		\
// Now validate what we've got. */				\
// \
// We can't get away without a bias to compensate	\
// for bit truncation errors.  To avoid it we'd need an	\
// additional bit to represent m which would overflow	\
// a 64-bit variable.					\
// \
// Instead we do m = p / b and n / b = (n * m + m) / p.	\
// \
// Compute m = (p << 64) / b */				\
// Reduce m / p to help avoid overflow handling later. */	\
// \
// Perform (m_bias + m * n) / (1 << 64).			\
// From now on there will be actual runtime code generated.	\
// \

//
// Default C implementation for __arch_xprod_64()
//
// Prototype: uint64_t __arch_xprod_64(const uint64_t m, uint64_t n, bool bias)
// Semantic:  retval = ((bias ? m : 0) + m * n) >> 64
//
// The product is a 128-bit value, scaled down to 64 bits.
// Hoping for compile-time optimization of  conditional code.
// Architectures may provide their own optimized assembly implementation.
//

// Determine if overflow handling can be dispensed with.

extern "C" {
    pub fn __div64_32(dividend: *mut u64, divisor: u32) -> u32;
}

// The unnecessary pointer compare is there
// to check for type safety (n must be 64bit)
//

// the remainder can be computed with 32-bit regs */ \

