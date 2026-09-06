//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/xor-8regs-prefetch.c
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

    static void
    xor_8regs_p_2(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2)
    {
    let mut lines: c_long = bytes / (sizeof (long)) / 8 - 1;
    prefetchw(p1);
    prefetch(p2);
    do {
    prefetchw(p1+8);
    prefetch(p2+8);
    once_more:
    p1[0] ^= p2[0];
    p1[1] ^= p2[1];
    p1[2] ^= p2[2];
    p1[3] ^= p2[3];
    p1[4] ^= p2[4];
    p1[5] ^= p2[5];
    p1[6] ^= p2[6];
    p1[7] ^= p2[7];
    p1 += 8;
    p2 += 8;
    } while (--lines > 0);
    if (lines == 0)
    goto once_more;
    }
    static void
    xor_8regs_p_3(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3)
    {
    let mut lines: c_long = bytes / (sizeof (long)) / 8 - 1;
    prefetchw(p1);
    prefetch(p2);
    prefetch(p3);
    do {
    prefetchw(p1+8);
    prefetch(p2+8);
    prefetch(p3+8);
    once_more:
    p1[0] ^= p2[0] ^ p3[0];
    p1[1] ^= p2[1] ^ p3[1];
    p1[2] ^= p2[2] ^ p3[2];
    p1[3] ^= p2[3] ^ p3[3];
    p1[4] ^= p2[4] ^ p3[4];
    p1[5] ^= p2[5] ^ p3[5];
    p1[6] ^= p2[6] ^ p3[6];
    p1[7] ^= p2[7] ^ p3[7];
    p1 += 8;
    p2 += 8;
    p3 += 8;
    } while (--lines > 0);
    if (lines == 0)
    goto once_more;
    }
    static void
    xor_8regs_p_4(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4)
    {
    let mut lines: c_long = bytes / (sizeof (long)) / 8 - 1;
    prefetchw(p1);
    prefetch(p2);
    prefetch(p3);
    prefetch(p4);
    do {
    prefetchw(p1+8);
    prefetch(p2+8);
    prefetch(p3+8);
    prefetch(p4+8);
    once_more:
    p1[0] ^= p2[0] ^ p3[0] ^ p4[0];
    p1[1] ^= p2[1] ^ p3[1] ^ p4[1];
    p1[2] ^= p2[2] ^ p3[2] ^ p4[2];
    p1[3] ^= p2[3] ^ p3[3] ^ p4[3];
    p1[4] ^= p2[4] ^ p3[4] ^ p4[4];
    p1[5] ^= p2[5] ^ p3[5] ^ p4[5];
    p1[6] ^= p2[6] ^ p3[6] ^ p4[6];
    p1[7] ^= p2[7] ^ p3[7] ^ p4[7];
    p1 += 8;
    p2 += 8;
    p3 += 8;
    p4 += 8;
    } while (--lines > 0);
    if (lines == 0)
    goto once_more;
    }
    static void
    xor_8regs_p_5(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4,
    const unsigned long * __restrict p5)
    {
    let mut lines: c_long = bytes / (sizeof (long)) / 8 - 1;
    prefetchw(p1);
    prefetch(p2);
    prefetch(p3);
    prefetch(p4);
    prefetch(p5);
    do {
    prefetchw(p1+8);
    prefetch(p2+8);
    prefetch(p3+8);
    prefetch(p4+8);
    prefetch(p5+8);
    once_more:
    p1[0] ^= p2[0] ^ p3[0] ^ p4[0] ^ p5[0];
    p1[1] ^= p2[1] ^ p3[1] ^ p4[1] ^ p5[1];
    p1[2] ^= p2[2] ^ p3[2] ^ p4[2] ^ p5[2];
    p1[3] ^= p2[3] ^ p3[3] ^ p4[3] ^ p5[3];
    p1[4] ^= p2[4] ^ p3[4] ^ p4[4] ^ p5[4];
    p1[5] ^= p2[5] ^ p3[5] ^ p4[5] ^ p5[5];
    p1[6] ^= p2[6] ^ p3[6] ^ p4[6] ^ p5[6];
    p1[7] ^= p2[7] ^ p3[7] ^ p4[7] ^ p5[7];
    p1 += 8;
    p2 += 8;
    p3 += 8;
    p4 += 8;
    p5 += 8;
    } while (--lines > 0);
    if (lines == 0)
    goto once_more;
    }
    DO_XOR_BLOCKS(8regs_p, xor_8regs_p_2, xor_8regs_p_3, xor_8regs_p_4,
    xor_8regs_p_5);
    struct xor_block_template xor_block_8regs_p = {
    .name		= "8regs_prefetch",
    .xor_gen	= xor_gen_8regs_p,
    };
