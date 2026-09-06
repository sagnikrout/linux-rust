//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/mpi/longlong.h
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


// longlong.h -- definitions for mixed size 32/64 bit arithmetic.
// Note: I added some stuff for use with gnupg
//
// Copyright (C) 1991, 1992, 1993, 1994, 1996, 1998,
// 2000, 2001, 2002, 2003 Free Software Foundation, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU Library General Public License as published by
// the Free Software Foundation; either version 2 of the License, or (at your
// option) any later version.
//
// This file is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Library General Public
// License for more details.
//
// You should have received a copy of the GNU Library General Public License
// along with this file; see the file COPYING.LIB.  If not, write to
// the Free Software Foundation, Inc., 59 Temple Place - Suite 330, Boston,
// MA 02111-1307, USA.

// You have to define the following before including this file:
//
// UWtype -- An unsigned type, default type for operations (typically a "word")
// UHWtype -- An unsigned type, at least half the size of UWtype.
// UDWtype -- An unsigned type, at least twice as large a UWtype
// W_TYPE_SIZE -- size in bits of UWtype
//
// SItype, USItype -- Signed and unsigned 32 bit types.
// DItype, UDItype -- Signed and unsigned 64 bit types.
//
// On a 32 bit machine UWtype should typically be USItype;
// on a 64 bit machine, UWtype should typically be UDItype.
//

// This is used to make sure no undesirable sharing between different libraries

// Define auxiliary asm macros.
//
// 1) umul_ppmm(high_prod, low_prod, multiplier, multiplicand) multiplies two
// UWtype integers MULTIPLIER and MULTIPLICAND, and generates a two UWtype
// word product in HIGH_PROD and LOW_PROD.
//
// 2) __umulsidi3(a,b) multiplies two UWtype integers A and B, and returns a
// UDWtype product.  This is just a variant of umul_ppmm.
// 3) udiv_qrnnd(quotient, remainder, high_numerator, low_numerator,
// denominator) divides a UDWtype, composed by the UWtype integers
// HIGH_NUMERATOR and LOW_NUMERATOR, by DENOMINATOR and places the quotient
// in QUOTIENT and the remainder in REMAINDER.	HIGH_NUMERATOR must be less
// than DENOMINATOR for correct operation.  If, in addition, the most
// significant bit of DENOMINATOR must be 1, then the pre-processor symbol
// UDIV_NEEDS_NORMALIZATION is defined to 1.
// 4) sdiv_qrnnd(quotient, remainder, high_numerator, low_numerator,
// denominator).  Like udiv_qrnnd but the numbers are signed.  The quotient
// is rounded towards 0.
//
// 5) count_leading_zeros(x) counts the number of zero-bits from the
// msb to the first non-zero bit in the UWtype X.  This is the number of
// steps X needs to be shifted left to set the msb.
// count_leading_zeros(0) == BITS_PER_LONG
//
// 6) count_trailing_zeros() like count_leading_zeros(), but counts
// from the least significant end.
//
// 7) add_ssaaaa(high_sum, low_sum, high_addend_1, low_addend_1,
// high_addend_2, low_addend_2) adds two UWtype integers, composed by
// HIGH_ADDEND_1 and LOW_ADDEND_1, and HIGH_ADDEND_2 and LOW_ADDEND_2
// respectively.  The result is placed in HIGH_SUM and LOW_SUM.  Overflow
// (i.e. carry out) is not stored anywhere, and is lost.
//
// 8) sub_ddmmss(high_difference, low_difference, high_minuend, low_minuend,
// high_subtrahend, low_subtrahend) subtracts two two-word UWtype integers,
// composed by HIGH_MINUEND_1 and LOW_MINUEND_1, and HIGH_SUBTRAHEND_2 and
// LOW_SUBTRAHEND_2 respectively.  The result is placed in HIGH_DIFFERENCE
// and LOW_DIFFERENCE.	Overflow (i.e. carry out) is not stored anywhere,
// and is lost.
//
// If any of these macros are left undefined for a particular CPU,
// C macros are used.
// The CPUs come in alphabetical order below.
//
// Please add support for more CPUs here, or improve the current support
// for the CPUs below!

// We sometimes need to clobber "cc" with gcc2, but that would not be

//
// A29K
//

pub const UMUL_TIME: c_int = 46;

extern "C" {
    pub fn __udiv_qrnnd(: *mut UDItype, _arg: UDItype, _arg: UDItype, _arg: UDItype) -> UDItype;
}
pub const UDIV_TIME: c_int = 220;

//
// ARM
//

pub const UMUL_TIME: c_int = 20;
pub const UDIV_TIME: c_int = 100;

//
// CLIPPER
//

//
// GMICRO
//

//
// HPPA
//

// xmpyu uses floating point register which is not allowed in Linux kernel.

pub const UMUL_TIME: c_int = 8;
pub const UDIV_TIME: c_int = 60;

pub const UMUL_TIME: c_int = 40;
pub const UDIV_TIME: c_int = 80;

extern "C" {
    pub fn __udiv_qrnnd() -> USItype;
}

//
// I370
//

//
// I386
//

pub const UMUL_TIME: c_int = 40;

pub const UDIV_TIME: c_int = 40;

//
// I860
//

//
// I960
//

//
// 68000
//

pub const UMUL_TIME: c_int = 45;

pub const UDIV_TIME: c_int = 90;

pub const UMUL_TIME: c_int = 100;
pub const UDIV_TIME: c_int = 400;

//
// 88000
//

pub const UMUL_TIME: c_int = 5;
pub const UDIV_TIME: c_int = 25;

pub const UMUL_TIME: c_int = 17;
pub const UDIV_TIME: c_int = 150;

//
// MIPS
//

pub const UMUL_TIME: c_int = 10;
pub const UDIV_TIME: c_int = 100;

//
// MIPS/64
//

//
// GCC ends up emitting a __multi3 intrinsic call for MIPS64r6 with the plain C
// code below, so we special case MIPS64r6 until the compiler can do better.
//

pub const UMUL_TIME: c_int = 20;
pub const UDIV_TIME: c_int = 140;

//
// 32000
//

//
// PPC
//

pub const UMUL_TIME: c_int = 15;

pub const SMUL_TIME: c_int = 14;
pub const UDIV_TIME: c_int = 120;

pub const UMUL_TIME: c_int = 8;

pub const SMUL_TIME: c_int = 4;

pub const UDIV_TIME: c_int = 100;

//
// PYR
//

// This insn works on Pyramids with AP, XP, or MI CPUs, but not with SP.

//
// RT/ROMP
//

pub const UMUL_TIME: c_int = 20;
pub const UDIV_TIME: c_int = 200;

//
// SH2
//

pub const UMUL_TIME: c_int = 5;

//
// SPARC
//

// Don't match immediate range because, 1) it is not often useful,

pub const UMUL_TIME: c_int = 5;

pub const UDIV_TIME: c_int = 25;

// This has hardware multiply but not divide.  It also has two additional

pub const UMUL_TIME: c_int = 5;

pub const UDIV_TIME: c_int = 37;

// Default to sparc v7 versions of umul_ppmm and udiv_qrnnd.

// It's quite necessary to add this much assembler for the sparc.

//
// VAX
//

//
// Z8000
//

//
// Generic Versions
//

// If this machine has no inline assembler, use C macros.

// Define this unconditionally, so it can be used for debugging.

// If the processor has no udiv_qrnnd but sdiv_qrnnd, go through

// If udiv_qrnnd was not defined for this processor, use __udiv_qrnnd_c.

pub const UDIV_NEEDS_NORMALIZATION: c_int = 1;

pub const UDIV_NEEDS_NORMALIZATION: c_int = 0;
