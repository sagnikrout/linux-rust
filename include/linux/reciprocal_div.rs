//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/reciprocal_div.h
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

//
// This algorithm is based on the paper "Division by Invariant
// Integers Using Multiplication" by Torbjörn Granlund and Peter
// L. Montgomery.
//
// The assembler implementation from Agner Fog, which this code is
// based on, can be found here:
// http://www.agner.org/optimize/asmlib.zip
//
// This optimization for A/B is helpful if the divisor B is mostly
// runtime invariant. The reciprocal of B is calculated in the
// slow-path with reciprocal_value(). The fast-path can then just use
// a much faster multiplication operation with a variable dividend A
// to calculate the division A/B.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reciprocal_value {
    pub m: u32,
    pub sh2: u8 sh1,,
}

// "reciprocal_value" and "reciprocal_divide" together implement the basic
// version of the algorithm described in Figure 4.1 of the paper.
//
extern "C" {
    pub fn reciprocal_value(d: u32) -> reciprocal_value;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reciprocal_value_adv {
    pub m: u32,
    pub exp: u8 sh,,
    pub is_wide_m: bool,
}

// "reciprocal_value_adv" implements the advanced version of the algorithm
// described in Figure 4.2 of the paper except when "divisor > (1U << 31)" whose
// ceil(log2(d)) result will be 32 which then requires u128 divide on host. The
// exception case could be easily handled before calling "reciprocal_value_adv".
//
// The advanced version requires more complex calculation to get the reciprocal
// multiplier and other control variables, but then could reduce the required
// emulation operations.
//
// It makes no sense to use this advanced version for host divide emulation,
// those extra complexities for calculating multiplier etc could completely
// waive our saving on emulation operations.
//
// However, it makes sense to use it for JIT divide code generation for which
// we are willing to trade performance of JITed code with that of host. As shown
// by the following pseudo code, the required emulation operations could go down
// from 6 (the basic version) to 3 or 4.
//
// To use the result of "reciprocal_value_adv", suppose we want to calculate
// n/d, the pseudo C code will be:
//
// struct reciprocal_value_adv rvalue;
// u8 pre_shift, exp;
//
// // handle exception case.
// if (d >= (1U << 31)) {
// result = n >= d;
// return;
// }
//
// rvalue = reciprocal_value_adv(d, 32)
// exp = rvalue.exp;
// if (rvalue.is_wide_m && !(d & 1)) {
// // floor(log2(d & (2^32 -d)))
// pre_shift = fls(d & -d) - 1;
// rvalue = reciprocal_value_adv(d >> pre_shift, 32 - pre_shift);
// } else {
// pre_shift = 0;
// }
//
// // code generation starts.
// if (imm == 1U << exp) {
// result = n >> exp;
// } else if (rvalue.is_wide_m) {
// // pre_shift must be zero when reached here.
// t = (n * rvalue.m) >> 32;
// result = n - t;
// result >>= 1;
// result += t;
// result >>= rvalue.sh - 1;
// } else {
// if (pre_shift)
// result = n >> pre_shift;
// result = ((u64)result * rvalue.m) >> 32;
// result >>= rvalue.sh;
// }
//
extern "C" {
    pub fn reciprocal_value_adv(d: u32, prec: u8) -> reciprocal_value_adv;
}
