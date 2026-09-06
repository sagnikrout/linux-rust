//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/stfs.c
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
    stfs(void *frS, void *ea)
    {
    FP_DECL_D(A);
    FP_DECL_S(R);
    FP_DECL_EX;
    float f;

    printk("%s: S %p, ea %p\n", __func__, frS, ea);

    FP_UNPACK_DP(A, frS);

    printk("A: %ld %lu %lu %ld (%ld)\n", A_s, A_f1, A_f0, A_e, A_c);

    FP_CONV(S, D, 1, 2, R, A);

    printk("R: %ld %lu %ld (%ld)\n", R_s, R_f, R_e, R_c);

    _FP_PACK_CANONICAL(S, 1, R);
    if (!FP_CUR_EXCEPTIONS || !__FPU_TRAP_P(FP_CUR_EXCEPTIONS)) {
    _FP_PACK_RAW_1_P(S, &f, R);
    if (copy_to_user(ea, &f, sizeof(float)))
    return -EFAULT;
    }
    return FP_CUR_EXCEPTIONS;
    }
