//! Automatically rewritten from C to Rust
//! Source: lib/raid/raid6/arm/recov_neon_inner.c
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
//
// Copyright (C) 2012 Intel Corporation
// Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
//

//
// AArch32 does not provide this intrinsic natively because it does not
// implement the underlying instruction. AArch32 only provides a 64-bit
// wide vtbl.8 instruction, so use that instead.
//
#[no_mangle]
unsafe extern "C" fn vqtbl1q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    static uint8x16_t vqtbl1q_u8(uint8x16_t a, uint8x16_t b)
    {
    union {
    uint8x16_t	val;
    uint8x8x2_t	pair;
    } __a = { a };
    return vcombine_u8(vtbl2_u8(__a.pair, vget_low_u8(b)),
    vtbl2_u8(__a.pair, vget_high_u8(b)));
    }

    void __raid6_2data_recov_neon(int bytes, uint8_t *p, uint8_t *q, uint8_t *dp,
    uint8_t *dq, const uint8_t *pbmul,
    const uint8_t *qmul)
    {
    let mut pm0: uint8x16_t = vld1q_u8(pbmul);
    let mut pm1: uint8x16_t = vld1q_u8(pbmul + 16);
    let mut qm0: uint8x16_t = vld1q_u8(qmul);
    let mut qm1: uint8x16_t = vld1q_u8(qmul + 16);
    let mut x0f: uint8x16_t = vdupq_n_u8(0x0f);
//
// while ( bytes-- ) {
// uint8_t px, qx, db;
//
// px    = *p ^ *dp;
// qx    = qmul[*q ^ *dq];
// *dq++ = db = pbmul[px] ^ qx;
// *dp++ = db ^ px;
// p++; q++;
// }
//
    while (bytes) {
    uint8x16_t vx, vy, px, qx, db;
    px = veorq_u8(vld1q_u8(p), vld1q_u8(dp));
    vx = veorq_u8(vld1q_u8(q), vld1q_u8(dq));
    vy = vshrq_n_u8(vx, 4);
    vx = vqtbl1q_u8(qm0, vandq_u8(vx, x0f));
    vy = vqtbl1q_u8(qm1, vy);
    qx = veorq_u8(vx, vy);
    vy = vshrq_n_u8(px, 4);
    vx = vqtbl1q_u8(pm0, vandq_u8(px, x0f));
    vy = vqtbl1q_u8(pm1, vy);
    vx = veorq_u8(vx, vy);
    db = veorq_u8(vx, qx);
    vst1q_u8(dq, db);
    vst1q_u8(dp, veorq_u8(db, px));
    bytes -= 16;
    p += 16;
    q += 16;
    dp += 16;
    dq += 16;
    }
    }
    void __raid6_datap_recov_neon(int bytes, uint8_t *p, uint8_t *q, uint8_t *dq,
    const uint8_t *qmul)
    {
    let mut qm0: uint8x16_t = vld1q_u8(qmul);
    let mut qm1: uint8x16_t = vld1q_u8(qmul + 16);
    let mut x0f: uint8x16_t = vdupq_n_u8(0x0f);
//
// while (bytes--) {
// *p++ ^= *dq = qmul[*q ^ *dq];
// q++; dq++;
// }
//
    while (bytes) {
    uint8x16_t vx, vy;
    vx = veorq_u8(vld1q_u8(q), vld1q_u8(dq));
    vy = vshrq_n_u8(vx, 4);
    vx = vqtbl1q_u8(qm0, vandq_u8(vx, x0f));
    vy = vqtbl1q_u8(qm1, vy);
    vx = veorq_u8(vx, vy);
    vy = veorq_u8(vx, vld1q_u8(p));
    vst1q_u8(dq, vx);
    vst1q_u8(p, vy);
    bytes -= 16;
    p += 16;
    q += 16;
    dq += 16;
    }
    }
