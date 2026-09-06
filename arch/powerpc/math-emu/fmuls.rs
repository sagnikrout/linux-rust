//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/fmuls.c
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
    fmuls(void *frD, void *frA, void *frB)
    {
    FP_DECL_D(A);
    FP_DECL_D(B);
    FP_DECL_D(R);
    FP_DECL_EX;

    printk("%s: %p %p %p\n", __func__, frD, frA, frB);

    FP_UNPACK_DP(A, frA);
    FP_UNPACK_DP(B, frB);

    printk("A: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n",
    A_s, A_f1, A_f0, A_e, A_c, A_f1, A_f0, A_e + 1023);
    printk("B: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n",
    B_s, B_f1, B_f0, B_e, B_c, B_f1, B_f0, B_e + 1023);

    if ((A_c == FP_CLS_INF && B_c == FP_CLS_ZERO) ||
    (A_c == FP_CLS_ZERO && B_c == FP_CLS_INF))
    FP_SET_EXCEPTION(EFLAG_VXIMZ);
    FP_MUL_D(R, A, B);

    printk("D: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n",
    R_s, R_f1, R_f0, R_e, R_c, R_f1, R_f0, R_e + 1023);

    __FP_PACK_DS(frD, R);
    return FP_CUR_EXCEPTIONS;
    }
