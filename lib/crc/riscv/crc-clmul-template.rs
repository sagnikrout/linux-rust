//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crc/riscv/crc-clmul-template.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright 2025 Google LLC
//
// This file is a "template" that generates a CRC function optimized using the
// RISC-V Zbc (scalar carryless multiplication) extension.  The includer of this
// file must define the following parameters to specify the type of CRC:
//
// crc_t: the data type of the CRC, e.g. u32 for a 32-bit CRC
// LSB_CRC: 0 for a msb (most-significant-bit) first CRC, i.e. natural
// mapping between bits and polynomial coefficients
// 1 for a lsb (least-significant-bit) first CRC, i.e. reflected
// mapping between bits and polynomial coefficients
//

//
// crc_load_long() loads one "unsigned long" of aligned data bytes, producing a
// polynomial whose bit order matches the CRC's bit order.
//

// XOR @crc into the end of @msgpoly that represents the high-order terms.

//
// Multiply the long-sized @msgpoly by x^n (a.k.a. x^CRC_BITS) and reduce it
// modulo the generator polynomial G.  This gives the CRC of @msgpoly.
//
// First step of Barrett reduction with integrated multiplication by
// x^n: calculate floor((msgpoly * x^n) / G).  This is the value by
// which G needs to be multiplied to cancel out the x^n and higher terms
// of msgpoly * x^n.  Do it using the following formula:
//
// msb-first:
// floor((msgpoly * floor(x^(BITS_PER_LONG-1+n) / G)) / x^(BITS_PER_LONG-1))
// lsb-first:
// floor((msgpoly * floor(x^(BITS_PER_LONG-1+n) / G) * x) / x^BITS_PER_LONG)
//
// barrett_reduction_const_1 contains floor(x^(BITS_PER_LONG-1+n) / G),
// which fits a long exactly.  Using any lower power of x there would
// not carry enough precision through the calculation, while using any
// higher power of x would require extra instructions to handle a wider
// multiplication.  In the msb-first case, using this power of x results
// in needing a floored division by x^(BITS_PER_LONG-1), which matches
// what clmulr produces.  In the lsb-first case, a factor of x gets
// implicitly introduced by each carryless multiplication (shown as
// '* x' above), and the floored division instead needs to be by
// x^BITS_PER_LONG which matches what clmul produces.
//

//
// Second step of Barrett reduction:
//
// crc := (msgpoly * x^n) + (G * floor((msgpoly * x^n) / G))
//
// This reduces (msgpoly * x^n) modulo G by adding the appropriate
// multiple of G to it.  The result uses only the x^0..x^(n-1) terms.
// HOWEVER, since the unreduced value (msgpoly * x^n) is zero in those
// terms in the first place, it is more efficient to do the equivalent:
//
// crc := ((G - x^n) * floor((msgpoly * x^n) / G)) mod x^n
//
// In the lsb-first case further modify it to the following which avoids
// a shift, as the crc ends up in the physically low n bits from clmulr:
//
// product := ((G - x^n) * x^(BITS_PER_LONG - n)) * floor((msgpoly * x^n) / G) * x
// crc := floor(product / x^(BITS_PER_LONG + 1 - n)) mod x^n
//
// barrett_reduction_const_2 contains the constant multiplier (G - x^n)
// or (G - x^n) * x^(BITS_PER_LONG - n) from the formulas above.  The
// cast of the result to crc_t is essential, as it applies the mod x^n!
//

extern "C" {
    pub fn clmulr(_arg: tmp, _arg: consts->barrett_reduction_const_2) -> return;
}

extern "C" {
    pub fn clmul(_arg: tmp, _arg: consts->barrett_reduction_const_2) -> return;
}

// Update @crc with the data from @msgpoly.
extern "C" {
    pub fn crc_clmul_long(_arg: crc_clmul_prep(crc, _arg: msgpoly), _arg: consts) -> return;
}
// Update @crc with 1 <= @len < sizeof(unsigned long) bytes of data.

extern "C" {
    pub fn crc_clmul_long(_arg: msgpoly, _arg: consts) -> return;
}

extern "C" {
    pub fn crc_clmul_long(_arg: msgpoly, (8*len): *mut consts) ^ (crc >>) -> return;
}

extern "C" {
    pub fn crc_clmul_long(_arg: msgpoly, (8*len): *mut consts) ^ (crc <<) -> return;
}

// This implementation assumes that the CRC fits in an unsigned long.
// If the buffer is not long-aligned, align it.
//
// Main loop.  Each iteration starts with a message polynomial
// (x^BITS_PER_LONG)*m0 + m1, then logically extends it by two
// more longs of data to form x^(3*BITS_PER_LONG)*m0 +
// x^(2*BITS_PER_LONG)*m1 + x^BITS_PER_LONG*m2 + m3, then
// "folds" that back into a congruent (modulo G) value that uses
// just m0 and m1 again.  This is done by multiplying m0 by the
// precomputed constant (x^(3*BITS_PER_LONG) mod G) and m1 by
// the precomputed constant (x^(2*BITS_PER_LONG) mod G), then
// adding the results to m2 and m3 as appropriate.  Each such
// multiplication produces a result twice the length of a long,
// which in RISC-V is two instructions clmul and clmulh.
//
// This could be changed to fold across more than 2 longs at a
// time if there is a CPU that can take advantage of it.
//
