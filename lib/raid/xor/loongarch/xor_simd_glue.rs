//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/loongarch/xor_simd_glue.c
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
// LoongArch SIMD XOR operations
//
// Copyright (C) 2023 WANG Xuerui <git@xen0n.name>
//

    DO_XOR_BLOCKS(flavor##_inner, __xor_##flavor##_2, __xor_##flavor##_3,		\
    __xor_##flavor##_4, __xor_##flavor##_5);			\
    \
    static void xor_gen_##flavor(void *dest, void **srcs, unsigned int src_cnt,	\
    unsigned int bytes)						\
    {										\
    kernel_fpu_begin();							\
    xor_gen_##flavor##_inner(dest, srcs, src_cnt, bytes);			\
    kernel_fpu_end();							\
    }										\
    \
    struct xor_block_template xor_block_##flavor = {				\
    .name		= __stringify(flavor),					\
    .xor_gen	= xor_gen_##flavor					\
    }

    MAKE_XOR_GLUES(lsx);

    MAKE_XOR_GLUES(lasx);
