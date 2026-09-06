//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/fsqrt.c
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
    fsqrt(void *frD, void *frB)
    {
    FP_DECL_D(B);
    FP_DECL_D(R);
    FP_DECL_EX;

    printk("%s: %p %p %p %p\n", __func__, frD, frB);

    FP_UNPACK_DP(B, frB);

    printk("B: %ld %lu %lu %ld (%ld)\n", B_s, B_f1, B_f0, B_e, B_c);

    if (B_s && B_c != FP_CLS_ZERO)
    FP_SET_EXCEPTION(EFLAG_VXSQRT);
    if (B_c == FP_CLS_NAN)
    FP_SET_EXCEPTION(EFLAG_VXSNAN);
    FP_SQRT_D(R, B);

    printk("R: %ld %lu %lu %ld (%ld)\n", R_s, R_f1, R_f0, R_e, R_c);

    __FP_PACK_D(frD, R);
    return FP_CUR_EXCEPTIONS;
    }
