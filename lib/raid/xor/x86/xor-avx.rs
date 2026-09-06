//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/x86/xor-avx.c
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
// Optimized XOR parity functions for AVX
//
// Copyright (C) 2012 Intel Corporation
// Author: Jim Kukunas <james.t.kukunas@linux.intel.com>
//
// Based on Ingo Molnar and Zach Brown's respective MMX and SSE routines
//

    BLOCK(32 * i, 0) \
    BLOCK(32 * (i + 1), 1) \
    BLOCK(32 * (i + 2), 2) \
    BLOCK(32 * (i + 3), 3)

    BLOCK4(0) \
    BLOCK4(4) \
    BLOCK4(8) \
    BLOCK4(12)
    static void xor_avx_2(unsigned long bytes, unsigned long * __restrict p0,
    const unsigned long * __restrict p1)
    {
    let mut lines: c_ulong = bytes >> 9;
    while (lines--) {

    do { \
    asm volatile("vmovdqa %0, %%ymm" #reg : : "m" (p1[i / sizeof(*p1)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm"  #reg : : \
    "m" (p0[i / sizeof(*p0)])); \
    asm volatile("vmovdqa %%ymm" #reg ", %0" : \
    "=m" (p0[i / sizeof(*p0)])); \
    } while (0);
    BLOCK16()
    p0 = (unsigned long *)((uintptr_t)p0 + 512);
    p1 = (unsigned long *)((uintptr_t)p1 + 512);
    }
    }
    static void xor_avx_3(unsigned long bytes, unsigned long * __restrict p0,
    const unsigned long * __restrict p1,
    const unsigned long * __restrict p2)
    {
    let mut lines: c_ulong = bytes >> 9;
    while (lines--) {

    do { \
    asm volatile("vmovdqa %0, %%ymm" #reg : : "m" (p2[i / sizeof(*p2)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p1[i / sizeof(*p1)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p0[i / sizeof(*p0)])); \
    asm volatile("vmovdqa %%ymm" #reg ", %0" : \
    "=m" (p0[i / sizeof(*p0)])); \
    } while (0);
    BLOCK16()
    p0 = (unsigned long *)((uintptr_t)p0 + 512);
    p1 = (unsigned long *)((uintptr_t)p1 + 512);
    p2 = (unsigned long *)((uintptr_t)p2 + 512);
    }
    }
    static void xor_avx_4(unsigned long bytes, unsigned long * __restrict p0,
    const unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3)
    {
    let mut lines: c_ulong = bytes >> 9;
    while (lines--) {

    do { \
    asm volatile("vmovdqa %0, %%ymm" #reg : : "m" (p3[i / sizeof(*p3)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p2[i / sizeof(*p2)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p1[i / sizeof(*p1)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p0[i / sizeof(*p0)])); \
    asm volatile("vmovdqa %%ymm" #reg ", %0" : \
    "=m" (p0[i / sizeof(*p0)])); \
    } while (0);
    BLOCK16();
    p0 = (unsigned long *)((uintptr_t)p0 + 512);
    p1 = (unsigned long *)((uintptr_t)p1 + 512);
    p2 = (unsigned long *)((uintptr_t)p2 + 512);
    p3 = (unsigned long *)((uintptr_t)p3 + 512);
    }
    }
    static void xor_avx_5(unsigned long bytes, unsigned long * __restrict p0,
    const unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4)
    {
    let mut lines: c_ulong = bytes >> 9;
    while (lines--) {

    do { \
    asm volatile("vmovdqa %0, %%ymm" #reg : : "m" (p4[i / sizeof(*p4)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p3[i / sizeof(*p3)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p2[i / sizeof(*p2)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p1[i / sizeof(*p1)])); \
    asm volatile("vxorps %0, %%ymm" #reg ", %%ymm" #reg : : \
    "m" (p0[i / sizeof(*p0)])); \
    asm volatile("vmovdqa %%ymm" #reg ", %0" : \
    "=m" (p0[i / sizeof(*p0)])); \
    } while (0);
    BLOCK16()
    p0 = (unsigned long *)((uintptr_t)p0 + 512);
    p1 = (unsigned long *)((uintptr_t)p1 + 512);
    p2 = (unsigned long *)((uintptr_t)p2 + 512);
    p3 = (unsigned long *)((uintptr_t)p3 + 512);
    p4 = (unsigned long *)((uintptr_t)p4 + 512);
    }
    }
    DO_XOR_BLOCKS(avx_inner, xor_avx_2, xor_avx_3, xor_avx_4, xor_avx_5);
    static void xor_gen_avx(void *dest, void **srcs, unsigned int src_cnt,
    unsigned int bytes)
    {
    kernel_fpu_begin();
    xor_gen_avx_inner(dest, srcs, src_cnt, bytes);
    kernel_fpu_end();
    }
    struct xor_block_template xor_block_avx = {
    .name		= "avx",
    .xor_gen	= xor_gen_avx,
    };
