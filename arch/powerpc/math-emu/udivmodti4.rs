//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/udivmodti4.c
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
// This has so very few changes over libgcc2's __udivmoddi4 it isn't funny.

    void
    _fp_udivmodti4(_FP_W_TYPE q[2], _FP_W_TYPE r[2],
    _FP_W_TYPE n1, _FP_W_TYPE n0,
    _FP_W_TYPE d1, _FP_W_TYPE d0)
    {
    _FP_W_TYPE q0, q1, r0, r1;
    _FP_I_TYPE b, bm;
    if (d1 == 0)
    {

    if (d0 > n1)
    {
// 0q = nn / 0D
    udiv_qrnnd (q0, n0, n1, n0, d0);
    q1 = 0;
// Remainder in n0.
    }
    else
    {
// qq = NN / 0d
    if (d0 == 0)
    d0 = 1 / d0;	/* Divide intentionally by zero.  */
    udiv_qrnnd (q1, n1, 0, n1, d0);
    udiv_qrnnd (q0, n0, n1, n0, d0);
// Remainder in n0.
    }
    r0 = n0;
    r1 = 0;

    if (d0 > n1)
    {
// 0q = nn / 0D
    count_leading_zeros (bm, d0);
    if (bm != 0)
    {
// Normalize, i.e. make the most significant bit of the
    denominator set.  */
    d0 = d0 << bm;
    n1 = (n1 << bm) | (n0 >> (_FP_W_TYPE_SIZE - bm));
    n0 = n0 << bm;
    }
    udiv_qrnnd (q0, n0, n1, n0, d0);
    q1 = 0;
// Remainder in n0 >> bm.
    }
    else
    {
// qq = NN / 0d
    if (d0 == 0)
    d0 = 1 / d0;	/* Divide intentionally by zero.  */
    count_leading_zeros (bm, d0);
    if (bm == 0)
    {
// From (n1 >= d0) /\ (the most significant bit of d0 is set),
    conclude (the most significant bit of n1 is set) /\ (the
    leading quotient digit q1 = 1).
    This special case is necessary, not an optimization.
    (Shifts counts of SI_TYPE_SIZE are undefined.)  */
    n1 -= d0;
    q1 = 1;
    }
    else
    {
    _FP_W_TYPE n2;
// Normalize.
    b = _FP_W_TYPE_SIZE - bm;
    d0 = d0 << bm;
    n2 = n1 >> b;
    n1 = (n1 << bm) | (n0 >> b);
    n0 = n0 << bm;
    udiv_qrnnd (q1, n1, n2, n1, d0);
    }
// n1 != d0...
    udiv_qrnnd (q0, n0, n1, n0, d0);
// Remainder in n0 >> bm.
    }
    r0 = n0 >> bm;
    r1 = 0;

    }
    else
    {
    if (d1 > n1)
    {
// 00 = nn / DD
    q0 = 0;
    q1 = 0;
// Remainder in n1n0.
    r0 = n0;
    r1 = n1;
    }
    else
    {
// 0q = NN / dd
    count_leading_zeros (bm, d1);
    if (bm == 0)
    {
// From (n1 >= d1) /\ (the most significant bit of d1 is set),
    conclude (the most significant bit of n1 is set) /\ (the
    quotient digit q0 = 0 or 1).
    This special case is necessary, not an optimization.  */
// The condition on the next line takes advantage of that
    n1 >= d1 (true due to program flow).  */
    if (n1 > d1 || n0 >= d0)
    {
    q0 = 1;
    sub_ddmmss (n1, n0, n1, n0, d1, d0);
    }
    else
    q0 = 0;
    q1 = 0;
    r0 = n0;
    r1 = n1;
    }
    else
    {
    _FP_W_TYPE m1, m0, n2;
// Normalize.
    b = _FP_W_TYPE_SIZE - bm;
    d1 = (d1 << bm) | (d0 >> b);
    d0 = d0 << bm;
    n2 = n1 >> b;
    n1 = (n1 << bm) | (n0 >> b);
    n0 = n0 << bm;
    udiv_qrnnd (q0, n1, n2, n1, d1);
    umul_ppmm (m1, m0, q0, d0);
    if (m1 > n1 || (m1 == n1 && m0 > n0))
    {
    q0--;
    sub_ddmmss (m1, m0, m1, m0, d1, d0);
    }
    q1 = 0;
// Remainder in (n1n0 - m1m0) >> bm.
    sub_ddmmss (n1, n0, n1, n0, m1, m0);
    r0 = (n1 << b) | (n0 >> bm);
    r1 = n1 >> bm;
    }
    }
    }
    q[0] = q0; q[1] = q1;
    r[0] = r0, r[1] = r1;
    }
