//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/cnum.c
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

pub const T: c_int = 32;

pub const T: c_int = 64;

#[no_mangle]
pub unsafe extern "C" fn cnum32_from_cnum64(cnum: cnum64) -> cnum32 {
    struct cnum32 cnum32_from_cnum64(struct cnum64 cnum)
    {
    if (cnum64_is_empty(cnum))
    return CNUM32_EMPTY;
    if (cnum.size >= U32_MAX)
    return (struct cnum32){ .base = 0, .size = U32_MAX };
    else
    return (struct cnum32){ .base = (u32)cnum.base, .size = cnum.size };
    }
//
// Suppose 'a' and 'b' are laid out as follows:
//
// 64-bit number axis --->
//
// N*2^32                   (N+1)*2^32                (N+2)*2^32                (N+3)*2^32
// ||------|---|=====|-------||----------|=====|-------||----------|=====|----|--||
// |   |< b >|                   |< b >|                   |< b >|    |
// |   |                                                         |    |
// |<--+--------------------------- a ---------------------------+--->|
// |                                                         |
// |<-------------------------- t -------------------------->|
//
// In such a case it is possible to infer a more tight representation t
// such that ∀ v ∈ a, (u32)v ∈ b: v ∈ t.
//
#[no_mangle]
pub unsafe extern "C" fn cnum64_cnum32_intersect(a: cnum64, b: cnum32) -> cnum64 {
    struct cnum64 cnum64_cnum32_intersect(struct cnum64 a, struct cnum32 b)
    {
//
// To simplify reasoning, rotate the circles so that [virtual] a1 starts
// at u32 boundary, b1 represents b in this new frame of reference.
//
    let mut b1: cnum32 = { b.base - (u32)a.base, b.size };
    let mut t: cnum64 = a;
    u64 d, b1_max;
    if (cnum64_is_empty(a) || cnum32_is_empty(b))
    return CNUM64_EMPTY;
    if (cnum32_urange_overflow(b1)) {
    b1_max = (u32)b1.base + (u32)b1.size; /* overflow here is fine and necessary */
    if ((u32)a.size > b1_max && (u32)a.size < b1.base) {
//
// N*2^32                   (N+1)*2^32
// ||=====|------------|=====||=====|---------|---|=====||
// |b1 ->|            |<- b1||b1 ->|         |   |<- b1|
// |<----------------- a1 ------------------>|
// |<-------------- t ------------>|<-- d -->| (after adjustment)
// ^
// b1_max
//
    d = (u32)a.size - b1_max;
    t.size -= d;
    } else {
//
// No adjustments possible in the following cases:
//
// ||=====|------------|=====||===|=|-------------|=|===||
// |b1 ->|            |<- b1||b1 +>|             |<+ b1|
// |<----------------- a1 ------>|                 |
// |<----------------- (or) a1 ------------------->|
//
    }
    } else {
    if (t.size < b1.base)
//
// N*2^32                   (N+1)*2^32
// ||----------|--|=======|--||------>
// |<-- a1 -->|  |<- b ->|
//
    return CNUM64_EMPTY;
//
// N*2^32                   (N+1)*2^32
// ||-------------|========|-||-----| -------|========|-||
// |             |<- b1 ->|        |        |<- b1 ->|
// |<------------+ a1 ------------>|
// |<------ t ------>| (after adjustment)
//
    t.base += b1.base;
    t.size -= b1.base;
    b1_max = b1.base + b1.size;
    d = 0;
    if ((u32)a.size < b1.base)
//
// N*2^32                   (N+1)*2^32
// ||-------------|========|-||------|-------|========|-||
// |             |<- b1 ->|         |       |<- b1 ->|
// |<------------+-- a1 --+-------->|
// |<- t  ->|<-- d -->| (after adjustment)
//
    d = (u32)a.size + (BIT_ULL(32) - b1_max);
#[no_mangle]
pub unsafe extern "C" fn if(b1_max: (u32)a.size >=) -> else {
    else if ((u32)a.size >= b1_max)
//
// N*2^32                   (N+1)*2^32
// ||--|========|------------||--|========|-------|-----||
// |  |<- b1 ->|                |<- b1 ->|       |
// |<-+------------------ a1 ------------+------>|
// |<-------------- t --------------->|<- d ->| (after adjustment)
//
    d = (u32)a.size - b1_max;
    if (t.size < d)
    return CNUM64_EMPTY;
    t.size -= d;
    }
    return t;
    }
