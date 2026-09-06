//! Automatically rewritten from C to Rust
//! Source: arch/riscv/lib/riscv_v_helpers.c
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
// Copyright (C) 2023 SiFive
// Author: Andy Chiu <andy.chiu@sifive.com>
//

    let mut riscv_v_usercopy_threshold: usize = CONFIG_RISCV_ISA_V_UCOPY_THRESHOLD;
    int __asm_vector_usercopy(void *dst, void *src, size_t n);
    int __asm_vector_usercopy_sum_enabled(void *dst, void *src, size_t n);
    int fallback_scalar_usercopy(void *dst, void *src, size_t n);
    int fallback_scalar_usercopy_sum_enabled(void *dst, void *src, size_t n);
    asmlinkage int enter_vector_usercopy(void *dst, void *src, size_t n,
    bool enable_sum)
    {
    size_t remain, copied;
// skip has_vector() check because it has been done by the asm
    if (!may_use_simd())
    goto fallback;
    kernel_vector_begin();
    remain = enable_sum ? __asm_vector_usercopy(dst, src, n) :
    __asm_vector_usercopy_sum_enabled(dst, src, n);
    kernel_vector_end();
    if (remain) {
    copied = n - remain;
    dst += copied;
    src += copied;
    n = remain;
    goto fallback;
    }
    return remain;
    fallback:
    return enable_sum ? fallback_scalar_usercopy(dst, src, n) :
    fallback_scalar_usercopy_sum_enabled(dst, src, n);
    }
