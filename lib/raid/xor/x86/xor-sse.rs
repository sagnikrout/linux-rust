//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/x86/xor-sse.c
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
//
// Optimized XOR parity functions for SSE.
//
// Cache avoiding checksumming functions utilizing KNI instructions
// Copyright (C) 1999 Zach Brown (with obvious credit due Ingo)
//
// Based on
// High-speed RAID5 checksumming functions utilizing SSE instructions.
// Copyright (C) 1998 Ingo Molnar.
//
// x86-64 changes / gcc fixes from Andi Kleen.
// Copyright 2002 Andi Kleen, SuSE Labs.
//

// reduce register pressure

// Macro flag: #define NOP(x)

    pf(i)					\
    op(i, 0)				\
    op(i + 1, 1)			\
    op(i + 2, 2)		\
    op(i + 3, 3)
    static void
    xor_sse_2(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2)
    {
    let mut lines: c_ulong = bytes >> 8;
    asm volatile(

    LD(i, 0)				\
    LD(i + 1, 1)			\
    PF1(i)					\
    PF1(i + 2)		\
    LD(i + 2, 2)		\
    LD(i + 3, 3)	\
    PF0(i + 4)				\
    PF0(i + 6)		\
    XO1(i, 0)				\
    XO1(i + 1, 1)			\
    XO1(i + 2, 2)		\
    XO1(i + 3, 3)	\
    ST(i, 0)				\
    ST(i + 1, 1)			\
    ST(i + 2, 2)		\
    ST(i + 3, 3)	\
    PF0(0)
    PF0(2)
    " .align 32			;\n"
    " 1:                            ;\n"
    BLOCK(0)
    BLOCK(4)
    BLOCK(8)
    BLOCK(12)
    "       add %[inc], %[p1]       ;\n"
    "       add %[inc], %[p2]       ;\n"
    "       dec %[cnt]              ;\n"
    "       jnz 1b                  ;\n"
    : [cnt] "+r" (lines),
    [p1] "+r" (p1), [p2] "+r" (p2)
    : [inc] XOR_CONSTANT_CONSTRAINT (256UL)
    : "memory");
    }
    static void
    xor_sse_2_pf64(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2)
    {
    let mut lines: c_ulong = bytes >> 8;
    asm volatile(

    BLK64(PF0, LD, i)	\
    BLK64(PF1, XO1, i)	\
    BLK64(NOP, ST, i)	\
    " .align 32			;\n"
    " 1:                            ;\n"
    BLOCK(0)
    BLOCK(4)
    BLOCK(8)
    BLOCK(12)
    "       add %[inc], %[p1]       ;\n"
    "       add %[inc], %[p2]       ;\n"
    "       dec %[cnt]              ;\n"
    "       jnz 1b                  ;\n"
    : [cnt] "+r" (lines),
    [p1] "+r" (p1), [p2] "+r" (p2)
    : [inc] XOR_CONSTANT_CONSTRAINT (256UL)
    : "memory");
    }
    static void
    xor_sse_3(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3)
    {
    let mut lines: c_ulong = bytes >> 8;
    asm volatile(

    PF1(i)					\
    PF1(i + 2)		\
    LD(i, 0)				\
    LD(i + 1, 1)			\
    LD(i + 2, 2)		\
    LD(i + 3, 3)	\
    PF2(i)					\
    PF2(i + 2)		\
    PF0(i + 4)				\
    PF0(i + 6)		\
    XO1(i, 0)				\
    XO1(i + 1, 1)			\
    XO1(i + 2, 2)		\
    XO1(i + 3, 3)	\
    XO2(i, 0)				\
    XO2(i + 1, 1)			\
    XO2(i + 2, 2)		\
    XO2(i + 3, 3)	\
    ST(i, 0)				\
    ST(i + 1, 1)			\
    ST(i + 2, 2)		\
    ST(i + 3, 3)	\
    PF0(0)
    PF0(2)
    " .align 32			;\n"
    " 1:                            ;\n"
    BLOCK(0)
    BLOCK(4)
    BLOCK(8)
    BLOCK(12)
    "       add %[inc], %[p1]       ;\n"
    "       add %[inc], %[p2]       ;\n"
    "       add %[inc], %[p3]       ;\n"
    "       dec %[cnt]              ;\n"
    "       jnz 1b                  ;\n"
    : [cnt] "+r" (lines),
    [p1] "+r" (p1), [p2] "+r" (p2), [p3] "+r" (p3)
    : [inc] XOR_CONSTANT_CONSTRAINT (256UL)
    : "memory");
    }
    static void
    xor_sse_3_pf64(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3)
    {
    let mut lines: c_ulong = bytes >> 8;
    asm volatile(

    BLK64(PF0, LD, i)	\
    BLK64(PF1, XO1, i)	\
    BLK64(PF2, XO2, i)	\
    BLK64(NOP, ST, i)	\
    " .align 32			;\n"
    " 1:                            ;\n"
    BLOCK(0)
    BLOCK(4)
    BLOCK(8)
    BLOCK(12)
    "       add %[inc], %[p1]       ;\n"
    "       add %[inc], %[p2]       ;\n"
    "       add %[inc], %[p3]       ;\n"
    "       dec %[cnt]              ;\n"
    "       jnz 1b                  ;\n"
    : [cnt] "+r" (lines),
    [p1] "+r" (p1), [p2] "+r" (p2), [p3] "+r" (p3)
    : [inc] XOR_CONSTANT_CONSTRAINT (256UL)
    : "memory");
    }
    static void
    xor_sse_4(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4)
    {
    let mut lines: c_ulong = bytes >> 8;
    asm volatile(

    PF1(i)					\
    PF1(i + 2)		\
    LD(i, 0)				\
    LD(i + 1, 1)			\
    LD(i + 2, 2)		\
    LD(i + 3, 3)	\
    PF2(i)					\
    PF2(i + 2)		\
    XO1(i, 0)				\
    XO1(i + 1, 1)			\
    XO1(i + 2, 2)		\
    XO1(i + 3, 3)	\
    PF3(i)					\
    PF3(i + 2)		\
    PF0(i + 4)				\
    PF0(i + 6)		\
    XO2(i, 0)				\
    XO2(i + 1, 1)			\
    XO2(i + 2, 2)		\
    XO2(i + 3, 3)	\
    XO3(i, 0)				\
    XO3(i + 1, 1)			\
    XO3(i + 2, 2)		\
    XO3(i + 3, 3)	\
    ST(i, 0)				\
    ST(i + 1, 1)			\
    ST(i + 2, 2)		\
    ST(i + 3, 3)	\
    PF0(0)
    PF0(2)
    " .align 32			;\n"
    " 1:                            ;\n"
    BLOCK(0)
    BLOCK(4)
    BLOCK(8)
    BLOCK(12)
    "       add %[inc], %[p1]       ;\n"
    "       add %[inc], %[p2]       ;\n"
    "       add %[inc], %[p3]       ;\n"
    "       add %[inc], %[p4]       ;\n"
    "       dec %[cnt]              ;\n"
    "       jnz 1b                  ;\n"
    : [cnt] "+r" (lines), [p1] "+r" (p1),
    [p2] "+r" (p2), [p3] "+r" (p3), [p4] "+r" (p4)
    : [inc] XOR_CONSTANT_CONSTRAINT (256UL)
    : "memory");
    }
    static void
    xor_sse_4_pf64(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4)
    {
    let mut lines: c_ulong = bytes >> 8;
    asm volatile(

    BLK64(PF0, LD, i)	\
    BLK64(PF1, XO1, i)	\
    BLK64(PF2, XO2, i)	\
    BLK64(PF3, XO3, i)	\
    BLK64(NOP, ST, i)	\
    " .align 32			;\n"
    " 1:                            ;\n"
    BLOCK(0)
    BLOCK(4)
    BLOCK(8)
    BLOCK(12)
    "       add %[inc], %[p1]       ;\n"
    "       add %[inc], %[p2]       ;\n"
    "       add %[inc], %[p3]       ;\n"
    "       add %[inc], %[p4]       ;\n"
    "       dec %[cnt]              ;\n"
    "       jnz 1b                  ;\n"
    : [cnt] "+r" (lines), [p1] "+r" (p1),
    [p2] "+r" (p2), [p3] "+r" (p3), [p4] "+r" (p4)
    : [inc] XOR_CONSTANT_CONSTRAINT (256UL)
    : "memory");
    }
    static void
    xor_sse_5(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4,
    const unsigned long * __restrict p5)
    {
    let mut lines: c_ulong = bytes >> 8;
    asm volatile(

    PF1(i)					\
    PF1(i + 2)		\
    LD(i, 0)				\
    LD(i + 1, 1)			\
    LD(i + 2, 2)		\
    LD(i + 3, 3)	\
    PF2(i)					\
    PF2(i + 2)		\
    XO1(i, 0)				\
    XO1(i + 1, 1)			\
    XO1(i + 2, 2)		\
    XO1(i + 3, 3)	\
    PF3(i)					\
    PF3(i + 2)		\
    XO2(i, 0)				\
    XO2(i + 1, 1)			\
    XO2(i + 2, 2)		\
    XO2(i + 3, 3)	\
    PF4(i)					\
    PF4(i + 2)		\
    PF0(i + 4)				\
    PF0(i + 6)		\
    XO3(i, 0)				\
    XO3(i + 1, 1)			\
    XO3(i + 2, 2)		\
    XO3(i + 3, 3)	\
    XO4(i, 0)				\
    XO4(i + 1, 1)			\
    XO4(i + 2, 2)		\
    XO4(i + 3, 3)	\
    ST(i, 0)				\
    ST(i + 1, 1)			\
    ST(i + 2, 2)		\
    ST(i + 3, 3)	\
    PF0(0)
    PF0(2)
    " .align 32			;\n"
    " 1:                            ;\n"
    BLOCK(0)
    BLOCK(4)
    BLOCK(8)
    BLOCK(12)
    "       add %[inc], %[p1]       ;\n"
    "       add %[inc], %[p2]       ;\n"
    "       add %[inc], %[p3]       ;\n"
    "       add %[inc], %[p4]       ;\n"
    "       add %[inc], %[p5]       ;\n"
    "       dec %[cnt]              ;\n"
    "       jnz 1b                  ;\n"
    : [cnt] "+r" (lines), [p1] "+r" (p1), [p2] "+r" (p2),
    [p3] "+r" (p3), [p4] "+r" (p4), [p5] "+r" (p5)
    : [inc] XOR_CONSTANT_CONSTRAINT (256UL)
    : "memory");
    }
    static void
    xor_sse_5_pf64(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4,
    const unsigned long * __restrict p5)
    {
    let mut lines: c_ulong = bytes >> 8;
    asm volatile(

    BLK64(PF0, LD, i)	\
    BLK64(PF1, XO1, i)	\
    BLK64(PF2, XO2, i)	\
    BLK64(PF3, XO3, i)	\
    BLK64(PF4, XO4, i)	\
    BLK64(NOP, ST, i)	\
    " .align 32			;\n"
    " 1:                            ;\n"
    BLOCK(0)
    BLOCK(4)
    BLOCK(8)
    BLOCK(12)
    "       add %[inc], %[p1]       ;\n"
    "       add %[inc], %[p2]       ;\n"
    "       add %[inc], %[p3]       ;\n"
    "       add %[inc], %[p4]       ;\n"
    "       add %[inc], %[p5]       ;\n"
    "       dec %[cnt]              ;\n"
    "       jnz 1b                  ;\n"
    : [cnt] "+r" (lines), [p1] "+r" (p1), [p2] "+r" (p2),
    [p3] "+r" (p3), [p4] "+r" (p4), [p5] "+r" (p5)
    : [inc] XOR_CONSTANT_CONSTRAINT (256UL)
    : "memory");
    }
    DO_XOR_BLOCKS(sse_inner, xor_sse_2, xor_sse_3, xor_sse_4, xor_sse_5);
    static void xor_gen_sse(void *dest, void **srcs, unsigned int src_cnt,
    unsigned int bytes)
    {
    kernel_fpu_begin();
    xor_gen_sse_inner(dest, srcs, src_cnt, bytes);
    kernel_fpu_end();
    }
    struct xor_block_template xor_block_sse = {
    .name		= "sse",
    .xor_gen	= xor_gen_sse,
    };
    DO_XOR_BLOCKS(sse_pf64_inner, xor_sse_2_pf64, xor_sse_3_pf64, xor_sse_4_pf64,
    xor_sse_5_pf64);
    static void xor_gen_sse_pf64(void *dest, void **srcs, unsigned int src_cnt,
    unsigned int bytes)
    {
    kernel_fpu_begin();
    xor_gen_sse_pf64_inner(dest, srcs, src_cnt, bytes);
    kernel_fpu_end();
    }
    struct xor_block_template xor_block_sse_pf64 = {
    .name		= "prefetch64-sse",
    .xor_gen	= xor_gen_sse_pf64,
    };
