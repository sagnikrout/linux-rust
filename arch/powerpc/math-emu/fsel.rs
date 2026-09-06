//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/fsel.c
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
    fsel(u32 *frD, void *frA, u32 *frB, u32 *frC)
    {
    FP_DECL_D(A);
    FP_DECL_EX;

    printk("%s: %p %p %p %p\n", __func__, frD, frA, frB, frC);

    FP_UNPACK_DP(A, frA);

    printk("A: %ld %lu %lu %ld (%ld)\n", A_s, A_f1, A_f0, A_e, A_c);
    printk("B: %08x %08x\n", frB[0], frB[1]);
    printk("C: %08x %08x\n", frC[0], frC[1]);

    if (A_c == FP_CLS_NAN || (A_c != FP_CLS_ZERO && A_s)) {
    frD[0] = frB[0];
    frD[1] = frB[1];
    } else {
    frD[0] = frC[0];
    frD[1] = frC[1];
    }

    printk("D: %08x.%08x\n", frD[0], frD[1]);

    return 0;
    }
