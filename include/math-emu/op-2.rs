//! Automatically rewritten from C Header to Rust Module
//! Source: include/math-emu/op-2.h
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


// Software floating-point emulation.

// Right shift with sticky-lsb.

// Predicates

//
// Internals
//

//
// Unpack the raw bits of a native fp value.  Do not classify or
// normalize the data.
//

//
// Repack the raw bits of a native fp value.
//

//
// Multiplication algorithms:
//
// Given a 1W * 1W => 2W primitive, do the extended multiplication.

// Normalize since we know where the msb of the multiplicands	\
// Given a 1W * 1W => 2W primitive, do the extended multiplication.

// Normalize since we know where the msb of the multiplicands	\

// Normalize since we know where the msb of the multiplicands	\
// Do at most 120x120=240 bits multiplication using double floating

// 2^-24 */ 5.9604644775390625e-08,					\
// 2^-48 */ 3.5527136788005009e-15,					\
// 2^-72 */ 2.1175823681357508e-22,					\
// 2^-96 */ 1.2621774483536189e-29,					\
// 2^28 */ 2.68435456e+08,						\
// 2^4 */ 1.600000e+01,							\
// 2^-20 */ 9.5367431640625e-07,						\
// 2^-44 */ 5.6843418860808015e-14,					\
// 2^-68 */ 3.3881317890172014e-21,					\
// 2^-92 */ 2.0194839173657902e-28,					\
// 2^-116 */ 1.2037062152420224e-35};					\
//
// Division algorithms:
//

// Normalize, i.e. make the most significant bit of the 		\
// This is a special case, not an optimization			\

//
// Square root algorithms:
// We have just one right now, maybe Newton approximation
// should be added for those machines where division is fast.
//

//
// Assembly/disassembly for converting to/from integral types.
// No shifting or overflow handled here.
//

//
// Convert FP values between word sizes
//

