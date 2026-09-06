//! Automatically rewritten from C Header to Rust Module
//! Source: include/math-emu/op-4.h
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

// This one was broken too

// Right shift with sticky-lsb.
// What this actually means is that we do a standard right-shift,
// but that if any of the bits that fall off the right hand side
// were one then we always set the LSbit.
//

// s is now != 0 if we want to set the LSbit */				\
// don't fix the LSB until the very end when we're sure f[0] is stable */	\

//
// Multiplication algorithms:
//
// Given a 1W * 1W => 2W primitive, do the extended multiplication.

// Normalize since we know where the msb of the multiplicands	    \

// Normalize since we know where the msb of the multiplicands	    \
//
// Helper utility for _FP_DIV_MEAT_4_udiv:
// pppp = m * nnn
//

//
// Division algorithms:
//

// Normalize, i.e. make the most significant bit of the 		    \
// This is a special case, not an optimization		    \
//
// Square root algorithms:
// We have just one right now, maybe Newton approximation
// should be added for those machines where division is fast.
//

//
// Internals
//

// Convert FP values between word sizes. This appears to be more
// complicated than I'd have expected it to be, so these might be
// wrong... These macros are in any case somewhat bogus because they
// use information about what various FRAC_n variables look like
// internally [eg, that 2 word vars are X_f0 and x_f1]. But so do
// the ones in op-2.h and op-1.h.
//

// Assembly/disassembly for converting to/from integral types.
// No shifting or overflow handled here.
//
// Put the FP value X into r, which is an integer of size rsize.

// I'm feeling lazy so we deal with int == 3words (implausible)*/	\
// and int == 4words as a single case.			 */	\
// "No disassemble Number Five!"
// move an integer of size rsize into X's fractional part. We rely on
// the _f[] array consisting of words of size _FP_W_TYPE_SIZE to avoid
// having to mask the values we store into it.
//

