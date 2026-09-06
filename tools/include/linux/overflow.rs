//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/overflow.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT

//
// We need to compute the minimum and maximum values representable in a given
// type. These macros may also be useful elsewhere. It would seem more obvious
// to do something like:
//
// #define type_min(T) (T)(is_signed_type(T) ? (T)1 << (8*sizeof(T)-1) : 0)
// #define type_max(T) (T)(is_signed_type(T) ? ((T)1 << (8*sizeof(T)-1)) - 1 : ~(T)0)
//
// Unfortunately, the middle expressions, strictly speaking, have
// undefined behaviour, and at least some versions of gcc warn about
// the type_max expression (but not if -fsanitize=undefined is in
// effect; in that case, the warning is deferred to runtime...).
//
// The slightly excessive casting in type_min is to make sure the
// macros also produce sensible values for the exotic type _Bool. [The
// overflow checkers only almost work for _Bool, but that's
// a-feature-not-a-bug, since people shouldn't be doing arithmetic on
// _Bools. Besides, the gcc builtins don't allow _Bool* as third
// argument.]
//
// Idea stolen from
// https://mail-index.netbsd.org/tech-misc/2007/02/05/0000.html -
// credit to Christian Biere.
//

//
// For simplicity and code hygiene, the fallback code below insists on
// a, b and *d having the same type (similar to the min() and max()
// macros), whereas gcc's type-generic overflow checkers accept
// different types. Hence we don't just make check_add_overflow an
// alias for __builtin_add_overflow, but add type checks similar to
// below.
//

//
// size_mul() - Calculate size_t multiplication with saturation at SIZE_MAX
// @factor1: first factor
// @factor2: second factor
//
// Returns: calculate @factor1 * @factor2, both promoted to size_t,
// with any overflow causing the return value to be SIZE_MAX. The
// lvalue must be size_t to avoid implicit type conversion.
//
// array_size() - Calculate size of 2-dimensional array.
//
// @a: dimension one
// @b: dimension two
//
// Calculates size of 2-dimensional array: @a * @b.
//
// Returns: number of bytes needed to represent the array or SIZE_MAX on
// overflow.
//
// array3_size() - Calculate size of 3-dimensional array.
//
// @a: dimension one
// @b: dimension two
// @c: dimension three
//
// Calculates size of 3-dimensional array: @a * @b * @c.
//
// Returns: number of bytes needed to represent the array or SIZE_MAX on
// overflow.
//
// struct_size() - Calculate size of structure with trailing array.
// @p: Pointer to the structure.
// @member: Name of the array member.
// @n: Number of elements in the array.
//
// Calculates size of memory needed for structure @p followed by an
// array of @n @member elements.
//
// Return: number of bytes needed or SIZE_MAX on overflow.
//

