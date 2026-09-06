//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/math-emu/lfs.c
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
    lfs(void *frD, void *ea)
    {
    FP_DECL_D(R);
    FP_DECL_S(A);
    FP_DECL_EX;
    float f;

    printk("%s: D %p, ea %p\n", __func__, frD, ea);

    if (copy_from_user(&f, ea, sizeof(float)))
    return -EFAULT;
    FP_UNPACK_S(A, f);

    printk("A: %ld %lu %ld (%ld) [%08lx]\n", A_s, A_f, A_e, A_c,
// (unsigned long *)&f);

    FP_CONV(D, S, 2, 1, R, A);

    printk("R: %ld %lu %lu %ld (%ld)\n", R_s, R_f1, R_f0, R_e, R_c);

    if (R_c == FP_CLS_NAN) {
    R_e = _FP_EXPMAX_D;
    _FP_PACK_RAW_2_P(D, frD, R);
    } else {
    __FP_PACK_D(frD, R);
    }
    return 0;
    }
