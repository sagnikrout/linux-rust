//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/x86/curve25519.h
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
// Copyright (C) 2020 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
// Copyright (c) 2016-2020 INRIA, CMU and Microsoft Corporation
//

// Computes the addition of four-element f1 with value in f2
// and returns the carry (if any)
// Clear registers to propagate the carry bit
// Begin addition chain
// Return the carry bit in a register
// Computes the field addition of two field elements
// Compute the raw addition of f1 + f2
// Wrap the result back into the field
// Step 1: Compute carry*38
// Step 2: Add carry*38 to the original sum
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// Computes the field subtraction of two field elements
// Compute the raw subtraction of f1-f2
// Wrap the result back into the field
// Step 1: Compute carry*38
// Step 2: Subtract carry*38 from the original difference
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// Store the result
// Computes a field multiplication: out <- f1 * f2
// Uses the 8-element buffer tmp for intermediate results
// Compute the raw multiplication: tmp <- src1 * src2
// Compute src1[0] * src2
// Compute src1[1] * src2
// Compute src1[2] * src2
// Compute src1[3] * src2
// Line up pointers
// Wrap the result back into the field
// Step 1: Compute dst + carry == tmp_hi * 38 + tmp_lo
// Step 2: Fold the carry back into dst
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// Computes two field multiplications:
// out[0] <- f1[0] * f2[0]
// out[1] <- f1[1] * f2[1]
// Uses the 16-element buffer tmp for intermediate results:
// Compute the raw multiplication tmp[0] <- f1[0] * f2[0]
// Compute src1[0] * src2
// Compute src1[1] * src2
// Compute src1[2] * src2
// Compute src1[3] * src2
// Compute the raw multiplication tmp[1] <- f1[1] * f2[1]
// Compute src1[0] * src2
// Compute src1[1] * src2
// Compute src1[2] * src2
// Compute src1[3] * src2
// Line up pointers
// Wrap the results back into the field
// Step 1: Compute dst + carry == tmp_hi * 38 + tmp_lo
// Step 2: Fold the carry back into dst
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// Step 1: Compute dst + carry == tmp_hi * 38 + tmp_lo
// Step 2: Fold the carry back into dst
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// Computes the field multiplication of four-element f1 with value in f2
// Requires f2 to be smaller than 2^17
// Compute the raw multiplication of f1*f2
// Wrap the result back into the field
// Step 1: Compute carry*38
// Step 2: Fold the carry back into dst
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// Computes p1 <- bit ? p2 : p1 in constant time
// Transfer bit into CF flag
// cswap p1[0], p2[0]
// cswap p1[1], p2[1]
// cswap p1[2], p2[2]
// cswap p1[3], p2[3]
// cswap p1[4], p2[4]
// cswap p1[5], p2[5]
// cswap p1[6], p2[6]
// cswap p1[7], p2[7]
// Computes the square of a field element: out <- f * f
// Uses the 8-element buffer tmp for intermediate results
// Compute the raw multiplication: tmp <- f * f
// Step 1: Compute all partial products
// Step 2: Compute two parallel carry chains
// Step 3: Compute intermediate squares
// Line up pointers
// Wrap the result back into the field
// Step 1: Compute dst + carry == tmp_hi * 38 + tmp_lo
// Step 2: Fold the carry back into dst
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// Computes two field squarings:
// out[0] <- f[0] * f[0]
// out[1] <- f[1] * f[1]
// Uses the 16-element buffer tmp for intermediate results
// Step 1: Compute all partial products
// Step 2: Compute two parallel carry chains
// Step 3: Compute intermediate squares
// Step 1: Compute all partial products
// Step 2: Compute two parallel carry chains
// Step 3: Compute intermediate squares
// Line up pointers
// Step 1: Compute dst + carry == tmp_hi * 38 + tmp_lo
// Step 2: Fold the carry back into dst
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// Step 1: Compute dst + carry == tmp_hi * 38 + tmp_lo
// Step 2: Fold the carry back into dst
// Step 3: Fold the carry bit back in; guaranteed not to carry at this point
// The below constants were generated using this sage script:
//
// #!/usr/bin/env sage
// import sys
// from sage.all import
// def limbs(n):
// n = int(n)
// l = ((n >> 0) % 2^64, (n >> 64) % 2^64, (n >> 128) % 2^64, (n >> 192) % 2^64)
// return "0x%016xULL, 0x%016xULL, 0x%016xULL, 0x%016xULL" % l
// ec = EllipticCurve(GF(2^255 - 19), [0, 486662, 0, 1, 0])
// p_minus_s = (ec.lift_x(9) - ec.lift_x(1))[0]
// print("static const u64 p_minus_s[] = { %s };\n" % limbs(p_minus_s))
// print("static const u64 table_ladder[] = {")
// p = ec.lift_x(9)
// for i in range(252):
// l = (p[0] + p[2]) / (p[0] - p[2])
// print(("\t%s" + ("," if i != 251 else "")) % limbs(l))
// p = p * 2
// print("};")
//
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: curve25519_use_bmi2_adx) -> static __ro_after_init;
}
