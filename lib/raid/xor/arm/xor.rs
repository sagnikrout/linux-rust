//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/arm/xor.c
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
// Copyright (C) 2001 Russell King
//

    __asm__("ldmia	%0, {%1, %2}" \
    : "=r" (dst), "=r" (a1), "=r" (a2) \
    : "0" (dst))

    __asm__("ldmia	%0, {%1, %2, %3, %4}" \
    : "=r" (dst), "=r" (a1), "=r" (a2), "=r" (a3), "=r" (a4) \
    : "0" (dst))

    __asm__("ldmia	%0!, {%1, %2}" \
    : "=r" (src), "=r" (b1), "=r" (b2) \
    : "0" (src)); \
    __XOR(a1, b1); __XOR(a2, b2);

    __asm__("ldmia	%0!, {%1, %2, %3, %4}" \
    : "=r" (src), "=r" (b1), "=r" (b2), "=r" (b3), "=r" (b4) \
    : "0" (src)); \
    __XOR(a1, b1); __XOR(a2, b2); __XOR(a3, b3); __XOR(a4, b4)

    __asm__ __volatile__("stmia	%0!, {%2, %3}" \
    : "=r" (dst) \
    : "0" (dst), "r" (a1), "r" (a2))

    __asm__ __volatile__("stmia	%0!, {%2, %3, %4, %5}" \
    : "=r" (dst) \
    : "0" (dst), "r" (a1), "r" (a2), "r" (a3), "r" (a4))
    static void
    xor_arm4regs_2(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2)
    {
    let mut lines: c_uint = bytes / sizeof(unsigned long) / 4;
    register unsigned int a1 __asm__("r4");
    register unsigned int a2 __asm__("r5");
    register unsigned int a3 __asm__("r6");
    register unsigned int a4 __asm__("r10");
    register unsigned int b1 __asm__("r8");
    register unsigned int b2 __asm__("r9");
    register unsigned int b3 __asm__("ip");
    register unsigned int b4 __asm__("lr");
    do {
    GET_BLOCK_4(p1);
    XOR_BLOCK_4(p2);
    PUT_BLOCK_4(p1);
    } while (--lines);
    }
    static void
    xor_arm4regs_3(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3)
    {
    let mut lines: c_uint = bytes / sizeof(unsigned long) / 4;
    register unsigned int a1 __asm__("r4");
    register unsigned int a2 __asm__("r5");
    register unsigned int a3 __asm__("r6");
    register unsigned int a4 __asm__("r10");
    register unsigned int b1 __asm__("r8");
    register unsigned int b2 __asm__("r9");
    register unsigned int b3 __asm__("ip");
    register unsigned int b4 __asm__("lr");
    do {
    GET_BLOCK_4(p1);
    XOR_BLOCK_4(p2);
    XOR_BLOCK_4(p3);
    PUT_BLOCK_4(p1);
    } while (--lines);
    }
    static void
    xor_arm4regs_4(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4)
    {
    let mut lines: c_uint = bytes / sizeof(unsigned long) / 2;
    register unsigned int a1 __asm__("r8");
    register unsigned int a2 __asm__("r9");
    register unsigned int b1 __asm__("ip");
    register unsigned int b2 __asm__("lr");
    do {
    GET_BLOCK_2(p1);
    XOR_BLOCK_2(p2);
    XOR_BLOCK_2(p3);
    XOR_BLOCK_2(p4);
    PUT_BLOCK_2(p1);
    } while (--lines);
    }
    static void
    xor_arm4regs_5(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4,
    const unsigned long * __restrict p5)
    {
    let mut lines: c_uint = bytes / sizeof(unsigned long) / 2;
    register unsigned int a1 __asm__("r8");
    register unsigned int a2 __asm__("r9");
    register unsigned int b1 __asm__("ip");
    register unsigned int b2 __asm__("lr");
    do {
    GET_BLOCK_2(p1);
    XOR_BLOCK_2(p2);
    XOR_BLOCK_2(p3);
    XOR_BLOCK_2(p4);
    XOR_BLOCK_2(p5);
    PUT_BLOCK_2(p1);
    } while (--lines);
    }
    DO_XOR_BLOCKS(arm4regs, xor_arm4regs_2, xor_arm4regs_3, xor_arm4regs_4,
    xor_arm4regs_5);
    struct xor_block_template xor_block_arm4regs = {
    .name		= "arm4regs",
    .xor_gen	= xor_gen_arm4regs,
    };
