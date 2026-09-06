//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/arm64/xor-eor3.c
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

    extern void __xor_eor3_2(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2);
#[no_mangle]
pub unsafe extern "C" fn eor3(p: uint64x2_t, q: uint64x2_t, r: uint64x2_t) -> uint64x2_t {
    static inline uint64x2_t eor3(uint64x2_t p, uint64x2_t q, uint64x2_t r)
    {
    uint64x2_t res;
    asm(ARM64_ASM_PREAMBLE ".arch_extension sha3\n"
    "eor3 %0.16b, %1.16b, %2.16b, %3.16b"
    : "=w"(res) : "w"(p), "w"(q), "w"(r));
    return res;
    }
    static void __xor_eor3_3(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3)
    {
    uint64_t *dp1 = (uint64_t *)p1;
    uint64_t *dp2 = (uint64_t *)p2;
    uint64_t *dp3 = (uint64_t *)p3;
    register uint64x2_t v0, v1, v2, v3;
    let mut lines: c_long = bytes / (sizeof(uint64x2_t) * 4);
    do {
// p1 ^= p2 ^ p3
    v0 = eor3(vld1q_u64(dp1 + 0), vld1q_u64(dp2 + 0),
    vld1q_u64(dp3 + 0));
    v1 = eor3(vld1q_u64(dp1 + 2), vld1q_u64(dp2 + 2),
    vld1q_u64(dp3 + 2));
    v2 = eor3(vld1q_u64(dp1 + 4), vld1q_u64(dp2 + 4),
    vld1q_u64(dp3 + 4));
    v3 = eor3(vld1q_u64(dp1 + 6), vld1q_u64(dp2 + 6),
    vld1q_u64(dp3 + 6));
// store
    vst1q_u64(dp1 + 0, v0);
    vst1q_u64(dp1 + 2, v1);
    vst1q_u64(dp1 + 4, v2);
    vst1q_u64(dp1 + 6, v3);
    dp1 += 8;
    dp2 += 8;
    dp3 += 8;
    } while (--lines > 0);
    }
    static void __xor_eor3_4(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4)
    {
    uint64_t *dp1 = (uint64_t *)p1;
    uint64_t *dp2 = (uint64_t *)p2;
    uint64_t *dp3 = (uint64_t *)p3;
    uint64_t *dp4 = (uint64_t *)p4;
    register uint64x2_t v0, v1, v2, v3;
    let mut lines: c_long = bytes / (sizeof(uint64x2_t) * 4);
    do {
// p1 ^= p2 ^ p3
    v0 = eor3(vld1q_u64(dp1 + 0), vld1q_u64(dp2 + 0),
    vld1q_u64(dp3 + 0));
    v1 = eor3(vld1q_u64(dp1 + 2), vld1q_u64(dp2 + 2),
    vld1q_u64(dp3 + 2));
    v2 = eor3(vld1q_u64(dp1 + 4), vld1q_u64(dp2 + 4),
    vld1q_u64(dp3 + 4));
    v3 = eor3(vld1q_u64(dp1 + 6), vld1q_u64(dp2 + 6),
    vld1q_u64(dp3 + 6));
// p1 ^= p4
    v0 = veorq_u64(v0, vld1q_u64(dp4 + 0));
    v1 = veorq_u64(v1, vld1q_u64(dp4 + 2));
    v2 = veorq_u64(v2, vld1q_u64(dp4 + 4));
    v3 = veorq_u64(v3, vld1q_u64(dp4 + 6));
// store
    vst1q_u64(dp1 + 0, v0);
    vst1q_u64(dp1 + 2, v1);
    vst1q_u64(dp1 + 4, v2);
    vst1q_u64(dp1 + 6, v3);
    dp1 += 8;
    dp2 += 8;
    dp3 += 8;
    dp4 += 8;
    } while (--lines > 0);
    }
    static void __xor_eor3_5(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4,
    const unsigned long * __restrict p5)
    {
    uint64_t *dp1 = (uint64_t *)p1;
    uint64_t *dp2 = (uint64_t *)p2;
    uint64_t *dp3 = (uint64_t *)p3;
    uint64_t *dp4 = (uint64_t *)p4;
    uint64_t *dp5 = (uint64_t *)p5;
    register uint64x2_t v0, v1, v2, v3;
    let mut lines: c_long = bytes / (sizeof(uint64x2_t) * 4);
    do {
// p1 ^= p2 ^ p3
    v0 = eor3(vld1q_u64(dp1 + 0), vld1q_u64(dp2 + 0),
    vld1q_u64(dp3 + 0));
    v1 = eor3(vld1q_u64(dp1 + 2), vld1q_u64(dp2 + 2),
    vld1q_u64(dp3 + 2));
    v2 = eor3(vld1q_u64(dp1 + 4), vld1q_u64(dp2 + 4),
    vld1q_u64(dp3 + 4));
    v3 = eor3(vld1q_u64(dp1 + 6), vld1q_u64(dp2 + 6),
    vld1q_u64(dp3 + 6));
// p1 ^= p4 ^ p5
    v0 = eor3(v0, vld1q_u64(dp4 + 0), vld1q_u64(dp5 + 0));
    v1 = eor3(v1, vld1q_u64(dp4 + 2), vld1q_u64(dp5 + 2));
    v2 = eor3(v2, vld1q_u64(dp4 + 4), vld1q_u64(dp5 + 4));
    v3 = eor3(v3, vld1q_u64(dp4 + 6), vld1q_u64(dp5 + 6));
// store
    vst1q_u64(dp1 + 0, v0);
    vst1q_u64(dp1 + 2, v1);
    vst1q_u64(dp1 + 4, v2);
    vst1q_u64(dp1 + 6, v3);
    dp1 += 8;
    dp2 += 8;
    dp3 += 8;
    dp4 += 8;
    dp5 += 8;
    } while (--lines > 0);
    }
    __DO_XOR_BLOCKS(eor3_inner, __xor_eor3_2, __xor_eor3_3, __xor_eor3_4,
    __xor_eor3_5);
