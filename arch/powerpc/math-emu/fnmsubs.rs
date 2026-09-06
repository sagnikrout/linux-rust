//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/fnmsubs.c
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
    fnmsubs(void *frD, void *frA, void *frB, void *frC)
    {
    FP_DECL_D(R);
    FP_DECL_D(A);
    FP_DECL_D(B);
    FP_DECL_D(C);
    FP_DECL_D(T);
    FP_DECL_EX;

    printk("%s: %p %p %p %p\n", __func__, frD, frA, frB, frC);

    FP_UNPACK_DP(A, frA);
    FP_UNPACK_DP(B, frB);
    FP_UNPACK_DP(C, frC);

    printk("A: %ld %lu %lu %ld (%ld)\n", A_s, A_f1, A_f0, A_e, A_c);
    printk("B: %ld %lu %lu %ld (%ld)\n", B_s, B_f1, B_f0, B_e, B_c);
    printk("C: %ld %lu %lu %ld (%ld)\n", C_s, C_f1, C_f0, C_e, C_c);

    if ((A_c == FP_CLS_INF && C_c == FP_CLS_ZERO) ||
    (A_c == FP_CLS_ZERO && C_c == FP_CLS_INF))
    FP_SET_EXCEPTION(EFLAG_VXIMZ);
    FP_MUL_D(T, A, C);
    if (B_c != FP_CLS_NAN)
    B_s ^= 1;
    if (T_s != B_s && T_c == FP_CLS_INF && B_c == FP_CLS_INF)
    FP_SET_EXCEPTION(EFLAG_VXISI);
    FP_ADD_D(R, T, B);
    if (R_c != FP_CLS_NAN)
    R_s ^= 1;

    printk("D: %ld %lu %lu %ld (%ld)\n", R_s, R_f1, R_f0, R_e, R_c);

    __FP_PACK_DS(frD, R);
    return FP_CUR_EXCEPTIONS;
    }
