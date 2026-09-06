//! Automatically rewritten from C Header to Rust Module
//! Source: include/math-emu/op-1.h
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

pub const _FP_ZEROFRAC_1: c_int = 0;
pub const _FP_MINFRAC_1: c_int = 1;

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
// Basic.  Assuming the host word size is >= 2*FRACBITS, we can do the

// Normalize since we know where the msb of the multiplicands	\
// Given a 1W * 1W => 2W primitive, do the extended multiplication.

// Normalize since we know where the msb of the multiplicands	\
// Finally, a simple widening multiply algorithm.  What fun!

// split the words in half */					\
// multiply the pieces */						\
// reassemble into two full words */				\
// normalize */							\
//
// Division algorithms:
//
// Basic.  Assuming the host word size is >= 2*FRACBITS, we can do the

// GCC's longlong.h defines a 2W / 1W => (1W,1W) primitive udiv_qrnnd

// Normalize Y -- i.e. make the most significant bit set.  */	\
// Shift X op correspondingly high, that is, up one full word.  */	\

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

