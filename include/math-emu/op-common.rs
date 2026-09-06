//! Automatically rewritten from C Header to Rust Module
//! Source: include/math-emu/op-common.h
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


// Software floating-point emulation. Common operations.

//
// Finish truly unpacking a native fp value by classifying the kind
// of fp value and normalizing both the exponent and the fraction.
//

// a denormalized number */					\
// Check for signaling NaN */					\
//
// Before packing the bits back into the native fp result, take care
// of such mundane things as rounding and overflow.  Also, for some
// kinds of fp values, the original parts may not have been fully
// extracted -- but that is ok, we can regenerate them now.
//

// overflow */					\
// Overflow to infinity */			\
// Overflow to maximum normal */		\
// we've got a denormalized number */			\
// underflow to zero */				\
// This one accepts raw argument and not cooked,  returns
// 1 if X is a signaling NaN.
//

//
// Main addition routine.  The input values should be cooked.
//

// shift the smaller number so that its exponent matches the larger */ \
// return an exact zero */				     \
// renormalize after subtraction */			     \
// +INF + -INF => NAN */					     \
// make sure the sign is correct */					     \

//
// Main negation routine.  FIXME -- when we care about setting exception
// bits reliably, this will not do.  We should examine all of the fp classes.
//

//
// Main multiplication routine.  The input values should be cooked.
//

//
// Main division routine.  The input values should be cooked.
//

//
// Main differential comparison routine.  The inputs should be raw not
// cooked.  The return is -1,0,1 for normal values, 2 otherwise.
//

// NANs are unordered */						\
// Simplification for strict equality.

// NANs are unordered */						  \
//
// Main square root routine.  The input value should be cooked.
//

//
// Convert from FP to integer
//
// RSIGNED can have following values:
// 0:  the number is required to be 0..(2^rsize)-1, if not, NV is set plus
// the result is either 0 or (2^rsize)-1 depending on the sign in such case.
// 1:  the number is required to be -(2^(rsize-1))..(2^(rsize-1))-1, if not, NV is
// set plus the result is either -(2^(rsize-1)) or (2^(rsize-1))-1 depending
// on the sign in such case.
// 2:  the number is required to be -(2^(rsize-1))..(2^(rsize-1))-1, if not, NV is
// set plus the result is truncated to fit into destination.
// -1: the number is required to be -(2^(rsize-1))..(2^rsize)-1, if not, NV is
// set plus the result is either -(2^(rsize-1)) or (2^(rsize-1))-1 depending
// on the sign in such case.
//

//
// Helper primitives.
//
// Count leading zeros in a word.

// this is just to shut the compiler up about shifts > word length -- PMM 02/1998

