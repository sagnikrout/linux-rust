//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/fcmpu.c
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

    int
    fcmpu(u32 *ccr, int crfD, void *frA, void *frB)
    {
    FP_DECL_D(A);
    FP_DECL_D(B);
    FP_DECL_EX;
    int code[4] = { (1 << 3), (1 << 1), (1 << 2), (1 << 0) };
    long cmp;

    printk("%s: %p (%08x) %d %p %p\n", __func__, ccr, *ccr, crfD, frA, frB);

    FP_UNPACK_DP(A, frA);
    FP_UNPACK_DP(B, frB);

    printk("A: %ld %lu %lu %ld (%ld)\n", A_s, A_f1, A_f0, A_e, A_c);
    printk("B: %ld %lu %lu %ld (%ld)\n", B_s, B_f1, B_f0, B_e, B_c);

    FP_CMP_D(cmp, A, B, 2);
    cmp = code[(cmp + 1) & 3];
    __FPU_FPSCR &= ~(0x1f000);
    __FPU_FPSCR |= (cmp << 12);
// ccr &= ~(15 << ((7 - crfD) << 2));
// ccr |= (cmp << ((7 - crfD) << 2));

    printk("CR: %08x\n", *ccr);

    return 0;
    }
