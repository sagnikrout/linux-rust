//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/sfp-machine.h
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


// Machine-dependent software floating-point definitions.  PPC version.
//
// basic word size definitions
pub const _FP_W_TYPE_SIZE: c_int = 32;

// You can optionally code some things like addition in asm. For
// example, i386 defines __FP_FRAC_ADD_2 as asm. If you don't
// then you get a fragment of C code [if you change an #ifdef 0
// in op-2.h] or a call to add_ssaaaa (see below).
// Good places to look for asm fragments to use are gcc and glibc.
// gcc's longlong.h is useful.
//
// We need to know how to multiply and divide. If the host word size
// is >= 2*fracbits you can use FP_MUL_MEAT_n_imm(t,R,X,Y) which
// codes the multiply with whatever gcc does to 'a * b'.
// _FP_MUL_MEAT_n_wide(t,R,X,Y,f) is used when you have an asm
// function that can multiply two 1W values and get a 2W result.
// Otherwise you're stuck with _FP_MUL_MEAT_n_hard(t,R,X,Y) which
// does bitshifting to avoid overflow.
// For division there is FP_DIV_MEAT_n_imm(t,R,X,Y,f) for word size
// >= 2*fracbits, where f is either _FP_DIV_HELP_imm or
// _FP_DIV_HELP_ldiv (see op-1.h).
// _FP_DIV_MEAT_udiv() is if you have asm to do 2W/1W => (1W, 1W).
// [GCC and glibc have longlong.h which has the asm macro udiv_qrnnd
// to do this.]
// In general, 'n' is the number of words required to hold the type,
// and 't' is either S, D or Q for single/double/quad.
// -- PMM
//
// Example: SPARC64:
// #define _FP_MUL_MEAT_S(R,X,Y)	_FP_MUL_MEAT_1_imm(S,R,X,Y)
// #define _FP_MUL_MEAT_D(R,X,Y)	_FP_MUL_MEAT_1_wide(D,R,X,Y,umul_ppmm)
// #define _FP_MUL_MEAT_Q(R,X,Y)	_FP_MUL_MEAT_2_wide(Q,R,X,Y,umul_ppmm)
//
// #define _FP_DIV_MEAT_S(R,X,Y)	_FP_DIV_MEAT_1_imm(S,R,X,Y,_FP_DIV_HELP_imm)
// #define _FP_DIV_MEAT_D(R,X,Y)	_FP_DIV_MEAT_1_udiv(D,R,X,Y)
// #define _FP_DIV_MEAT_Q(R,X,Y)	_FP_DIV_MEAT_2_udiv_64(Q,R,X,Y)
//
// Example: i386:
// #define _FP_MUL_MEAT_S(R,X,Y)   _FP_MUL_MEAT_1_wide(S,R,X,Y,_i386_mul_32_64)
// #define _FP_MUL_MEAT_D(R,X,Y)   _FP_MUL_MEAT_2_wide(D,R,X,Y,_i386_mul_32_64)
//
// #define _FP_DIV_MEAT_S(R,X,Y)   _FP_DIV_MEAT_1_udiv(S,R,X,Y,_i386_div_64_32)
// #define _FP_DIV_MEAT_D(R,X,Y)   _FP_DIV_MEAT_2_udiv_64(D,R,X,Y)
//

// These macros define what NaN looks like. They're supposed to expand to
// a comma-separated set of 32bit unsigned ints that encode NaN.
//

pub const _FP_NANSIGN_S: c_int = 0;
pub const _FP_NANSIGN_D: c_int = 0;
pub const _FP_NANSIGN_Q: c_int = 0;
pub const _FP_KEEPNANFRACP: c_int = 1;

pub const FP_INHIBIT_RESULTS: c_int = 0;

// Exception flags.  We use the bit positions of the appropriate bits

// We only actually write to the destination register
// if exceptions signalled (if any) will not trap.
//

//
// If one NaN is signaling and the other is not,
// we choose that one, otherwise we choose X.
//

// Obtain the current rounding mode.

// the asm fragments go here: all these are taken from glibc-2.0.5's
// stdlib/longlong.h
//

// add_ssaaaa is used in op-2.h and should be equivalent to
// #define add_ssaaaa(sh,sl,ah,al,bh,bl) (sh = ah+bh+ (( sl = al+bl) < al))
// add_ssaaaa(high_sum, low_sum, high_addend_1, low_addend_1,
// high_addend_2, low_addend_2) adds two UWtype integers, composed by
// HIGH_ADDEND_1 and LOW_ADDEND_1, and HIGH_ADDEND_2 and LOW_ADDEND_2
// respectively.  The result is placed in HIGH_SUM and LOW_SUM.  Overflow
// (i.e. carry out) is not stored anywhere, and is lost.
//

// sub_ddmmss is used in op-2.h and udivmodti4.c and should be equivalent to
// #define sub_ddmmss(sh, sl, ah, al, bh, bl) (sh = ah-bh - ((sl = al-bl) > al))
// sub_ddmmss(high_difference, low_difference, high_minuend, low_minuend,
// high_subtrahend, low_subtrahend) subtracts two two-word UWtype integers,
// composed by HIGH_MINUEND_1 and LOW_MINUEND_1, and HIGH_SUBTRAHEND_2 and
// LOW_SUBTRAHEND_2 respectively.  The result is placed in HIGH_DIFFERENCE
// and LOW_DIFFERENCE.  Overflow (i.e. carry out) is not stored anywhere,
// and is lost.
//

// asm fragments for mul and div
// umul_ppmm(high_prod, low_prod, multipler, multiplicand) multiplies two
// UWtype integers MULTIPLER and MULTIPLICAND, and generates a two UWtype
// word product in HIGH_PROD and LOW_PROD.
//

// udiv_qrnnd(quotient, remainder, high_numerator, low_numerator,
// denominator) divides a UDWtype, composed by the UWtype integers
// HIGH_NUMERATOR and LOW_NUMERATOR, by DENOMINATOR and places the quotient
// in QUOTIENT and the remainder in REMAINDER.  HIGH_NUMERATOR must be less
// than DENOMINATOR for correct operation.  If, in addition, the most
// significant bit of DENOMINATOR must be 1, then the pre-processor symbol
// UDIV_NEEDS_NORMALIZATION is defined to 1.
//

pub const UDIV_NEEDS_NORMALIZATION: c_int = 1;

// Exception flags.

