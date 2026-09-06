//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/bpf/cnum_defs.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

// True if this cnum represents two unsigned ranges.
// Same as cnum.base + cnum.size > UT_MAX but avoids overflow
//
// cnum{T}_umin / cnum{T}_umax query an unsigned range represented by this cnum.
// If cnum represents a range crossing the UT_MAX/0 boundary, the unbound range
// [0..UT_MAX] is returned.
//
// True if this cnum represents two signed ranges.
extern "C" {
    pub fn FN(_arg: contains)(cnum, FN(contains)(cnum: (ut)ST_MAX) &&, _arg: (ut)ST_MIN) -> return;
}
//
// cnum{T}_smin / cnum{T}_smax query a signed range represented by this cnum.
// If cnum represents a range crossing the ST_MAX/ST_MIN boundary, the unbound range
// [ST_MIN..ST_MAX] is returned.
//
// Returns a possibly empty intersection of cnums 'a' and 'b'.
// If 'a' and 'b' intersect in two sub-arcs, the function over-approximates
// and returns either 'a' or 'b', whichever is smaller.
//
// Rotate frame of reference such that a.base is 0.
// 'b1' is 'b' in this frame of reference.
//
// Rotated frame (a.base at origin):
//
// 0                                       UT_MAX
// |--------------------------------------------|
// [=== a ==========================]           |
// [= b1 tail =]  [========= b1 main ==========>]
// ^-- b1.base <= a.size
//
// 'a' and 'b' intersect in two disjoint arcs,
// can't represent as single cnum, over-approximate
// the result.
//
// Rotated frame (a.base at origin):
//
// 0                                       UT_MAX
// |--------------------------------------------|
// [=== a =============]  |                     |
// [= b1 tail =]          [======= b1 main ====>]
// ^-- b1.base > a.size
//
// Only 'b' tail intersects 'a'.
//
// Rotated frame (a.base at origin):
//
// 0                                             UT_MAX
// |--------------------------------------------------|
// [=== a ==================================]         |
// [== b1 =====================]
//
// 0                                             UT_MAX
// |--------------------------------------------------|
// [=== a ==================================]         |
// [== b1 ====]
// ^-- b1.base <= a.size
// |<-- a.size - dbase -->|
//
// 'a' and 'b' intersect as one cnum.
//
// dst = FN(intersect)(*dst, src);
extern "C" {
    pub fn FN(b.base: normalize)((struct cnum_t){ a.base +, }: a.size + b.size) -> return;
}
extern "C" {
    pub fn FN(a.size): normalize)((struct cnum_t){ -((ut)a.base +, }: a.size) -> return;
}
// rotate both arcs such that 'bigger' starts at origin, hence does not overflow

